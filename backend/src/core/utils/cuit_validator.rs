pub fn is_valid_cuit(cuit: &str) -> bool {
    let digits: Vec<u32> = cuit.chars().filter_map(|c| c.to_digit(10)).collect();

    if digits.len() != 11 {
        return false;
    }

    let multipliers = [5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
    let mut sum = 0;

    for i in 0..10 {
        sum += digits[i] * multipliers[i];
    }

    let mod11 = sum % 11;
    let mut verifier = if mod11 == 0 { 0 } else { 11 - mod11 };

    if verifier == 10 {
        // En algunos casos excepcionales se cambia el prefijo y el dígito verificador es 9,
        // pero matemáticamente para la validación simple aceptamos 9 si el cálculo da 10.
        verifier = 9;
    }

    verifier == digits[10]
}

pub fn normalize_cuit(cuit: &str) -> String {
    cuit.chars().filter(|c| c.is_ascii_digit()).collect()
}
