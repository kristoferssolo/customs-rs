use std::{cmp, fmt::Display};

use crate::{
    citizenship::{Citizenship, CitizenshipType},
    customs::Customs,
    officer::Officer,
    utils::Time,
};

#[derive(Debug)]
pub struct Output(Vec<OutputTime>);

#[derive(Debug)]
pub struct OutputTime {
    input: Time,
    output: Time,
}

impl Output {
    pub fn new(mut customs: Customs, all_citizens: Vec<Citizenship>) -> Self {
        customs.sort();
        dbg!(&customs);
        let citizens = Self::from((&customs.citizens, &all_citizens, CitizenshipType::Citizen));
        let noncitizens = Self::from((
            &customs.noncitizens,
            &all_citizens,
            CitizenshipType::NonCitizen,
        ));
        let mut output = Vec::new();
        output.extend(citizens.0);
        output.extend(noncitizens.0);
        Self(output)
    }
}

impl From<(&Vec<Officer>, &Vec<Citizenship>, CitizenshipType)> for Output {
    fn from(value: (&Vec<Officer>, &Vec<Citizenship>, CitizenshipType)) -> Self {
        let (customs, citizens, type_) = value;
        let mut outputs = Vec::new();
        for officer in customs {
            let mut prev = 0;
            for citizen in citizens {
                if citizen.type_ == officer.citizenship && citizen.type_ == type_ {
                    let output_ = officer.time + cmp::max(citizen.id, prev);
                    let output = OutputTime::new(citizen.id, output_);
                    outputs.push(output);
                    prev = output_;
                }
            }
        }
        Self(outputs)
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output_string = String::new();
        for time in &self.0 {
            let output_str = format!("{}\n", time);
            output_string.push_str(&output_str);
        }
        write!(f, "{}", output_string)
    }
}

impl Into<String> for Output {
    fn into(self) -> String {
        self.to_string()
    }
}

impl OutputTime {
    pub fn new(input: Time, output: Time) -> Self {
        Self { input, output }
    }
}

impl Display for OutputTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.input, self.output)
    }
}
