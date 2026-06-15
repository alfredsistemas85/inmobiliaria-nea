pub fn mask_email(email: &str) -> String {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return email.to_string();
    }
    let username = parts[0];
    let domain = parts[1];

    if username.len() <= 1 {
        return format!("*@{}", domain);
    }

    let first_char = username.chars().next().unwrap();
    format!("{}***@{}", first_char, domain)
}

pub fn mask_phone(phone: &str) -> String {
    if phone.len() < 7 {
        return phone.to_string(); // Too short to mask meaningfully
    }
    let prefix_len = 3;
    let suffix_len = 3;

    let prefix = &phone[0..prefix_len];
    let suffix = &phone[phone.len() - suffix_len..];

    format!("{}****{}", prefix, suffix)
}
