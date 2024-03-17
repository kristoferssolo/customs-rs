use std::str::FromStr;

use crate::{
    citizenship::{Citizenship, CitizenshipType},
    customs::Customs,
    customs_info::CustomsInfo,
    error::ParseError,
    officer::Officer,
    output::Output,
};

pub fn process(input: &str) -> String {
    match parse_input(input) {
        Ok(data) => data.into(),
        Err(e) => {
            println!("Error: {e}");
            "nothing\n".into()
        }
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

    while let Some(line) = lines.next() {
        if line.contains("X") {
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

    let mut citizen_customs_new = Vec::new();
    for customs_id in 1..=first.citizen.count {
        if customs
            .citizens
            .iter()
            .any(|x| x.id == customs_id && x.citizenship == CitizenshipType::Citizen)
        {
            continue;
        }
        let new = Officer::new(customs_id, first.citizen.time, CitizenshipType::Citizen);
        citizen_customs_new.push(new);
    }

    let mut noncitizen_customs_new = Vec::new();
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
        noncitizen_customs_new.push(new);
    }

    customs.citizens.extend(citizen_customs_new);
    customs.noncitizens.extend(noncitizen_customs_new);

    Ok(Output::new(customs, citizens))
}
