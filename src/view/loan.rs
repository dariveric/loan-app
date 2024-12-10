// src/view/load.rs
use crate::model::loan::Loan;
pub fn load_info_view(load: &Loan) {
    println!("Tipo de préstamo: {:?}", load.get_type_loan());
    println!("Monto prestado: ${:.2}", load.get_amount_borrowed());
    println!("Tasa de interés: {:.2}%", load.get_interest_rate());
    println!("Fecha de inicio: {}", load.get_start_date());
    println!("Fechas de pago:");

    for (i, payment_date) in load.get_remaining_payments().iter().enumerate() {
        println!("{:02}. {}", i + 1, payment_date);
    }

    let total_amount = load.calculate_interest_and_loan();
    let total_payment = load.calculate_payments();
    println!("\nTotal a pagar (con interés): ${:.2}", total_amount);
    println!("Monto de cada pago: ${:.2}", total_payment);
}

