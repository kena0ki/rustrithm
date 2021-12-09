
#![allow(unused)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustrithm::bitarray;
//use std::time::Instant;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("make_dp_array", |b| b.iter(|| make_dp_array(black_box(1000), black_box(100_000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn make_dp_array(n:usize, size:usize) {
    //let start = Instant::now();
    let mut v = Vec::with_capacity(n);
    let ba = bitarray::BitArray::from_u8slice_with_size(&[1], size);
    v.push(ba);
    for i in 0..n {
        let ba = &(&v[i] << i+1) | &v[i];
        v.push(ba);
    }
    //let duration = start.elapsed();

    //println!("Time elapsed in _f_dp_bit() is: {:?}", duration);
}
