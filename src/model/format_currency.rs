//src/model/format_currency.rs
pub fn format_currency(amount: f64) -> String {
    let formatted_amount = format!("{:.2}", amount); // Asegura siempre 2 decimales
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
    let decimal_part = parts.get(1).unwrap_or(&"00");
    format!("${}.{}", integer_part, decimal_part)
}
