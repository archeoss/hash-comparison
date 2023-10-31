use std::{
    hash::{BuildHasher, Hasher},
    time::Duration,
};

use criterion::{
    black_box, criterion_group, criterion_main, AxisScale, Criterion, PlotConfiguration, Throughput,
};
use wyhash::wyhash;

const KB: usize = 1024;
const SEED: u64 = 0x0123456789ABCDEF;
const PARAMS: [usize; 7] = [7, 8, 32, 256, KB, 4 * KB, 16 * KB];

lazy_static::lazy_static! {
    static ref DATA: Vec<u8> = (0..16 * KB).map(|b| b as _).collect::<Vec<_>>();
}

// fn bench_memory(c: &mut Criterion) {
//     let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Linear);
//     let mut group = c.benchmark_group("Memory");
//     group.plot_config(plot_config);
//     group.bench_function(
//         "memory",
//         ParameterizedBenchmark::new(
//             "sum",
//             move |b, &&size| {
//                 let s = unsafe {
//                     slice::from_raw_parts(DATA.as_ptr() as *mut u32, size / mem::size_of::<u32>())
//                 };
//
//                 b.iter(|| {
//                     black_box(s.iter().fold(0u64, |acc, &x| acc + x as u64));
//                 })
//             },
//             &PARAMS,
//         )
//         .throughput(|&&size| Throughput::Bytes(size as _)),
//     );
// }

// fn bench_hash32(c: &mut Criterion) {
//     let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Linear);
//     let mut group = c.benchmark_group("hash32");
//     group.plot_config(plot_config);
//     for input in &PARAMS {
//         let mut cityhasher = fasthash::city::Hasher32::default();
//         let mut ahasher = ahash::AHasher::default();
//         let mut ahasher_old = ahash_old::AHasher::default();
//         let mut wyhasher = wyhash::WyHash::default();
//         let mut fxhasher = fxhash::FxHasher32::default();
//         let mut highway_hasher = highway::HighwayHasher::default();
//
//         group.throughput(Throughput::Bytes(*input as u64));
//         group.bench_with_input("cityhash", input, move |b, &size| {
//             b.iter(|| {
//                 cityhasher.write(&DATA.as_slice()[..size]);
//                 cityhasher.finish()
//             });
//         });
//         group.bench_with_input("ahash v.0.8.6", input, move |b, &size| {
//             b.iter(|| {
//                 ahasher.write(&DATA.as_slice()[..size]);
//                 ahasher.finish()
//             });
//         });
//         group.bench_with_input("ahash v.0.7.6", input, move |b, &size| {
//             b.iter(|| {
//                 ahasher_old.write(&DATA.as_slice()[..size]);
//                 ahasher_old.finish()
//             });
//         });
//         group.bench_with_input("wyhash", input, move |b, &size| {
//             b.iter(|| {
//                 wyhasher.write(&DATA.as_slice()[..size]);
//                 wyhasher.finish()
//             });
//         });
//         group.bench_with_input("fxhash", input, move |b, &size| {
//             b.iter(|| {
//                 fxhasher.write(&DATA.as_slice()[..size]);
//                 fxhasher.finish()
//             });
//         });
//         group.bench_with_input("highway", input, move |b, &size| {
//             b.iter(|| {
//                 highway_hasher.write(&DATA.as_slice()[..size]);
//                 highway_hasher.finish()
//             });
//         });
//     }
//     group.finish();
// }
//
fn bench_hash64(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Linear);
    let mut group = c.benchmark_group("hash64");
    group.measurement_time(Duration::from_secs(3));
    group.plot_config(plot_config);
    for input in &PARAMS {
        // let mut cityhasher = fasthash::city::Hasher64::default();
        let mut ahasher = ahash::AHasher::default();
        let mut ahasher_old = ahash_old::AHasher::default();
        let mut wyhasher = wyhash::WyHash::default();
        let mut fxhasher = fxhash::FxHasher64::default();
        let mut highway_hasher = highway::HighwayHasher::default();
        // let mut meow_hasher = meowhash::meow::Hasher::default();
        let mut seahasher = seahash::SeaHasher::default();
        let mut xxhasher = xxhash_rust::xxh3::Xxh3Builder::default().build_hasher();
        let mut metrohasher = metrohash::MetroHash64::default();

        group.throughput(Throughput::Bytes(*input as u64));
        // group.bench_with_input("cityhash", input, move |b, &size| {
        //     b.iter(|| {
        //         cityhasher.write(&DATA.as_slice()[..size]);
        //         cityhasher.finish()
        //     });
        // });
        group.bench_with_input("fxhash", input, move |b, &size| {
            b.iter(|| {
                fxhasher.write(&DATA.as_slice()[..size]);
                fxhasher.finish()
            });
        });
        group.bench_with_input("ahash v.0.8.6", input, move |b, &size| {
            b.iter(|| {
                ahasher.write(&DATA.as_slice()[..size]);
                ahasher.finish()
            });
        });
        group.bench_with_input("ahash v.0.7.6", input, move |b, &size| {
            b.iter(|| {
                ahasher_old.write(&DATA.as_slice()[..size]);
                ahasher_old.finish()
            });
        });
        group.bench_with_input("wyhash", input, move |b, &size| {
            b.iter(|| {
                wyhasher.write(&DATA.as_slice()[..size]);
                wyhasher.finish()
            });
        });
        group.bench_with_input("seahash", input, move |b, &size| {
            b.iter(|| {
                seahasher.write(&DATA.as_slice()[..size]);
                seahasher.finish()
            });
        });
        group.bench_with_input("xxhash", input, move |b, &size| {
            b.iter(|| {
                xxhasher.write(&DATA.as_slice()[..size]);
                xxhasher.finish()
            });
        });
        group.bench_with_input("metrohash", input, move |b, &size| {
            b.iter(|| {
                metrohasher.write(&DATA.as_slice()[..size]);
                metrohasher.finish()
            });
        });
        group.bench_with_input("highway", input, move |b, &size| {
            b.iter(|| {
                highway_hasher.write(&DATA.as_slice()[..size]);
                highway_hasher.finish()
            });
        });
    }
    group.finish();
}

// fn bench_hash128(c: &mut Criterion) {
//     let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Linear);
//     let mut group = c.benchmark_group("hash128");
//     group.measurement_time(Duration::from_secs(3));
//     group.plot_config(plot_config);
//     for input in &PARAMS {
//         // let mut cityhasher_crc = fasthash::city::crc::Hasher128::default();
//         // let mut cityhasher = fasthash::city::Hasher128::default();
//         let mut ahasher = ahash::AHasher::default();
//         let mut ahasher_old = ahash_old::AHasher::default();
//         let mut wyhasher = wyhash::WyHash::default();
//         // let fxhasher = fxhash::FxHasher64::default();
//         let mut highway_hasher = highway::HighwayHasher::default();
//         // let mut meow_hasher = meowhash::meow::Hasher::default();
//         let mut seahasher = seahash::SeaHasher::default();
//         let mut xxhasher = xxhash_rust::xxh3::Xxh3Builder::default().build_hasher();
//         let mut metrohasher = metrohash::MetroHash128::default();
//
//         group.throughput(Throughput::Bytes(*input as u64));
//         // group.bench_with_input("cityhash", input, move |b, &size| {
//         //     b.iter(|| {
//         //         cityhasher.write(&DATA.as_slice()[..size]);
//         //         cityhasher.finish()
//         //     });
//         // });
//         // group.bench_with_input("fxhash (HW CRC)", input, move |b, &size| {
//         //     b.iter(|| {
//         //         cityhasher_crc.write(&DATA.as_slice()[..size]);
//         //         cityhasher_crc.finish()
//         //     });
//         // });
//         group.bench_with_input("ahash v.0.8.6", input, move |b, &size| {
//             b.iter(|| {
//                 ahasher.write(&DATA.as_slice()[..size]);
//                 ahasher.finish()
//             });
//         });
//         group.bench_with_input("ahash v.0.7.6", input, move |b, &size| {
//             b.iter(|| {
//                 ahasher_old.write(&DATA.as_slice()[..size]);
//                 ahasher_old.finish()
//             });
//         });
//         group.bench_with_input("wyhash", input, move |b, &size| {
//             b.iter(|| {
//                 wyhasher.write(&DATA.as_slice()[..size]);
//                 wyhasher.finish()
//             });
//         });
//         group.bench_with_input("seahash", input, move |b, &size| {
//             b.iter(|| {
//                 seahasher.write(&DATA.as_slice()[..size]);
//                 seahasher.finish()
//             });
//         });
//         group.bench_with_input("xxhash", input, move |b, &size| {
//             b.iter(|| {
//                 xxhasher.write(&DATA.as_slice()[..size]);
//                 xxhasher.finish()
//             });
//         });
//         group.bench_with_input("metrohash", input, move |b, &size| {
//             b.iter(|| {
//                 metrohasher.write(&DATA.as_slice()[..size]);
//                 metrohasher.finish()
//             });
//         });
//         // group.bench_with_input("meow", input, move |b, &size| {
//         //     b.iter(|| {
//         //         meow_hasher.write(&DATA.as_slice()[..size]);
//         //         meow_hasher.finish()
//         //     });
//         // });
//         group.bench_with_input("highway", input, move |b, &size| {
//             b.iter(|| {
//                 highway_hasher.write(&DATA.as_slice()[..size]);
//                 highway_hasher.finish()
//             });
//         });
//     }
//     group.finish();
// }

// criterion_group!(benches, bench_hash32, bench_hash64, bench_hash128);
// criterion_group!(benches, bench_hash128);
criterion_group!(benches, bench_hash64);
criterion_main!(benches);
