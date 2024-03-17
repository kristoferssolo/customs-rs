use std::str::FromStr;

use crate::{
    citizenship::CitizenshipType,
    error::InputError,
    utils::{CustomsCount, Time},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Officer {
    pub id: CustomsCount,
    pub processing_time: Time,
    pub departure_time: Time,
    pub citizenship: CitizenshipType,
}

impl Officer {
    pub fn new(id: CustomsCount, time: Time, citizenship: CitizenshipType) -> Self {
        Self {
            id,
            processing_time: time,
            departure_time: 0,
            citizenship,
        }
    }

    pub fn potential_departure_time(&self) -> Time {
        self.processing_time + self.departure_time
    }

    pub fn filter_departure_time(&self, num: Time) -> bool {
        self.departure_time == 0 || self.departure_time == num
    }
}

impl PartialOrd for Officer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl Ord for Officer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl FromStr for Officer {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();

        match (iter.next(), iter.next(), iter.next(), iter.next()) {
            (Some("T"), Some(citizenship_type), Some(customs_num), Some(time)) => {
                let citizenship = CitizenshipType::from_str(citizenship_type)?;
                let time = time
                    .parse::<Time>()
                    .map_err(|_| InputError::InvalidFormat)?;
                let id = customs_num
                    .parse::<CustomsCount>()
                    .map_err(|_| InputError::InvalidFormat)?;
                Ok(Self {
                    id,
                    citizenship,
                    departure_time: 0,
                    processing_time: time,
                })
            }
            _ => Err(InputError::InvalidFormat),
        }
    }
}
