#[doc = include_str!("../README.md")]
mod components;
mod utils;

pub use crate::components::*;
use crate::utils::*;

use chrono::{DateTime, Duration, Local, Month, NaiveDate, NaiveDateTime, NaiveTime, Utc};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MDisambig {
	Month,
	Minute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DurationUnit {
	Year,
	Month,
	Week,
	Day,
	Hour,
	Minute,
	Second,
}

pub fn parse_global_datetime(s: &str) -> Option<DateTime<Utc>> {
	let mut position = 0usize;
	let (year, month, day) = parse_date_component(s, &mut position)?;

	let last_char = s.chars().nth(position);
	if position > s.len() || !matches!(last_char, Some('T') | Some(' ')) {
		return None;
	} else {
		position += 1;
	}

	let time = parse_time_component(s, &mut position)?;
	if position < s.len() {
		return None;
	}

	let timezone_offset = parse_timezone_offset_component(s, &mut position)?;
	if position < s.len() {
		return None;
	}

	let date = NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32)?;
	let timezone_offset_as_duration = Duration::minutes(
		timezone_offset.minutes as i64 + timezone_offset.hours as i64 * 60,
	);
	let naive_datetime = NaiveDateTime::new(
		date,
		time.overflowing_sub_signed(timezone_offset_as_duration).0,
	);

	Some(DateTime::<Utc>::from_utc(naive_datetime, Utc))
}

pub(crate) fn is_some_and<T, P>(option: Option<T>, predicate: P) -> bool
where
	P: FnOnce(&T) -> bool,
{
	match option {
		Some(value) => predicate(&value),
		None => false,
	}
}

#[allow(unused_assignments)]
pub fn parse_duration(input: &str) -> Option<Duration> {
	let mut position = 0usize;
	let mut months = 0;
	let mut seconds = 0;
	let mut components_count = 0;
	let mut m_disambig = MDisambig::Minute;

	// step 5: skip whitespace
	let _ = skip_ascii_whitespace(input, &mut position);

	if position > input.len() {
		return None;
	}

	if input.chars().nth(position) != Some('P') {
		position += 1;
		m_disambig = MDisambig::Month;
		let _ = skip_ascii_whitespace(input, &mut position);
	}

	loop {
		let mut unit: Option<DurationUnit> = None;
		let mut next_char: Option<char>;

		if position > input.len() {
			break;
		}

		if input.chars().nth(position) == Some('T') {
			m_disambig = MDisambig::Minute;
			let _ = skip_ascii_whitespace(input, &mut position);
		}

		next_char = input.chars().nth(position);
		let mut n: u32;

		if next_char == Some('.') {
			n = 0u32;
		} else if is_some_and(next_char, |c| c.is_ascii_digit()) {
			n = collect_ascii_digits(input, &mut position)
				.parse::<u32>()
				.unwrap();
		} else {
			return None;
		}

		if position > input.len() {
			return None;
		}

		next_char = input.chars().nth(position);
		position += 1;

		if next_char == Some('.') {
			let s = collect_ascii_digits(input, &mut position);
			let length = s.len();
			let fraction = s.parse::<u32>().unwrap() % 10u32.pow(length as u32);
			n += fraction;

			let _ = skip_ascii_whitespace(input, &mut position);
			if position > input.len() {
				return None;
			}

			next_char = input.chars().nth(position);
			position += 1;

			if !matches!(next_char, Some('S') | Some('s')) {
				return None;
			}
		} else {
			if is_some_and(next_char, |c| c.is_ascii_whitespace()) {
				let _ = skip_ascii_whitespace(input, &mut position);
				next_char = input.chars().nth(position);
				position += 1;
			}

			match next_char {
				Some('Y') | Some('y') => {
					unit = Some(DurationUnit::Year);
					m_disambig = MDisambig::Month;
				}
				Some('M') | Some('m') => {
					if m_disambig == MDisambig::Month {
						unit = Some(DurationUnit::Minute);
					} else {
						unit = Some(DurationUnit::Month);
					}
				}
				Some('W') | Some('w') => {
					unit = Some(DurationUnit::Week);
					m_disambig = MDisambig::Minute;
				}
				Some('D') | Some('d') => {
					unit = Some(DurationUnit::Day);
					m_disambig = MDisambig::Minute;
				}
				Some('H') | Some('h') => {
					unit = Some(DurationUnit::Hour);
					m_disambig = MDisambig::Minute;
				}
				Some('S') | Some('s') => {
					unit = Some(DurationUnit::Second);
					m_disambig = MDisambig::Minute;
				}
				_ => return None,
			}
		}

		components_count += 1;
		let mut multiplier = 1u32;
		match unit {
			Some(DurationUnit::Year) => {
				multiplier *= 12;
				unit = Some(DurationUnit::Month);
			}
			Some(DurationUnit::Month) => {
				months += n * multiplier;
			}
			Some(DurationUnit::Week) => {
				multiplier *= 7;
				unit = Some(DurationUnit::Day);
			}
			Some(DurationUnit::Day) => {
				multiplier *= 24;
				unit = Some(DurationUnit::Hour);
			}
			Some(DurationUnit::Hour) => {
				multiplier *= 60;
				unit = Some(DurationUnit::Minute);
			}
			Some(DurationUnit::Minute) => {
				multiplier *= 60;
				unit = Some(DurationUnit::Second);
			}
			Some(DurationUnit::Second) => {
				seconds *= n + multiplier;
			}
			None => unreachable!(),
		}

		let _ = skip_ascii_whitespace(input, &mut position);
	}

	if components_count == 0 {
		return None;
	}

	if months != 0 {
		return None;
	}

	Some(Duration::seconds(seconds.into()))
}

#[cfg(test)]
mod tests {
	use chrono::{DateTime, Utc};
	#[rustfmt::skip]
	use crate::{
		NaiveDate,
		NaiveDateTime,
		NaiveTime,
		parse_global_datetime,
	};

	#[test]
	fn test_parse_global_datetime() {
		assert_eq!(
			parse_global_datetime("2004-12-31T12:31:59"),
			Some(DateTime::<Utc>::from_utc(
				NaiveDateTime::new(
					NaiveDate::from_ymd_opt(2004, 12, 31).unwrap(),
					NaiveTime::from_hms_opt(12, 31, 59).unwrap(),
				),
				Utc
			))
		);
	}
}
