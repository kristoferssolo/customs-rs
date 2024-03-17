use std::str::FromStr;

use crate::{error::InputError, utils::Time};

#[derive(Debug, PartialEq, Eq)]
pub enum CitizenshipType {
    Citizen,
    NonCitizen,
}

impl FromStr for CitizenshipType {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "P" => Ok(Self::Citizen),
            "N" => Ok(Self::NonCitizen),
            _ => Err(InputError::InvalidType("citizenship type".into())),
        }
    }
}

#[derive(Debug)]
pub struct Citizenship {
    pub id: Time,
    pub type_: CitizenshipType,
}

impl Citizenship {
    pub fn new(id: Time, type_: CitizenshipType) -> Self {
        Self { id, type_ }
    }
}

impl FromStr for Citizenship {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();

        match (iter.next(), iter.next()) {
            (Some(citizenship), Some(id)) => {
                let id = id.parse::<Time>().map_err(|_| InputError::InvalidFormat)?;
                let type_ = CitizenshipType::from_str(citizenship)?;
                Ok(Self { id, type_ })
            }
            _ => Err(InputError::InvalidFormat),
        }
    }
}
