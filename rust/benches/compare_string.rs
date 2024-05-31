use criterion::Criterion;
use rust::utils::compare_string::{compare_string,safe_compare_string};


struct TestCompareStruct {
    str1: String,
    str2: String,
}

pub fn compare_string_benchmark(c: &mut Criterion) {
    let tests: [TestCompareStruct; 5] = [
        TestCompareStruct{ str1: String::from("same string"), str2: String::from("same string") },
        TestCompareStruct{ str1: String::from("r2131230ck"), str2: String::from("r1233129lk") },
        TestCompareStruct{ 
            str1: String::from("very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string"), 
            str2: String::from("very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string"),  
        },
        TestCompareStruct{ 
            str1: String::from("very_short_string_very_short_string_very_short_string_very_short_string_very_short_string_very_short_string_very_short_string_very_short_string_very_short"), 
            str2: String::from("very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_v"), 
        },
        TestCompareStruct{ str1: String::from("long string"), str2: String::from("short string") },
    ];

    for (index, test) in tests.iter().enumerate() {
        c.bench_function(
            &format!("test compare string id: {}", index), 
            |b| b.iter(|| compare_string(&test.str1, &test.str2))
        );
    }

    for (index, test) in tests.iter().enumerate() {
        c.bench_function(
            &format!("test safe compare string id: {}", index), 
            |b| b.iter(|| safe_compare_string(&test.str1, &test.str2))
        );
    }
}
