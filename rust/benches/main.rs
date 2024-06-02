use criterion::{
    criterion_group,
    criterion_main
};
mod  convert_to_number;
mod compare_string;
mod check_string;

use convert_to_number::convert_number_benchmark;
use compare_string::compare_string_benchmark;
use check_string::check_is_email_benchmark;

criterion_group!(benches, convert_number_benchmark, compare_string_benchmark, check_is_email_benchmark);
criterion_main!(benches);