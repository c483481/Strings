struct TestIsEmailStruct {
    str: String,
    should_be: bool,
}

use crate::utils::check_string::{is_email, simple_check_role};

#[test]
fn test_check_is_email() {
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

struct TestCheckRoleStruct {
    str: String,
    should_be: String,
}

#[test]
fn test_simple_check_role() {
    let tests: [TestCheckRoleStruct; 5] = [
        TestCheckRoleStruct{ str: String::from("U"), should_be: String::from("users") },
        TestCheckRoleStruct{ str: String::from("A"), should_be: String::from("admin") },
        TestCheckRoleStruct{ str: String::from("SA"), should_be: String::from("super admin") },
        TestCheckRoleStruct{ str: String::from("u"), should_be: String::from("unknown") },
        TestCheckRoleStruct{ str: String::from("MR"), should_be: String::from("unknown") },
    ];
    for test in tests {
        let result = simple_check_role(&test.str);
        assert_eq!(result, test.should_be)
    }
}
