// src/model/loan.rs
#[derive(Debug)]
pub enum TypeLoan {
    Semanal,
    Quincenal,
    Mensual,
}

impl TypeLoan {
    pub fn from_str(loan_type: &str) -> Option<Self> {
        match loan_type.to_lowercase().as_str() {
            "semanal" => Some(TypeLoan::Semanal),
            "quincenal" => Some(TypeLoan::Quincenal),
            "mensual" => Some(TypeLoan::Mensual),
            _ => None,
        }
    }

    pub fn type_loan_match(&self) -> &str {
        match self {
            TypeLoan::Semanal => "Semanal",
            TypeLoan::Quincenal => "Quincenal",
            TypeLoan::Mensual => "Mensual",
        }
    }
}
