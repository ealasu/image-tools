extern crate mount_service_api;
extern crate docopt;
extern crate rustc_serialize;
extern crate point;

use docopt::Docopt;
use mount_service_api::Client;

const USAGE: &'static str = "
Usage:
  slew --ra=<degrees> --dec=<degrees> --threshold=<degrees>
";

#[derive(RustcDecodable)]
struct Args {
    flag_ra: String,
    flag_dec: String,
    flag_threshold: f64,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    let client = Client::new("ubuntu:1234").unwrap();
    point::point(&client, &args.flag_ra, &args.flag_dec, args.flag_threshold);
    println!("done.");
}

