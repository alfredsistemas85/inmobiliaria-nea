use std::collections::HashSet;
use std::sync::LazyLock; // or lazy_static if available, but I'll just use simple arrays for now

const DISPOSABLE_DOMAINS: &[&str] = &[
    "mailinator.com",
    "guerrillamail.com",
    "10minutemail.com",
    "temp-mail.org",
    "maildrop.cc",
    "yopmail.com",
];

const PUBLIC_DOMAINS: &[&str] = &[
    "gmail.com",
    "hotmail.com",
    "outlook.com",
    "yahoo.com",
    "live.com",
    "icloud.com",
];

pub fn is_disposable(email: &str) -> bool {
    if let Some(domain) = email.split('@').nth(1) {
        DISPOSABLE_DOMAINS.contains(&domain.to_lowercase().as_str())
    } else {
        false
    }
}

pub fn get_email_type(email: &str) -> String {
    if let Some(domain) = email.split('@').nth(1) {
        if PUBLIC_DOMAINS.contains(&domain.to_lowercase().as_str()) {
            return "PUBLIC".to_string();
        }
    }
    "CORPORATE".to_string()
}
