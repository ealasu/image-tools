use std::thread;
use std::sync::Mutex;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::fmt::Debug;
use crossbeam;
use signal::{Signal, IterableSignal};

//pub type Image = u64;
//pub type Pos = u64;

//fn shoot(&self) -> Image;
//fn correct(&self, Pos);
//fn calculate_correction(&self, image: Image) -> Pos;

//pub trait Camera {
    //fn shoot() -> Image;
    //fn correct(Pos);
//}


fn run_autoguider<Image, Pos, ShootFn, CorrectFn, CalcFn>(
    mut shoot: ShootFn,
    mut calculate_correction: CalcFn,
    mut correct: CorrectFn)
where Image: Send + Debug, Pos: Send,
      ShootFn: FnMut() -> Option<Image>,
      CalcFn: FnMut(Image) -> Pos, CalcFn: Sync + Send,
      CorrectFn: FnMut(Pos), CorrectFn: Send
{
    let image_signal = Signal::new();
    //let correction_signal = Signal::new();
    let camera_mutex = Mutex::new(());


    let (image_tx, image_rx) = channel();
    thread::spawn(move|| {
        tx.send(10).unwrap();
    });
    assert_eq!(rx.recv().unwrap(), 10);Run


    crossbeam::scope(|scope| {
        scope.spawn(|| {
            println!("start of thread");
            'outer loop {
                for i in image_rx.try_iter() {
                    let i = if let Some(i) = i {
                        i
                    } else {
                        break 'outer;
                    };
                    println!("skipping image {:?}", i);
                }
                let image = if let Some(image) = image_rx.recv().unwrap() {
                    image
                } else {
                    break 'outer;
                };
            for image in image_signal.iter() {
                println!("thread got image {:?}", image);
                let correction = calculate_correction(image);
                println!("thread waiting for correction");
                let lock = camera_mutex.lock().unwrap();
                correct(correction);
                //correction_signal.set_wait(correction);
                println!("thread moving on");
            }
            println!("end of thread");
        });
        loop {
            let image = if let Some(image) = {
                let lock = camera_mutex.lock().unwrap();
                shoot()
            } {
                image
            } else {
                break
            };
            println!("got image: {:?}", image);
            image_signal.set_notify(Some(image));
            //let correction = correction_signal.get_notify();
            //if let Some(correction) = correction {
                //correct(correction);
            //}
        }
        //correction_signal.get_notify();
        println!("notifying image_signal to end thread");
        image_signal.set_notify(None);
        println!("end of scope");
    });
    println!("after scope");
}

#[cfg(test)]
mod tests {
    use crossbeam;
    use super::*;

    #[test]
    fn test() {
        let images = ["1", "2", "3", "4", "5"];

        crossbeam::scope(|scope| {
            scope.spawn(|| {
                let mut image_iter = images.iter();
                run_autoguider(
                    || {
                        let i = image_iter.next();
                        println!("shooting {:?}", i);
                        thread::sleep(Duration::from_secs(1));
                        i
                    },
                    |image| {
                        println!("calculating correction for {:?}", image);
                        image
                    },
                    |pos| {
                        println!("correcting {:?}", pos);
                    },
                );
            });
        });

    }
}
