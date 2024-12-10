use chrono::{NaiveDate, Duration};
use std::collections::VecDeque;

pub struct Loan {
    pub amount_borrowed: f64,
    pub interest_rate: f64,
    pub start_date: String,
    pub remaining_payments: VecDeque<String>,
}

impl Loan {
    pub fn new(amount_borrowed: f64, interest_rate: f64, start_date: String) -> Self {
        let remaining_payments = Loan::generate_payment_dates(&start_date, 5);
        Loan {
            amount_borrowed,
            interest_rate,
            start_date,
            remaining_payments,
        }
    }

    // Función para generar las fechas de los pagos sumando una semana
    fn generate_payment_dates(start_date: &str, num_payments: usize) -> VecDeque<String> {
        let mut payment_dates = VecDeque::new();
        let start_date = NaiveDate::parse_from_str(start_date, "%d/%m/%Y").unwrap();
        
        for i in 0..num_payments {
            let payment_date = start_date + Duration::weeks(i as i64);
            payment_dates.push_back(payment_date.format("%d/%m/%Y").to_string());
        }

        payment_dates
    }

    // Función para calcular el pago con interés
    pub fn calculate_interest_and_loan(&self) -> f64 {
        self.amount_borrowed * (1.0 + self.interest_rate / 100.0)
    }

    // Función para calcular los pagos
    pub fn calculate_payments(&self) -> f64 {
        self.calculate_interest_and_loan() / self.remaining_payments.len() as f64
    }

    // Función para calcular y procesar los pagos
    pub fn calculate_and_process_payments(&self) {
        let payment_amount = self.calculate_payments();
        let mut total_paid = 0.0;
        
        println!("Pago    Fecha       Monto Abonado     Acumulado");
        
        for (i, payment_date) in self.remaining_payments.iter().enumerate() {
            total_paid += payment_amount;
            println!("{:02}     {:<12}    ${:.2}             ${:.2}", 
                     i + 1, payment_date, payment_amount, total_paid);
        }
    }
}
