// src/view/load.rs
use crate::model::{ format_currency::format_currency, loan::Loan };

pub fn load_info_view(load: &Loan) {
    println!("Tipo de préstamo: {:?}", load.get_type_loan());
    println!("Monto prestado: {}", format_currency(load.get_amount_borrowed()));
    println!("Tasa de interés: {:.2}%", load.get_interest_rate());
    println!(
        "Total a pagar (con interés): {}",
        format_currency(load.calculate_interest_and_loan())
    );
    println!("Monto de cada pago: ${:.2}", format_currency(load.calculate_payments()));
    println!("Fecha de inicio: {}", load.get_start_date());
    println!("Fechas de pago\n");

    load.calculate_and_process_payments();
    println!("");
}
