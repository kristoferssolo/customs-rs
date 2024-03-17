use std::str::FromStr;

use crate::{
    citizenship::CitizenshipType,
    error::InputError,
    utils::{CustomsCount, Time},
};

#[derive(Debug)]
pub struct DefaultOfficer {
    pub count: CustomsCount,
    pub time: Time,
    pub citizenship: CitizenshipType,
}

impl DefaultOfficer {
    fn new(count: CustomsCount, time: Time, citizenship: CitizenshipType) -> Self {
        Self {
            count,
            time,
            citizenship,
        }
    }
}

#[derive(Debug)]
pub struct CustomsInfo {
    pub citizen: DefaultOfficer,
    pub noncitizen: DefaultOfficer,
}

impl CustomsInfo {
    pub fn count(&self) -> u8 {
        self.citizen.count + self.noncitizen.count
    }
}

impl FromStr for CustomsInfo {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split_whitespace()
            .map(|x| x.parse::<Time>().map_err(|_| InputError::InvalidFormat))
            .collect::<Result<Vec<_>, _>>();

        let nums = nums?;

        let (citizen_customs, noncitizen_customs, citizen_time, noncitizen_time) = (
            nums[0] as CustomsCount,
            nums[1] as CustomsCount,
            nums[2],
            nums[3],
        );
        let citizen = DefaultOfficer::new(citizen_customs, citizen_time, CitizenshipType::Citizen);
        let noncitizen = DefaultOfficer::new(
            noncitizen_customs,
            noncitizen_time,
            CitizenshipType::NonCitizen,
        );

        Ok(Self {
            citizen,
            noncitizen,
        })
    }
}
