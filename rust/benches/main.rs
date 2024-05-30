use criterion::{
    criterion_group,
    criterion_main
};
mod  convert_to_number;

use convert_to_number::convert_number_benchmark;

criterion_group!(benches, convert_number_benchmark);
criterion_main!(benches);