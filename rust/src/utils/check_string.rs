use regex::Regex;
use once_cell::sync::Lazy;

static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    println!("initial regex");
    Regex::new(r"(?i)^[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+(?:[A-Z]{2}|com|org|net|gov|mil|biz|info|mobi|name|aero|jobs|museum|id)\b$").unwrap()
});

pub fn is_email(email: &str) -> bool {
    EMAIL_REGEX.is_match(email)
}