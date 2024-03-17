use std::fmt::Display;

use crate::{
    citizenship::{Citizenship, CitizenshipType},
    officer::Officer,
    output_time::OutputTime,
    utils::{Data, Time},
};

#[derive(Debug)]
pub struct Output(Vec<OutputTime>);

impl Output {
    pub fn sort(&mut self) {
        // first by departure time, then by CitizenshipType (citizens first), then by Officer id
        self.0.sort_by(|a, b| {
            let departure_time_cmp = a.departure_time.cmp(&b.departure_time);
            if departure_time_cmp.is_ne() {
                return departure_time_cmp;
            }
            let citizenship_cmp = a.citizenship.cmp(&b.citizenship);
            if citizenship_cmp.is_ne() {
                return citizenship_cmp;
            }
            a.officer_id.cmp(&b.officer_id)
        });
    }
}

impl From<Data> for Output {
    fn from(value: Data) -> Self {
        let mut outputs = Vec::new();
        let mut citizens = value.customs.citizens;
        let mut noncitizens = value.customs.noncitizens;

        for citizen in &value.citizens {
            let time = match citizen.type_ {
                CitizenshipType::Citizen => process_citizenship_entry(&citizen, &mut citizens),
                CitizenshipType::NonCitizen => {
                    process_citizenship_entry(&citizen, &mut noncitizens)
                }
            };
            outputs.push(time);
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

fn calculate_departure_time(officer: &mut Officer, arrival_time: Time) -> Time {
    if arrival_time > officer.departure_time {
        arrival_time + officer.processing_time
    } else {
        officer.potential_departure_time()
    }
}

fn process_citizenship_entry(citizen: &Citizenship, customs: &mut Vec<Officer>) -> OutputTime {
    let arrival_time = citizen.arrival_time;

    let officer = customs
        .iter_mut()
        .filter(|officer| officer.departure_time == 0 || arrival_time >= officer.departure_time)
        .min_by_key(|officer| officer.id);

    let officer = match officer {
        Some(x) => x,
        None => customs
            .iter_mut()
            .min_by_key(|officer| (officer.departure_time, officer.id))
            .unwrap(),
    };

    let departure_time = calculate_departure_time(officer, arrival_time);

    officer.departure_time = departure_time;
    OutputTime::new(
        arrival_time,
        departure_time,
        officer.citizenship,
        officer.id,
    )
}
