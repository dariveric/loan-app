mod model;
use model::loan::Loan;
use model::person::Person;
mod view;
use view::person::person_info_view;
use view::loan::load_info_view;



fn main() {
    
    // varibles de Person
    let id: u32 = 1;
    let name =  "Juan".to_string();
    let paternal_surname  = "Perez".to_string();
    let maternal_surname = "Maldonado".to_string();
    let age:u8 = 47;
    let gender  = model::person::Gender::Masculino;

    // creamos una instancia de persona
    let juan = Person::new(id, name, paternal_surname, maternal_surname, age, gender);
    person_info_view(&juan);

    //variables de loan
    
    let amount_borrowed = 1000.00;
    let interest_rate = 20.00;
    let start_date = "2/12/2024".to_string();
    
    let juan_load = Loan::new(model::type_load::TypeLoan::Semanal, amount_borrowed, interest_rate, start_date);
    load_info_view(&juan_load);
   

}
