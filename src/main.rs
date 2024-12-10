mod model;
mod view;

use model::loan::Loan;
use model::person::Person;
use model::type_load::TypeLoan;
use view::person::person_info_view;
use view::loan::load_info_view;

fn main() {
    // varibles de Person
    let id: u32 = 1;
    let name = "Juan".to_string();
    let paternal_surname = "Perez".to_string();
    let maternal_surname = "Maldonado".to_string();
    let age: u8 = 47;
    let gender = model::person::Gender::Masculino;

    // creamos una instancia de person
    let juan = Person::new(id, name, paternal_surname, maternal_surname, age, gender);
    person_info_view(&juan);

    //variables de loan
    let type_load = TypeLoan::Semanal;
    let amount_borrowed = 1000.0;
    let interest_rate = 20.0;
    let start_date = "2/12/2024".to_string();

    //creamos una instancia de loan
    let juan_load = Loan::new(type_load, amount_borrowed, interest_rate, start_date);
    load_info_view(&juan_load);
}
