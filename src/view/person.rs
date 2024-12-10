// src/view/person.rs
use crate::model::person::Person;
// view person
pub fn person_info_view(person: &Person) {
    println!("");
    println!("id: {}", person.get_id());
    println!("Nombre: {}", person.get_first_name());
    println!("Apellido Paterno: {}", person.get_last_name());
    println!("Apellido Materno: {}", person.get_middle_name());
    println!("Edad: {}", person.get_age());
    println!("Genero: {:?}", person.get_gender());
    println!("\n");
}
