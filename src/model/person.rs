#[derive(Debug)]
pub enum Gender {
    MASCULINO, FEMENINO,
}

#[derive(Debug)]
pub struct Person {
    id: u32,
    name: &str,
    paternal_surname: &str,
    maternal_surname: &str,
    age: u8,
    gender: Gender
}

impl Person {
    fn new(id: u32, name: String, paternal_surname: String, maternal_surname: String, age: u8, gender: Gender) -> Self {
        Person {
            id, name. paternal_surname, maternal_surname, age, gender
        }    
    }
}