use crate::{citizenship::CitizenshipType, officer::Officer};

#[derive(Debug, Default)]
pub struct Customs {
    pub citizens: Vec<Officer>,
    pub noncitizens: Vec<Officer>,
}

impl Customs {
    pub fn new() -> Self {
        Self {
            citizens: Vec::new(),
            noncitizens: Vec::new(),
        }
    }
    pub fn push(&mut self, officer: Officer) {
        match officer.citizenship {
            CitizenshipType::Citizen => self.citizens.push(officer),
            CitizenshipType::NonCitizen => self.noncitizens.push(officer),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.citizens.is_empty() && self.noncitizens.is_empty()
    }

    pub fn sort(&mut self) {
        self.citizens.sort();
        self.noncitizens.sort();
    }
}
