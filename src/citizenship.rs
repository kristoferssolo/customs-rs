use std::str::FromStr;

use crate::{error::InputError, utils::Time};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CitizenshipType {
    Citizen,
    NonCitizen,
}

impl PartialOrd for CitizenshipType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CitizenshipType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (CitizenshipType::Citizen, CitizenshipType::NonCitizen) => std::cmp::Ordering::Less,
            (CitizenshipType::NonCitizen, CitizenshipType::Citizen) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        }
    }
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
    pub arrival_time: Time,
    pub type_: CitizenshipType,
}

impl Citizenship {
    pub fn new(arrival_time: Time, type_: CitizenshipType) -> Self {
        Self {
            arrival_time,
            type_,
        }
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
                Ok(Self {
                    arrival_time: id,
                    type_,
                })
            }
            _ => Err(InputError::InvalidFormat),
        }
    }
}
