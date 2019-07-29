#[macro_use]
extern crate criterion;

use criterion::Criterion;

fn criterion_benchmark(_c: &mut Criterion) {
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);