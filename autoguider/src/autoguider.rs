use std::sync::Mutex;
use std::fmt::Debug;
use crossbeam;
use signal::Signal;

//fn shoot(&self) -> Image;
//fn correct(&self, Pos);
//fn calculate_correction(&self, image: Image) -> Pos;

//pub trait Camera {
    //fn shoot() -> Image;
    //fn correct(Pos);
//}


pub fn run_autoguider<Image, Pos, ShootFn, CorrectFn, CalcFn>(
    mut shoot: ShootFn,
    mut calculate_correction: CalcFn,
    mut correct: CorrectFn)
where Image: Send + Debug, Pos: Send,
      ShootFn: FnMut() -> Option<Image>,
      CalcFn: FnMut(Image) -> Pos, CalcFn: Sync + Send,
      CorrectFn: FnMut(Pos), CorrectFn: Send
{
    let camera_mutex = Mutex::new(());
    let image_signal = Signal::new();

    crossbeam::scope(|scope| {
        scope.spawn(|| {
            trace!("[thread] start");
            'outer: loop {
                if let Some(None) = image_signal.get() {
                    break 'outer;
                }
                let image = if let Some(image) = image_signal.get_wait() {
                    image
                } else {
                    break 'outer;
                };
                trace!("[thread] got image {:?}", image);
                let correction = calculate_correction(image);
                trace!("[thread] locking camera");
                let _lock = camera_mutex.lock().unwrap();
                trace!("[thread] locked camera, correcting");
                correct(correction);
                trace!("[thread] moving on");
            }
            trace!("[thread] end");
        });
        loop {
            let image = if let Some(image) = {
                trace!("[main loop] locking camera");
                let _lock = camera_mutex.lock().unwrap();
                shoot()
            } {
                image
            } else {
                break
            };
            trace!("[main loop] got image: {:?}", image);
            image_signal.set_notify(Some(image));
        }
        trace!("[main loop] sending None to end thread");
        image_signal.set_notify(None);
        trace!("[main loop] end of scope");
    });
}

#[cfg(test)]
mod tests {
    use env_logger;
    use crossbeam;
    use std::thread;
    use std::time::Duration;
    use super::*;

    #[test]
    fn test() {
        env_logger::init().unwrap();
        let images = ["1", "2", "3", "4"];
        let events = crossbeam::sync::MsQueue::new();

        let mut image_iter = images.iter();
        run_autoguider(
            || {
                let i = image_iter.next();
                events.push(format!("shoot {}", i.unwrap_or(&"None")));
                thread::sleep(Duration::from_secs(1));
                events.push(format!("end shoot {}", i.unwrap_or(&"None")));
                i
            },
            |image| {
                //info!("calculating correction for {:?}", image);
                events.push(format!("calc {}", image));
                image
            },
            |pos| {
                //info!("correcting {:?}", pos);
                events.push(format!("slew {}", pos));
            },
        );

        let mut ev = vec![];
        loop {
            if let Some(v) = events.try_pop() {
                ev.push(v);
            } else {
                break;
            }
        }

        let expected = [
            "shoot 1",
            "end shoot 1",
            "shoot 2",
            "calc 1",
            "end shoot 2",
            "slew 1",

            "shoot 3",
            "end shoot 3",
            "shoot 4",
            "calc 3",
            "end shoot 4",
            "slew 3",

            "shoot None",
            "end shoot None",
        ];
        assert_eq!(ev[..], expected[..]);
    }
}
