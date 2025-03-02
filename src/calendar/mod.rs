use months::Month;
use weeks::Week;

pub mod weeks;
pub mod months;

pub type GlobalDayInt = i64;
pub type YearInt = i32;
pub type MonthUint = u32;
pub type DayUint = u32;

#[derive(Debug)]
pub struct Calendar {
    week_def: Week,
    months: Vec<Month>,

    /// Date this calendar starts in the global scope
    start_date: GlobalDayInt,
    
    /// Calendar date of start_date
    start_offset: DayUint,
}

impl Calendar {
    pub fn new() -> Self {
        Self {
            week_def: Week::new(),
            months: vec![
                Month::new("Springino", 31),
                Month::new("Summerino", 25),
                Month::new("Autumnino", 90),
                Month::new("Winterino", 15),
            ],
            start_date: 13,
            start_offset: 69,
        }
    }

    pub fn week_def(&self) -> &Week {
        &self.week_def
    }

    pub fn months(&self) -> &[Month] {
        &self.months
    }

    pub fn starting_weekday_of_month(&self, year: YearInt, month: MonthUint) -> u32 {
        assert!(year == 0 && month == 0); // for simplicity for now

        return (self.start_offset as usize % self.week_def.days().len()) as u32;
    }
}
