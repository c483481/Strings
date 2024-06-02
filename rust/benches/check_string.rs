use criterion::Criterion;
use rust::utils::check_string::is_email;

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
