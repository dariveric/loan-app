mod model;
use model::format_currency::format_currency;

fn main() {
    
    let id: u32 = 1;
    let name: &str = "Juan";
    let paternal_surname: &str ="Perez";
    let maternal_surname: &str ="Maldonado";
    let age:u8 =47;

    let loan_type:&str = "Semanal";
    let amount_borrowed = 1000.00;
    let interest_on_amount_borrowed = 20.00;
    let calculate_interest_and_loan = amount_borrowed * interest_on_amount_borrowed / 100 as f64 + amount_borrowed;
    let calculate_payments = calculate_interest_and_loan / 5 as f64;
    let loan_start_date = "2/12/2024";
    let remaining_payments =  ("9/12/2024","16/12/2024","23/12/2024","30/12/2024","6/01/2024");

    // formateamos el calculo del interes mas prestamo
    let format_output_calculation = format_currency(calculate_interest_and_loan);
    let format_output_amount_borrowed = format_currency(amount_borrowed);
    let format_payment_output = format_currency(calculate_payments);

    // salidad datos prestamista
    println!("______Datos Prestamista______");
    println!("id: {}", id);
    println!("Nombre: {}", name);
    println!("Apellido Paterno: {}", paternal_surname);
    println!("Apellido Materno: {}", maternal_surname);
    println!("Edad: {}", age);
    println!("_____________________________\n");
    
    // datos del prestamo
    println!("____Información del prestamo____");
    println!("Tipo de prestamo {}", loan_type);
    println!("Cantidad prestada: {}", format_output_amount_borrowed);
    println!("Interes: {:.2}%", interest_on_amount_borrowed);
    println!("Pago con interes {}", format_output_calculation);
    println!("Los Pagos {} son {}", loan_type, format_payment_output);
    println!("Fecha inicio de pagos {}", loan_start_date);
    println!("Pagos restantes {:?}", remaining_payments);
    println!("________________________________\n");

    

}
