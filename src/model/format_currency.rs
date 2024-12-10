pub fn format_currency(amount: f64) -> String {
    let formatted_amount = format!("{:.2}", amount);
    let parts: Vec<&str> = formatted_amount.split('.').collect();
    let integer_part = parts[0]
        .chars()
        .rev()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(",")
        .chars()
        .rev()
        .collect::<String>();
    let decimal_part = parts[1];
    format!("${}.{}", integer_part, decimal_part)
}