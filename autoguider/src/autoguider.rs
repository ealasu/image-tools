use std::sync::{Arc,Mutex};
use std::sync::mpsc::channel;
use std::fmt::Debug;
use std::time::{Instant, Duration};
use std::thread;
use crossbeam;

struct Timestamped<T> {
    pub time: Instant,
    pub value: T
}

struct Event {
    pub start: Instant,
    pub end: Instant,
}

pub fn run_autoguider<ImageId, ImageIds, Image, Pos, ShootFn, CorrectFn, CalcFn>(
    mut image_ids: ImageIds,
    shot_duration: Duration,
    initial_correction: Pos,
    mut shoot: ShootFn,
    mut calculate_correction: CalcFn,
    mut correct: CorrectFn)
where Image: Send + Debug,
      Pos: Send + Copy,
      ImageId: Send,
      ImageIds: Iterator<Item=ImageId> + Send,
      ShootFn: Fn(ImageId) -> Image,
      ShootFn: Send + Sync,
      CalcFn: FnMut(Image) -> Pos,
      CalcFn: Sync + Send,
      CorrectFn: FnMut(Pos),
      CorrectFn: Send
{
    let (images_tx, images_rx) = channel::<Timestamped<Image>>();
    let (slew_tx, slew_rx) = channel::<Event>();
    let correction = Arc::new(Mutex::new(initial_correction));

    crossbeam::scope(|scope| {
        let mut last_slew_end = Instant::now() - Duration::from_secs(1);
        {
            let correction = correction.clone();
            scope.spawn(move || {
                trace!("[thread] start");
                loop {
                    let image = if let Some(i) = images_rx.iter().find(|e| {
                        e.time > last_slew_end
                    }).map(|e| e.value) {
                        i
                    } else {
                        break;
                    };
                    trace!("[thread] got image {:?}", image);

                    let calc_start = Instant::now();
                    let c = calculate_correction(image);
                    let calc_duration = calc_start.elapsed();
                    info!("calculation time: {:?}", calc_duration);

                    let correction_time = Instant::now();
                    {
                        *correction.lock().unwrap() = c;
                    }
                    last_slew_end = if let Some(e) = slew_rx.iter().find(|e| e.start > correction_time) {
                        e.end
                    } else {
                        break;
                    };
                }
                trace!("[thread] end");
            });
        }
        for image_id in image_ids {
            let shot_start = Instant::now();
            let shot = scope.spawn(|| {
                shoot(image_id)
            });
            thread::sleep(shot_duration);

            let slew_start = Instant::now();
            let c = *(correction.lock().unwrap());
            correct(c);
            slew_tx.send(Event {
                start: slew_start,
                end: Instant::now()
            });

            let image = shot.join();
            images_tx.send(Timestamped {
                time: shot_start,
                value: image
            }).unwrap();
        }
        drop(images_tx);
        drop(slew_tx);
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
        let images = ["1", "2", "3", "4", "5", "6"];
        let events = crossbeam::sync::MsQueue::new();

        let mut image_iter = images.iter();
        run_autoguider(
            images.iter(),
            Duration::from_millis(30),
            "nothing",
            |id| {
                events.push(format!("shoot {}", id));
                thread::sleep(Duration::from_millis(50));
                events.push(format!("end shoot {}", id));
                id
            },
            |image| {
                info!("calculating correction for {:?}", image);
                thread::sleep(Duration::from_millis(10));
                image
            },
            |pos| {
                info!("correcting {:?}", pos);
                events.push(format!("slew {}", pos));
                thread::sleep(Duration::from_millis(20));
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

        //let expected = [
            //"shoot 1",
            //"end shoot 1",
            //"shoot 2",
            //"calc 1",
            //"end shoot 2",
            //"slew 1",

            //"shoot 3",
            //"end shoot 3",
            //"shoot 4",
            //"calc 3",
            //"end shoot 4",
            //"slew 3",

            //"shoot None",
            //"end shoot None",
        //];
        let expected = [
            "shoot 1",
                "slew nothing",
            "end shoot 1",
            "shoot 2",
                "slew 1",
            "end shoot 2",
            "shoot 3",
                "slew 1",
            "end shoot 3",
            "shoot 4",
                "slew 3",
            "end shoot 4",
            "shoot 5",
                "slew 3",
            "end shoot 5",
            "shoot 6",
                "slew 5",
            "end shoot 6",
        ];
        assert_eq!(ev[..], expected[..]);
    }
}
