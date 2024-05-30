use crate::utils::convert_to_number::convert_to_number;

struct TestConvertStruct {
    input: String,
    should_be: i32,
}


#[test]
fn check_convert_to_number() {
    let tests: [TestConvertStruct; 5] = [
        TestConvertStruct{ input: String::from("2"), should_be: 2 },
        TestConvertStruct{ input: String::from("231"), should_be: 231 },
        TestConvertStruct{ input: String::from("3129837"), should_be: 3129837 },
        TestConvertStruct{ input: String::from("213n23kk3"), should_be: 0 },
        TestConvertStruct{ input: String::from("123e213e"), should_be: 0 },
    ];
    for test in tests {
        let result = convert_to_number(&test.input);
        assert_eq!(result, test.should_be)
    }
}
