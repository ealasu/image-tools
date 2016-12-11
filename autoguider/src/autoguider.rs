use std::thread;
use std::time::Duration;
use crossbeam;
use signal::Signal;

pub type Image = u64;
pub type Pos = u64;

pub trait Autoguider: Sync {
    fn shoot(&self) -> Image;
    fn correct(&self, Pos);
    fn calculate_correction(&self, image: Image) -> Pos;

    fn run(&self) {
        let image_signal = Signal::new();
        let correction_signal = Signal::new();

        crossbeam::scope(|scope| {
            scope.spawn(|| {
                let image = image_signal.get_wait();
                let correction = self.calculate_correction(image);
                correction_signal.set_wait(correction);
            });
        });

        loop {
            let correction = correction_signal.get_notify();
            if let Some(correction) = correction {
                self.correct(correction);
            }
            let image = self.shoot();
            //thread::sleep(Duration::from_secs(1));
            image_signal.set_notify(image);
        }
    }
}
