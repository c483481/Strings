use crate::utils::replace_string::replace_string;

struct TestReplaceString {
    input: String,
    should_be: String,
}

#[test]
fn check_replace_string() {
    let tests: [TestReplaceString; 2] = [
        TestReplaceString{ input: String::from("The Quick Brown Fox"), should_be: String::from("The-Quick-Brown-Fox") },
        TestReplaceString{ 
            input: String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur."), 
            should_be: String::from("Lorem-ipsum-dolor-sit-amet,-consectetur-adipiscing-elit.-Sed-do-eiusmod-tempor-incididunt-ut-labore-et-dolore-magna-aliqua.-Ut-enim-ad-minim-veniam,-quis-nostrud-exercitation-ullamco-laboris-nisi-ut-aliquip-ex-ea-commodo-consequat.-Duis-aute-irure-dolor-in-reprehenderit-in-voluptate-velit-esse-cillum-dolore-eu-fugiat-nulla-pariatur."), 
        },
    ];
    for test in tests {
        let result = replace_string(&test.input, " ", "-");
        assert_eq!(result, test.should_be)
    }
}


