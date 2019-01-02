use std::fmt;
use std::str::FromStr;

use crate::errors;

#[derive(Debug, Copy, Clone)]
pub struct Date(::chrono::NaiveDate);

impl From<chrono::NaiveDate> for Date {
    fn from(date: chrono::NaiveDate) -> Self {
        Date(date)
    }
}

impl FromStr for Date {
    type Err = errors::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        ::chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map(Date)
            .map_err(From::from)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d").to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_valid_date() {
        let d = Date::from_str("2018-05-03");
        assert!(d.is_ok());
        assert_eq!(d.unwrap().to_string(), "2018-05-03");
    }
}
