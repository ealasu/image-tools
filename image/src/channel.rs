use std::default::Default;
use std::f32;
use std::fmt;
use std::iter::repeat;
use std::io::prelude::*;


pub struct Channel<P> {
    pub width: usize,
    pub height: usize,
    data: Vec<P>,
}

impl<P> fmt::Debug for Channel<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[image {}x{}]", self.width, self.height)
    }
}

impl<P: Clone + Copy + Default> Channel<P> {
    pub fn from_data(width: usize, height: usize, data: Vec<P>) -> Self {
        Channel {
            width: width,
            height: height,
            data: data,
        }
    }

    pub fn new(width: usize, height: usize) -> Self {
        let mut data: Vec<P> = Vec::with_capacity(width * height);
        let zero: P = Default::default();
        data.extend(repeat(zero).take(width * height));
        Self::from_data(width, height, data)
    }

    #[inline(always)]
    pub fn at(&self, x: usize, y: usize) -> P {
        //assert!(x < self.width);
        //assert!(y < self.height);
        self.data[x + y * self.width]
    }

    #[inline(always)]
    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut P {
        //assert!(x < self.width);
        //assert!(y < self.height);
        &mut self.data[x + y * self.width]
    }

    #[inline(always)]
    pub fn pixels(&self) -> &Vec<P> {
        &self.data
    }

    #[inline(always)]
    pub fn pixels_mut(&mut self) -> &mut Vec<P> {
        &mut self.data
    }
}

impl Channel<f32> {
    pub fn average(&self) -> f32 {
        self.pixels().iter().fold(0.0, |acc, v| acc + v) / self.pixels().len() as f32
    }

    pub fn min(&self) -> f32 {
        self.pixels().iter().fold(f32::MAX, |acc, &v| acc.min(v))
    }

    pub fn max(&self) -> f32 {
        self.pixels().iter().fold(f32::MIN, |acc, &v| acc.max(v))
    }
}
