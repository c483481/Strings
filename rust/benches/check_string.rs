use criterion::Criterion;
use rust::utils::check_string::{is_email, simple_check_role};

pub fn check_is_email_benchmark(c: &mut Criterion) {
    let tests = vec![
            "admin@gmail.com", 
            "student@student.univ.ac.id",
            "not a email",
            "asdfg@asdf.asd"
        ];

    for (index, test) in tests.iter().enumerate() {
        c.bench_function(
            &format!("test check is email id: {}", index), 
            |b| b.iter(|| is_email(&test))
        );
    }
}

pub fn check_simple_role_benchmark(c: &mut Criterion) {
    let tests = vec![
            "U", 
            "A",
            "SA",
            "u",
            "MR"
        ];

    for (index, test) in tests.iter().enumerate() {
        c.bench_function(
            &format!("test check simple role id: {}", index), 
            |b| b.iter(|| simple_check_role(&test))
        );
    }
}

