#[macro_use] extern crate log;
extern crate env_logger;
extern crate rand;
extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate router;
extern crate logger;
extern crate persistent;
extern crate serde_json;
extern crate docopt;
extern crate rustc_serialize;

use std::path::{Path, PathBuf};
use std::fs;
use std::env;
use std::process::Command;
use iron::prelude::*;
use iron::status;
use iron::modifiers::Header;
use iron::headers::{ContentType, CacheControl, CacheDirective};
use iron::mime::*;
use iron::typemap::Key;
use staticfile::Static;
use mount::Mount;
use router::Router;
use logger::Logger;
use persistent::Read;
use docopt::Docopt;
use rand::Rng;

const USAGE: &'static str = "
Sieve.

Usage:
    sieve <dir>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_dir: String,
}

struct Config {
    root: PathBuf,
    session_id: String,
}

impl Config {
    pub fn wrap_id(&self, id: &str) -> String {
        format!("{}-{}", self.session_id, id)
    }

    pub fn unwrap_id(&self, wrapped_id: &str) -> String {
        let mut i = wrapped_id.splitn(2, "-");
        assert_eq!(i.next().unwrap(), &self.session_id);
        i.next().unwrap().to_string()
    }
}

struct ConfigKey;
impl Key for ConfigKey { type Value = Config; }


fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    env_logger::init().unwrap();

    let mut router = Router::new();
    router.get("/list", handle_list, "list");
    router.get("/image/:id", handle_image, "image");
    router.post("/move/:id/:dest", handle_move, "move");

    let mut mount = Mount::new();
    mount.mount("/api", router);
    mount.mount("/", Static::new(Path::new("./ui/target")));

    let (logger_before, logger_after) = Logger::new(None);
    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    let config = Config {
        root: PathBuf::from(args.arg_dir),
        session_id: rand::thread_rng()
            .gen_ascii_chars()
            .take(32)
            .collect::<String>(),
    };
    chain.link(Read::<ConfigKey>::both(config));
  
    let port = 3000;
    println!("listening on port {}", port);
    Iron::new(chain).http(("0.0.0.0", port)).unwrap();
}

fn handle_list(req: &mut Request) -> IronResult<Response> {
    let config = req.get::<Read<ConfigKey>>().unwrap();
    let list: Vec<_> = fs::read_dir(&config.root).unwrap().map(|f| {
        f.unwrap().file_name().into_string().unwrap()
    }).filter(|name| {
        name.ends_with(".CR2")
    }).map(|name| {
        config.wrap_id(&name)
    }).collect();
    Ok(Response::with((
                status::Ok,
                serde_json::to_string(&list).unwrap(),
                ContentType::json().0)))
}

fn handle_image(req: &mut Request) -> IronResult<Response> {
    let config = req.get::<Read<ConfigKey>>().unwrap();
    let ref wrapped_id = req.extensions.get::<Router>().unwrap().find("id").unwrap();
    let id = config.unwrap_id(wrapped_id);
    let path = config.root.join(Path::new(&id));
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
                Header(CacheControl(vec![CacheDirective::Public, CacheDirective::MaxAge(99999999)]))
                )))
}

fn handle_move(req: &mut Request) -> IronResult<Response> {
    let config = req.get::<Read<ConfigKey>>().unwrap();
    let ref wrapped_id = req.extensions.get::<Router>().unwrap().find("id").unwrap();
    let id = config.unwrap_id(wrapped_id);
    let src_path = config.root.join(&id);
    let ref dest = req.extensions.get::<Router>().unwrap().find("dest").unwrap();
    let dest_dir = config.root.join(dest);
    fs::create_dir_all(&dest_dir).unwrap();
    let dest_path = dest_dir.join(id);
    info!("move {:?} to {:?}", src_path.to_str(), dest_path.to_str());
    fs::rename(src_path, dest_path).unwrap();
    Ok(Response::with((status::NoContent)))
}
