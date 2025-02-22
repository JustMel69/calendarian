use months::Month;
use weeks::Week;

pub mod weeks;
pub mod months;

#[derive(Debug)]
pub struct Calendar {
    week_def: Week,
    months: Vec<Month>,
}

impl Calendar {
    pub fn new() -> Self {
        Self {
            week_def: Week::new(),
            months: vec![
                Month::new("Springino", 31),
                Month::new("Summerino", 25),
                Month::new("Autumnino", 40),
                Month::new("Winterino", 15),
            ],
        }
    }

    pub fn week_def(&self) -> &Week {
        &self.week_def
    }

    pub fn months(&self) -> &[Month] {
        &self.months
    }
}
