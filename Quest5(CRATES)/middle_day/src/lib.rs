use chrono::NaiveDate;
use chrono::Datelike;

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0){
        return None;
    }else {
        NaiveDate::from_yo_opt(year as i32, 183).map(|date| date.weekday())
    }
    
}

/*
    * Q & A :
    * Q0 : What is a leap year and why do we have them?
    -A0 : A leap year is a year that has an extra day added to it, making it 366 days long instead of the usual 365 days. This extra day is added to the month of February, which has 29 days instead of 28. Leap years occur every four years, with some exceptions for years that are divisible by 100 but not divisible by 400. For example, the year 2000 was a leap year, but the year 1900 was not. The purpose of leap years is to keep our calendar in alignment with the Earth's revolutions around the Sun, as it takes approximately 365.25 days for the Earth to complete one orbit.
    * Q1 : What is the `chrono` crate in Rust, and how does it help with date and time manipulation?
    - A1 : The `chrono` crate in Rust is a powerful library for handling date and time. It provides a comprehensive set of types and functions for working with dates, times, durations, and time zones. The crate allows you to perform various operations such as parsing and formatting dates, calculating differences between dates, and manipulating date and time values. It also supports different calendar systems and provides functionality for working with time zones. Overall, the `chrono` crate makes it easier to work with date and time in Rust by providing a rich set of features and a user-friendly API.
    * Q2 : What does the `NaiveDate::from_yo_opt` function do in the context of this code?
    - A2 : The `NaiveDate::from_yo_opt` function is used to create a `NaiveDate` object from a given year and day of the year (also known as "year ordinal"). In this code, it takes the year as an integer (converted from u32 to i32) and the day of the year (183) as arguments. The function returns an `Option<NaiveDate>`, which will be `Some(NaiveDate)` if the provided year and day of the year are valid, or `None` if they are not. In this context, it is used to determine the date corresponding to the middle day of the year (the 183rd day) for non-leap years.
    * Q3 : Does map here work only on Option or can it be used with other types as well?
    - A3 : The `map` function is a method that can be used with the `Option` type in Rust. It allows you to apply a function to the value contained within an `Option` if it is `Some`, and it will return `None` if the `Option` is `None`. However, the `map` function is not limited to just the `Option` type; it can also be used with other types that implement the `Functor` trait, such as `Result` and `Vec` etc. In the case of `Result`, the `map` function applies a function to the `Ok` value, while leaving the `Err` value unchanged. So, while `map` is commonly used with `Option`, it can also be used with other types that support it, making it a versatile tool for handling various types of data transformations in Rust.
*/