use std::fmt::Display;

use crate::{
    citizenship::CitizenshipType,
    utils::{CustomsCount, Time},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputTime {
    pub arrival_time: Time,
    pub departure_time: Time,
    pub citizenship: CitizenshipType,
    pub officer_id: CustomsCount,
}

impl OutputTime {
    pub fn new(
        input: Time,
        output: Time,
        citizenship: CitizenshipType,
        officer_id: CustomsCount,
    ) -> Self {
        Self {
            arrival_time: input,
            departure_time: output,
            citizenship,
            officer_id,
        }
    }
}

impl PartialOrd for OutputTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.departure_time.partial_cmp(&other.departure_time)
    }
}

impl Ord for OutputTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.departure_time.cmp(&other.departure_time)
    }
}

impl Display for OutputTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.arrival_time, self.departure_time)
    }
}
