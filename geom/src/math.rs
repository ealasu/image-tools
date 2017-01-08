//use types::*;
use unit::Unit;

//pub fn distance(p1: Star, p2: Star) -> Unit {
    //let a = p2.x - p1.x;
    //let b = p2.y - p1.y;
    //((a * a) + (b * b)).sqrt()
//}

pub fn are_close(a: Unit, b: Unit, epsilon: Unit) -> bool {
    (a - b).abs() < epsilon
}

//pub fn max_by<I,F,C>(iter: I, start: C, compare: F) -> I::Item
//where I: Iterator, C: PartialOrd, F: Fn(&I::Item) -> C {
    //iter.fold((start, None), |(max_c, max_v), v| {
        //let c = compare(&v);
        //if c > max_c {
            //(c, Some(v))
        //} else {
            //(max_c, max_v)
        //}
    //}).1.unwrap()
//}
