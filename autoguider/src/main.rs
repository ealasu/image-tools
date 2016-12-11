#![feature(loop_break_value)]
#![feature(receiver_try_iter)]

#[macro_use] extern crate log;
extern crate env_logger;
extern crate crossbeam;

mod signal;
mod autoguider;

fn main() {
    env_logger::init().unwrap();
    println!("Hello, world!");
}
