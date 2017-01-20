extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate router;
extern crate serde_json;

use std::path::Path;
use iron::prelude::*;
use iron::status;
use iron::modifiers::Header;
use iron::headers::{ContentType, CacheControl, CacheDirective};
use iron::mime::*;
use staticfile::Static;
use mount::Mount;
use router::Router;


fn main() {
    let mut router = Router::new();
    router.get("/list", handle_list, "list");
    router.get("/image/:id", handle_image, "image");
    router.post("/move/:id/:dest", handle_move, "move");

    let mut mount = Mount::new();
    mount.mount("/api", router);
    mount.mount("/", Static::new(Path::new("static/")));
  
    let port = 3000;
    println!("listening on port {}", port);
    Iron::new(mount).http(("127.0.0.1", port)).unwrap();
}

fn handle_list(req: &mut Request) -> IronResult<Response> {
    let list = vec!["todo"];
    Ok(Response::with((
                status::Ok,
                serde_json::to_string(&list).unwrap(),
                ContentType::json().0)))
}

fn handle_image(req: &mut Request) -> IronResult<Response> {
    let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap();
    let data = vec![0u8];

    Ok(Response::with((
                status::Ok,
                data,
                Header(ContentType::jpeg()),
                Header(CacheControl(vec![CacheDirective::NoStore, CacheDirective::MustRevalidate]))
                )))
}

fn handle_move(req: &mut Request) -> IronResult<Response> {
    let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap();
    let ref dest = req.extensions.get::<Router>().unwrap().find("dest").unwrap();

    Ok(Response::with((status::NoContent)))
}
