// src/view/person.rs
use model::person::Person;

pub fn person_info_view(person: &Person) {
    println!("id: {}", person.get_id());
    println!("Nombre: {}", person.get_first_name());
    println!("Apellido Paterno: {}", person.get_last_name());
    println!("Apellido Materno: {}", Person.get_middle_name());
    println!("Edad: {}", person.get_age());
    println!("Genero: {:?}", person.get_gender());
}
    