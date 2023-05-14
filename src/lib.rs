#[doc = include_str!("../README.md")]
mod components;
mod utils;

pub use crate::components::*;
use chrono::{DateTime, Duration, Local, Month, NaiveDate, NaiveTime, Utc};

// pub(crate) const TOKEN_ABBR_DAY: char = 'D';
// pub(crate) const TOKEN_ABBR_HOUR: char = 'H';
// pub(crate) const TOKEN_ABBR_MIN: char = 'M';
// pub(crate) const TOKEN_ABBR_SEC: char = 'S';
pub(crate) const TOKEN_ABBR_WEEK: char = 'W';
pub(crate) const TOKEN_DATETIME_SEPARATOR: char = '-';

pub type ParseStringFn<T> = dyn Fn(&str) -> Option<T>;
pub type ParseComponentFn<T> = dyn Fn(&str, &mut usize) -> Option<T>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateTimeValue {
	Month(Month),
	Date(NaiveDate),
	YearlessDate(YearlessDate),
	Time(NaiveTime),
	TimeZoneOffset(TimeZoneOffset),
	LocalDateTime(DateTime<Local>),
	GlobalDateTime(DateTime<Utc>),
	Week(YearWeek),
	Duration(Duration),
}
