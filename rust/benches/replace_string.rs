use criterion::Criterion;
use rust::utils::replace_string::replace_string;

pub fn replace_string_benchmark(c: &mut Criterion) {
    let tests = vec![
            "The Quick Brown Fox", 
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.",
        ];

    for (index, test) in tests.iter().enumerate() {
        c.bench_function(
            &format!("replace string id: {}", index), 
            |b| b.iter(|| replace_string(&test, " ", "-"))
        );
    }
}
