use crate::utils::compare_string::{compare_string, safe_compare_string};

struct TestCompareStruct {
    str1: String,
    str2: String,
    should_be: bool,
}

#[test]
fn check_compare_string() {
    let tests: [TestCompareStruct; 5] = [
        TestCompareStruct{ str1: String::from("same string"), str2: String::from("same string"), should_be: true },
        TestCompareStruct{ str1: String::from("r2131230ck"), str2: String::from("r1233129lk"), should_be: false },
        TestCompareStruct{ 
            str1: String::from("very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string"), 
            str2: String::from("very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string"), 
            should_be: true 
        },
        TestCompareStruct{ 
            str1: String::from("very_short_string_very_short_string_very_short_string_very_short_string_very_short_string_very_short_string_very_short_string_very_short_string_very_short"), 
            str2: String::from("very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_v"), 
            should_be: false 
        },
        TestCompareStruct{ str1: String::from("long string"), str2: String::from("short string"), should_be: false },
    ];
    for test in tests {
        let result = compare_string(&test.str1, &test.str2);
        assert_eq!(result, test.should_be)
    }
}

#[test]
fn check_safe_compare_string() {
    let tests: [TestCompareStruct; 5] = [
        TestCompareStruct{ str1: String::from("same string"), str2: String::from("same string"), should_be: true },
        TestCompareStruct{ str1: String::from("r2131230ck"), str2: String::from("r1233129lk"), should_be: false },
        TestCompareStruct{ 
            str1: String::from("very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string"), 
            str2: String::from("very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string"), 
            should_be: true 
        },
        TestCompareStruct{ 
            str1: String::from("very_short_string_very_short_string_very_short_string_very_short_string_very_short_string_very_short_string_very_short_string_very_short_string_very_short"), 
            str2: String::from("very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_very_long_string_v"), 
            should_be: false 
        },
        TestCompareStruct{ str1: String::from("long string"), str2: String::from("short string"), should_be: false },
    ];
    for test in tests {
        let result = safe_compare_string(&test.str1, &test.str2);
        assert_eq!(result, test.should_be)
    }
}
