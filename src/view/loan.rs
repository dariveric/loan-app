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