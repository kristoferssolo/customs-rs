use std::str::FromStr;

use crate::{
    citizenship::{Citizenship, CitizenshipType},
    customs::Customs,
    customs_info::CustomsInfo,
    error::ParseError,
    officer::Officer,
    output::Output,
    utils::Data,
};

pub fn process(input: &str) -> String {
    match parse_input(input) {
        Ok(mut data) => {
            data.sort();
            data.into()
        }
        Err(_) => "nothing\n".into(),
    }
}

pub fn parse_input(input: &str) -> Result<Output, ParseError> {
    let mut lines = input.lines();
    let first = match CustomsInfo::from_str(lines.next().unwrap()) {
        Ok(info) => info,
        Err(_) => return Err(ParseError::NoFirstLine),
    };

    let mut customs = Customs::new();
    let mut citizens = Vec::new();
    customs.citizens.reserve_exact(first.citizen.count as usize);
    customs
        .noncitizens
        .reserve_exact(first.noncitizen.count as usize);

    for line in lines {
        if line.contains('X') {
            break;
        };
        if let Ok(officer) = Officer::from_str(line) {
            customs.push(officer);
        }
        if let Ok(citizen) = Citizenship::from_str(line) {
            citizens.push(citizen);
        }
    }

    if citizens.is_empty() {
        return Err(ParseError::NoCitizens);
    }

    for customs_id in 1..=first.citizen.count {
        if customs
            .citizens
            .iter()
            .any(|x| x.id == customs_id && x.citizenship == CitizenshipType::Citizen)
        {
            continue;
        }
        let new = Officer::new(customs_id, first.citizen.time, CitizenshipType::Citizen);
        customs.push(new);
    }

    for customs_id in 1..=first.noncitizen.count {
        if customs
            .noncitizens
            .iter()
            .any(|x| x.id == customs_id && x.citizenship == CitizenshipType::NonCitizen)
        {
            continue;
        }
        let new = Officer::new(
            customs_id,
            first.noncitizen.time,
            CitizenshipType::NonCitizen,
        );
        customs.push(new);
    }

    let data = Data::new(customs, citizens);
    Ok(Output::from(data))
}
