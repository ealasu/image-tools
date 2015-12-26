use channel::*;


pub trait Image<P> {
    fn new(width: usize, height: usize) -> Self;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn channels<'a>(&'a self) -> Box<Iterator<Item=&'a Channel<P>> + 'a>;
    fn channels_mut<'a>(&'a mut self) -> Box<Iterator<Item=&'a mut Channel<P>> + 'a>;
}

//pub trait RescaleTo<From,To>: Image<From> {
    //fn rescale_to<I: Image<To>>(&self) -> I;
//}

//impl RescaleTo<u16, f32> for Image<u16> {
    //fn rescale_to<I: Image<u32>>(&self) -> I;
//}
