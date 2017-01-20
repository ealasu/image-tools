#[macro_use] extern crate log;
extern crate env_logger;
extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate router;
extern crate logger;
extern crate serde_json;

use std::path::{Path, PathBuf};
use std::fs;
use std::env;
use std::process::Command;
use iron::prelude::*;
use iron::status;
use iron::modifiers::Header;
use iron::headers::{ContentType, CacheControl, CacheDirective};
use iron::mime::*;
use staticfile::Static;
use mount::Mount;
use router::Router;
use logger::Logger;


fn main() {
    env_logger::init().unwrap();

    let mut router = Router::new();
    router.get("/list", handle_list, "list");
    router.get("/image/:id", handle_image, "image");
    router.post("/move/:id/:dest", handle_move, "move");

    let mut mount = Mount::new();
    mount.mount("/api", router);
    mount.mount("/", Static::new(Path::new("../ui/target")));

    let (logger_before, logger_after) = Logger::new(None);
    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_after(logger_after);
  
    let port = 3000;
    println!("listening on port {}", port);
    Iron::new(chain).http(("0.0.0.0", port)).unwrap();
}

fn handle_list(req: &mut Request) -> IronResult<Response> {
    let list: Vec<_> = fs::read_dir(root()).unwrap().filter_map(|f| {
        let name = f.unwrap().file_name().into_string().unwrap();
        if name.ends_with(".CR2") {
            Some(name)
        } else {
            None
        }
    }).collect();
    Ok(Response::with((
                status::Ok,
                serde_json::to_string(&list).unwrap(),
                ContentType::json().0)))
}

fn handle_image(req: &mut Request) -> IronResult<Response> {
    let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap();
    let path = root().join(Path::new(id));
    let output = Command::new("exiftool")
        .arg("-PreviewImage")
        .arg("-b")
        .arg(path)
        .output()
        .expect("failed to execute exiftool");
    Ok(Response::with((
                status::Ok,
                output.stdout,
                Header(ContentType::jpeg()),
                Header(CacheControl(vec![CacheDirective::NoStore, CacheDirective::MustRevalidate]))
                )))
}

fn handle_move(req: &mut Request) -> IronResult<Response> {
    let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap();
    let src_path = root().join(id);
    let ref dest = req.extensions.get::<Router>().unwrap().find("dest").unwrap();
    let dest_dir = root().join(dest);
    fs::create_dir_all(&dest_dir).unwrap();
    let dest_path = dest_dir.join(id);
    info!("move {:?} to {:?}", src_path.to_str(), dest_path.to_str());
    fs::rename(src_path, dest_path).unwrap();
    Ok(Response::with((status::NoContent)))
}

fn root() -> PathBuf {
    PathBuf::from(env::args().skip(1).next().unwrap())
}
