struct TestIsEmailStruct {
    str: String,
    should_be: bool,
}

use crate::utils::check_string::is_email;

#[test]
fn check_check_is_email() {
    let tests: [TestIsEmailStruct; 4] = [
        TestIsEmailStruct{ str: String::from("admin@gmail.com"), should_be: true },
        TestIsEmailStruct{ str: String::from("student@student.univ.ac.id"), should_be: true },
        TestIsEmailStruct{ str: String::from("not a email"), should_be: false },
        TestIsEmailStruct{ str: String::from("asdfg@asdf.asd"), should_be: false },
    ];

    for test in tests {
        let result = is_email(&test.str);
        assert_eq!(result, test.should_be)
    }
}
