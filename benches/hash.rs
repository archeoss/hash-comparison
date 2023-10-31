#![allow(clippy::all, clippy::pedantic)]
use std::{
    hash::{BuildHasher, Hasher},
    time::{Duration, Instant},
};

use criterion::{
    black_box, criterion_group, criterion_main, AxisScale, Criterion, PlotConfiguration, Throughput,
};
use metrohash::MetroHash64;

const KB: usize = 1024;
const SEED: u64 = 0x0123456789ABCDEF;
// const PARAMS: [usize; 7] = [7, 8, 32, 256, KB, 4 * KB, 16 * KB];
const PARAMS: [usize; 7] = [4, 8, 16, 256, KB, 4 * KB, 16 * KB];
// const PARAMS: [usize; 3] = [4, 8, 16];

lazy_static::lazy_static! {
    static ref DATA: Vec<u8> = (0..16 * KB).map(|b| b as _).collect::<Vec<_>>();
}

fn bench_ahash(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Linear);
    let mut group = c.benchmark_group("ahash");
    group.measurement_time(Duration::from_secs(15));
    group.plot_config(plot_config);
    for input in &PARAMS {
        let hasher = ahash::RandomState::with_seed(SEED as usize).build_hasher();
        group.throughput(Throughput::Bytes(*input as u64));
        group.bench_with_input("ahash", input, move |b, &size| {
            b.iter(|| {
                black_box({
                    let mut hasher = hasher.clone();
                    hasher.write(&DATA.as_slice()[..size]);
                    hasher.finish()
                });
            });
        });
    }
    group.finish();
}

// fn bench_hash64(c: &mut Criterion) {
//     let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Linear);
//     let mut group = c.benchmark_group("hash64 (4B, 8B, 16B)");
//     group.measurement_time(Duration::from_secs(5));
//     group.plot_config(plot_config);
//     for input in &PARAMS {
//         let cityhasher = cityhasher::CityHasher::with_seed(SEED);
//         let mut fxhasher = fxhash::FxHasher64::default();
//         fxhasher.write_u64(SEED);
//         let ahasher = ahash::RandomState::with_seed(SEED as usize).build_hasher();
//         let ahasher_old = ahash_old::AHasher::new_with_keys(SEED as u128, (SEED + 1) as u128);
//         let wyhasher = wyhash::WyHash::with_seed(SEED);
//         let seahasher = seahash::SeaHasher::with_seeds(SEED, SEED + 1, SEED + 2, SEED + 3);
//         let xxhasher = xxhash_rust::xxh3::Xxh3Builder::default()
//             .with_seed(SEED)
//             .build_hasher();
//         let metrohasher = metrohash::MetroHash::with_seed(SEED);
//         let highwayhasher =
//             highway::HighwayHasher::new(highway::Key([SEED, SEED + 1, SEED + 2, SEED + 3]));
//         group.throughput(Throughput::Bytes(*input as u64));
//         group.bench_with_input("cityhash", input, move |b, &size| {
//             b.iter(|| {
//                 black_box({
//                     let mut hasher = cityhasher.clone();
//                     hasher.write(&DATA.as_slice()[..size]);
//                     hasher.finish();
//                 });
//             });
//         });
//         group.bench_with_input("fxhash", input, move |b, &size| {
//             b.iter(|| {
//                 black_box({
//                     let mut hasher = fxhasher.clone();
//                     hasher.write(&DATA.as_slice()[..size]);
//                     hasher.finish();
//                 });
//             });
//         });
//         group.bench_with_input("ahash v.0.8.6", input, move |b, &size| {
//             b.iter(|| {
//                 black_box({
//                     let mut hasher = ahasher.clone();
//                     hasher.write(&DATA.as_slice()[..size]);
//                     hasher.finish();
//                 });
//             });
//         });
//         group.bench_with_input("ahash v.0.7.6", input, move |b, &size| {
//             b.iter(|| {
//                 black_box({
//                     let mut hasher = ahasher_old.clone();
//                     hasher.write(&DATA.as_slice()[..size]);
//                     hasher.finish();
//                 });
//             });
//         });
//         group.bench_with_input("wyhash", input, move |b, &size| {
//             b.iter(|| {
//                 black_box({
//                     let mut hasher = wyhasher.clone();
//                     hasher.write(&DATA.as_slice()[..size]);
//                     hasher.finish();
//                 });
//             });
//         });
//         group.bench_with_input("seahash", input, move |b, &size| {
//             b.iter(|| {
//                 black_box({
//                     let mut hasher = seahasher.clone();
//                     hasher.write(&DATA.as_slice()[..size]);
//                     hasher.finish();
//                 });
//             });
//         });
//         group.bench_with_input("xxhash", input, move |b, &size| {
//             b.iter(|| {
//                 black_box({
//                     let mut hasher = xxhasher.clone();
//                     hasher.write(&DATA.as_slice()[..size]);
//                     hasher.finish();
//                 });
//             });
//         });
//         group.bench_with_input("metrohash", input, move |b, &size| {
//             b.iter(|| {
//                 black_box({
//                     let mut hasher =
//                         unsafe { std::mem::transmute_copy::<_, MetroHash64>(&metrohasher) };
//                     hasher.write(&DATA.as_slice()[..size]);
//                     hasher.finish();
//                 });
//             });
//         });
//         group.bench_with_input("highway", input, move |b, &size| {
//             b.iter(|| {
//                 black_box({
//                     let mut hasher = highwayhasher.clone();
//                     hasher.write(&DATA.as_slice()[..size]);
//                     hasher.finish();
//                 });
//             });
//         });
//     }
//     group.finish();
// }

// criterion_group!(benches, bench_ahash, bench_hash64);
// criterion_group!(benches, bench_hash64);
criterion_group!(benches, bench_ahash);
criterion_main!(benches);
