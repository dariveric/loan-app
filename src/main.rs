mod model;
mod view;
use model::loan::Loan;
use model::person::Person;


fn main() {
    
    // varibles de Person
    let id: u32 = 1;
    let name =  "Juan".to_string();
    let paternal_surname  = "Perez".to_string();
    let maternal_surname = "Maldonado".to_string();
    let age:u8 = 47;
    let gender  = model::person::Gender::MALE;

    // creamos una instancia de persona
    let juan = Person::new(id, name, paternal_surname, maternal_surname, age, gender);
    juan.person_info_view();

    //variables de loan
    let amount_borrowed = 1000.00;
    let interest_rate = 20.00;
    let start_date = "2/12/2024".to_string();

    let juan_load = Loan::new(amount_borrowed, interest_rate, start_date);
    juan_load.calculate_and_process_payments();    
    


}
