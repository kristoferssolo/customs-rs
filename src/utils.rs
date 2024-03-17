use crate::{citizenship::Citizenship, customs::Customs};

pub type Time = u32;
pub type CustomsCount = u8;

#[derive(Debug)]
pub struct Data {
    pub customs: Customs,
    pub citizens: Vec<Citizenship>,
}

impl Data {
    pub fn new(customs: Customs, citizens: Vec<Citizenship>) -> Self {
        Self { customs, citizens }
    }
}
