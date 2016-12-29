#![feature(conservative_impl_trait)]
#![feature(test)]

#[cfg(test)] extern crate test;
#[cfg(test)] extern crate rand;
extern crate statistical;
extern crate quickersort;

pub mod image;
mod remove_background;
