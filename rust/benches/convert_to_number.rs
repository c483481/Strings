use criterion::Criterion;
use rust::utils::convert_to_number::convert_to_number;

struct TestConvertStruct {
    input: String,
}

pub fn convert_number_benchmark(c: &mut Criterion) {
    let tests: [TestConvertStruct; 5] = [
        TestConvertStruct{ input: String::from("2") },
        TestConvertStruct{ input: String::from("231") },
        TestConvertStruct{ input: String::from("3129837") },
        TestConvertStruct{ input: String::from("213n23kk3") },
        TestConvertStruct{ input: String::from("123e213e") },
    ];

    for (index, test) in tests.iter().enumerate() {
        c.bench_function(
            &format!("test convert to number id: {}", index), 
            |b| b.iter(|| convert_to_number(&test.input))
        );
    }
}