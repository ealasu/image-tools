#[macro_use]
extern crate bencher;

use bencher::Bencher;

fn bench_to_gray(b: &mut Bencher) {
    let image = Image::<Rgb<f32>>::random(5000, 4000);
    b.iter(|| {
        image.to_gray()
    });
}

fn bench_crop(b: &mut Bencher) {
    let image = Image::<Rgb<f32>>::random(5000, 4000);
    b.iter(|| {
        image.center_crop(900, 900)
    });
}

benchmark_group!(benches, bench_to_gray, bench_crop);
benchmark_main!(benches);
