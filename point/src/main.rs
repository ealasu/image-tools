extern crate structopt;
#[macro_use] extern crate structopt_derive;
extern crate mount_service_api;
extern crate env_logger;
extern crate point;

use structopt::StructOpt;
use mount_service_api::Client;

#[derive(StructOpt, Debug)]
#[structopt(name = "point", about = "")]
struct Args {
    #[structopt(long = "ra", help = "degrees")]
    flag_ra: String,
    #[structopt(long = "dec", help = "degrees")]
    flag_dec: String,
    #[structopt(long = "threshold", help = "degrees")]
    flag_threshold: f64,
}

fn main() {
    let args = Args::from_args();
    env_logger::init().unwrap();
    let client = Client::new("ubuntu:1234").unwrap();
    point::point(&client, &args.flag_ra, &args.flag_dec, args.flag_threshold);
    println!("done.");
}

