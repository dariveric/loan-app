// función para convertir los numero a una cadena con dos decimales
fn format_currency(amount: f64) -> String {
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

// inicia programa principal
fn main() {
    
    let id: u32 = 1;
    let name: &str = "Juan";
    let paternal_surname: &str ="Perez";
    let maternal_surname: &str ="Maldonado";
    let age:u8 =47;

    let loan = 1000.00;
    let loan_interest = 20.00;
    let calculate_loan_interest = loan * loan_interest / 100.0 + loan;
    let loan_type:&str = "Semanal";
    let loan_start_date = "2/12/2024";

    

}
