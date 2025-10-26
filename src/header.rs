use chrono::NaiveDateTime;
use chrono::NaiveDate;

use crate::messagetype::MessageType;
use crate::processingtype::ProcessingType;
use crate::versionidentifier::VersionIdentifier;
use crate::versionidentifier::T0399;
use crate::codedelement::CodedElement;
use crate::entityidentifier::EntityIdentifier;
use crate::id::IDs;

pub enum T0155 {
    AL,
    ER,
    NE,
    SU
}

impl T0155 {
    pub fn from_str(s: &str) -> Option<Self> {
        match s{
            "AL" => Some(T0155::AL),
            "ER" => Some(T0155::ER),
            "NE" => Some(T0155::NE),
            "SU" => Some(T0155::SU),
            _ => None
        }
    }
}



pub enum T0356 {
    V2_3,
    Iso_2022_1994
}

impl T0356 {
    pub fn from_str(s: &str) -> Option<Self> {
        match s{
            "2.3" => Some(T0356::V2_3),
            "ISO2022-1994" => Some(T0356::Iso_2022_1994),
            _ => None
        }
    }
}






pub struct Header {
    pub msh1: String, // Filed Seperator
    pub msh2: String, // Econdic Characters
    pub msh3: Option<String>, // Sending Application -> Opt
    pub msh4: Option<String>, // Sending Facility -> Opt
    pub msh5: Option<String>, // Receiving Application -> Opt
    pub msh6: Option<String>, // Receiving Facility -> Opt
    pub msh7: chrono::NaiveDateTime, // Date/Time OF Message
    pub msh8: Option<String>, // Security -> Opt
    pub msh9: MessageType,
    pub msh10: String,
    pub msh11: ProcessingType,
    pub msh12: VersionIdentifier,
    pub msh13: Option<i32>,
    pub msh14: Option<String>,
    pub msh15: Option<T0155>,
    pub msh16: Option<T0155>,
    pub msh17: Option<T0399>,
    pub msh18: Option<IDs>,  //Null perch nnon ho voglia di farlo
    pub msh19: Option<CodedElement>,
    pub msh20: Option<T0356>,
    pub msh21: Option<EntityIdentifier>,
}


impl Header {
    fn validate(&self) -> Vec<String> {
        todo!("");

        /*

         let mut errors = Vec::new();

        if self.campo1.is_empty() {
            errors.push("campo1 non può essere vuoto".to_string());
        }

        if let Some(ref val) = self.campo2 {
            if val.len() < 3 {
                errors.push("campo2 deve avere almeno 3 caratteri".to_string());
            }
        }

        if self.campo3 == 0 {
            errors.push("campo3 deve essere diverso da zero".to_string());
        }

        errors

        */
    }
}
