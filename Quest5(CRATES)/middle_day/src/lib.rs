use chrono::NaiveDate;
use chrono::Datelike;

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0){
        return None;
    }else {
        NaiveDate::from_yo_opt(year as i32, 183).map(|date| date.weekday())
    }
    
}