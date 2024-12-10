// Enum Gender
#[derive(Debug)]
pub enum Gender {
    MALE, FEMALE,
}

// Struct Person
#[derive(Debug)]
pub struct Person {
    id: u32,
    first_name: String,
    last_name: String,
    middle_name: String,
    age: u8,
    gender: Gender,
}

// Implementation of Person
impl Person {
    pub fn new(id: u32, first_name: String, last_name: String, middle_name: String, age: u8, gender: Gender) -> Self {
        Person {
            id,
            first_name,
            last_name,
            middle_name,
            age,
            gender,
        }
    }

    // Getter Methods
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_first_name(&self) -> &str {
        &self.first_name
    }

    pub fn get_last_name(&self) -> &str {
        &self.last_name
    }

    pub fn get_middle_name(&self) -> &str {
        &self.middle_name
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }

    pub fn get_gender(&self) -> &Gender {
        &self.gender
    }
}
