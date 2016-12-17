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
    use super::*;

    #[test]
    fn test() {
        env_logger::init().unwrap();
        let images = ["1", "2", "3", "4", "5"];

        crossbeam::scope(|scope| {
            scope.spawn(|| {
                let mut image_iter = images.iter();
                run_autoguider(
                    || {
                        let i = image_iter.next();
                        info!("shooting {:?}", i);
                        thread::sleep(Duration::from_secs(1));
                        i
                    },
                    |image| {
                        info!("calculating correction for {:?}", image);
                        image
                    },
                    |pos| {
                        info!("correcting {:?}", pos);
                    },
                );
            });
        });

    }
}
