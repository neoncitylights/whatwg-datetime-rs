#[doc = include_str!("../README.md")]
mod utils;

use crate::utils::*;
use chrono::{DateTime, Duration, Local, Month, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use whatwg_infra::collect_codepoints;

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
pub struct YearWeek {
	pub(crate) year: i32,
	pub(crate) week: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearlessDate {
	pub(crate) month: u8,
	pub(crate) day: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeZoneOffset {
	pub(crate) hours: i8,
	pub(crate) minutes: i8,
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

pub fn parse_month_string(s: &str) -> Option<(u32, u8)> {
	let mut position = 0usize;
	let parsed = parse_month_component(s, &mut position)?;
	if position < s.len() {
		return None;
	}

	Some(parsed)
}

pub fn parse_month_component(s: &str, position: &mut usize) -> Option<(u32, u8)> {
	let parsed_year = collect_ascii_digits(s, position);
	if parsed_year.len() < 4 {
		return None;
	}

	let year = parsed_year.parse::<u32>().ok()?;
	if *position > s.len() || s.chars().nth(*position) != Some(TOKEN_DATETIME_SEPARATOR) {
		return None;
	} else {
		*position += 1;
	}

	let parsed_month = collect_ascii_digits(s, position);
	let month = parsed_month.parse::<u8>().ok()?;
	if !is_valid_month(&month) {
		return None;
	}

	Some((year, month))
}

pub fn parse_date_string(s: &str) -> Option<(u32, u8, u8)> {
	let mut position = 0usize;
	let parsed = parse_date_component(s, &mut position)?;
	if position < s.len() {
		return None;
	}

	Some(parsed)
}

pub fn parse_date_component(s: &str, position: &mut usize) -> Option<(u32, u8, u8)> {
	let (year, month) = parse_month_component(s, position)?;

	if *position > s.len() || s.chars().nth(*position) != Some(TOKEN_DATETIME_SEPARATOR) {
		return None;
	} else {
		*position += 1;
	}

	let parsed_day = collect_ascii_digits(s, position);
	if parsed_day.len() != 2 {
		return None;
	}

	let max_day = max_days_in_month_year(month, year)?;
	let day = parsed_day.parse::<u8>().ok()?;

	if !(1..=max_day).contains(&day) {
		return None;
	}

	Some((year, month, day))
}

pub fn parse_yearless_date_string(s: &str) -> Option<YearlessDate> {
	let mut position = 0usize;
	let parsed = parse_yearless_date_component(s, &mut position)?;
	if position < s.len() {
		return None;
	}

	Some(parsed)
}

pub fn parse_yearless_date_component(s: &str, position: &mut usize) -> Option<YearlessDate> {
	let collected = collect_codepoints(s, position, |c| c == TOKEN_DATETIME_SEPARATOR);
	if !matches!(collected.len(), 0 | 2) {
		return None;
	}

	let parsed_month = collect_ascii_digits(s, position);
	if parsed_month.len() != 2 {
		return None;
	}

	let month = parsed_month.parse::<u8>().ok()?;
	if !is_valid_month(&month) {
		return None;
	}

	if *position > s.len() || s.chars().nth(*position) != Some(TOKEN_DATETIME_SEPARATOR) {
		return None;
	} else {
		*position += 1;
	}

	let parsed_day = collect_ascii_digits(s, position);
	if parsed_day.len() != 2 {
		return None;
	}

	let day = parsed_day.parse::<u8>().ok()?;
	let max_days = max_days_in_month_year(month, 4).unwrap();
	if !(1..=max_days).contains(&day) {
		return None;
	}

	Some(YearlessDate { month, day })
}

pub fn parse_time_component(s: &str, position: &mut usize) -> Option<NaiveTime> {
	let parsed_hour = collect_ascii_digits(s, position);
	if parsed_hour.len() != 2 {
		return None;
	}

	let hour = parsed_hour.parse::<u8>().ok()?;
	if !is_valid_hour(&hour) {
		return None;
	}

	if *position > s.len() || s.chars().nth(*position) != Some(':') {
		return None;
	} else {
		*position += 1;
	}

	let parsed_minute = collect_ascii_digits(s, position);
	if parsed_minute.len() != 2 {
		return None;
	}
	let minute = parsed_minute.parse::<u8>().ok()?;
	if !is_valid_min_or_sec(&minute) {
		return None;
	}

	let mut second = 0u8;
	if *position < s.len() && s.chars().nth(*position) == Some(':') {
		*position += 1;

		if *position >= s.len() {
			return None;
		}

		let parsed_second =
			collect_codepoints(s, position, |c| c.is_ascii_digit() || c == '.');
		let parsed_second_len = parsed_second.len();
		if parsed_second_len == 3
			|| (parsed_second_len > 3 && parsed_second.chars().nth(3) != Some('.'))
			|| parsed_second.chars().any(|c| c == '.')
		{
			return None;
		}

		second = parsed_second.parse::<u8>().ok()?;
		if !is_valid_min_or_sec(&second) {
			return None;
		}
	}

	NaiveTime::from_hms_opt(hour as u32, minute as u32, second as u32)
}

pub fn parse_local_datetime(s: &str) -> Option<NaiveDateTime> {
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

	let date = NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32)?;
	Some(NaiveDateTime::new(date, time))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeZoneSign {
	Positive,
	Negative,
}

pub fn parse_timezone_offset_component(s: &str, position: &mut usize) -> Option<TimeZoneOffset> {
	let char_at = s.chars().nth(*position);

	let mut minutes = 0i8;
	let mut hours = 0i8;

	match char_at {
		Some('Z') => {
			*position += 1;
		}
		Some('+') | Some('-') => {
			let sign = match char_at {
				Some('+') => TimeZoneSign::Positive,
				Some('-') => TimeZoneSign::Negative,
				_ => unreachable!(),
			};

			*position += 1;

			let collected = collect_ascii_digits(s, position);
			let collected_len = collected.len();
			if collected_len == 2 {
				hours = collected.parse::<i8>().unwrap();
				if *position > s.len() || s.chars().nth(*position) != Some(':') {
					return None;
				} else {
					*position += 1;
				}

				let parsed_mins = collect_ascii_digits(s, position);
				if parsed_mins.len() != 2 {
					return None;
				}

				minutes = parsed_mins.parse::<i8>().unwrap();
			} else if collected_len == 4 {
				let (hour_str, min_str) = collected.split_at(2);
				hours = hour_str.parse::<i8>().unwrap();
				minutes = min_str.parse::<i8>().unwrap();
			} else {
				return None;
			}

			if !(0..=23).contains(&hours) {
				return None;
			}

			if !(0..=59).contains(&minutes) {
				return None;
			}

			if sign == TimeZoneSign::Negative {
				hours *= -1;
				minutes *= -1;
			}
		}
		_ => (),
	}

	Some(TimeZoneOffset { hours, minutes })
}

pub fn parse_week_string(input: &str) -> Option<YearWeek> {
	// Step 1, 2
	let mut position = 0usize;

	// Step 3, 4
	let year_string = collect_ascii_digits(input, &mut position);
	let year = year_string.parse::<i32>().unwrap();
	if year <= 0 {
		return None;
	}

	// Step 5
	if position > input.len() || input.chars().nth(position) != Some(TOKEN_DATETIME_SEPARATOR) {
		return None;
	} else {
		position += 1;
	}

	// Step 6
	if position > input.len() || input.chars().nth(position) != Some(TOKEN_ABBR_WEEK) {
		return None;
	} else {
		position += 1;
	}

	// Step 7
	let parsed_week = collect_ascii_digits(input, &mut position);
	if parsed_week.len() != 2 {
		return None;
	}

	let week = parsed_week.parse::<u8>().unwrap();
	let max_weeks = week_number_of_year(year)?;
	if week < 1 || week > max_weeks {
		return None;
	}

	Some(YearWeek { year, week })
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
		} else if next_char.is_some_and(|c| c.is_ascii_digit()) {
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
			if next_char.is_some_and(|c| c.is_ascii_whitespace()) {
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

	use crate::{parse_global_datetime, parse_week_string};
	#[rustfmt::skip]
	use crate::{
		NaiveDate,
		NaiveDateTime,
		NaiveTime,
		parse_date_component,
		parse_local_datetime,
		parse_month_component,
		parse_time_component,
		parse_timezone_offset_component,
		parse_yearless_date_component,
		TimeZoneOffset,
		YearlessDate,
		YearWeek,
	};

	#[test]
	fn test_parse_month_component() {
		let mut position = 0usize;
		let parsed = parse_month_component("2004-12", &mut position);

		assert_eq!(parsed, Some((2004, 12)));
	}

	#[test]
	fn test_parse_date_component() {
		let mut position = 0usize;
		let parsed = parse_date_component("2004-12-31", &mut position);

		assert_eq!(parsed, Some((2004, 12, 31)));
	}

	#[test]
	fn test_parse_yearless_date_component() {
		let mut position = 0usize;
		let parsed = parse_yearless_date_component("12-31", &mut position);

		assert_eq!(parsed, Some(YearlessDate { month: 12, day: 31 }));
	}

	#[test]
	fn test_parse_time_component() {
		let mut position = 0usize;
		let parsed = parse_time_component("12:31:59", &mut position);

		assert_eq!(parsed, NaiveTime::from_hms_opt(12, 31, 59));
	}

	#[test]
	pub fn test_parse_local_datetime() {
		let parsed = parse_local_datetime("2004-12-31T12:31:59");

		assert_eq!(
			parsed,
			Some(NaiveDateTime::new(
				NaiveDate::from_ymd_opt(2004, 12, 31).unwrap(),
				NaiveTime::from_hms_opt(12, 31, 59).unwrap(),
			))
		);
	}

	#[test]
	pub fn test_parse_timezone_offset_component() {
		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("Z", &mut position);

		assert_eq!(
			parsed,
			Some(TimeZoneOffset {
				hours: 0,
				minutes: 0
			})
		);

		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("+01:00", &mut position);

		assert_eq!(
			parsed,
			Some(TimeZoneOffset {
				hours: 1,
				minutes: 0
			})
		);

		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("-01:00", &mut position);

		assert_eq!(
			parsed,
			Some(TimeZoneOffset {
				hours: -1,
				minutes: 0
			})
		);

		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("+0100", &mut position);

		assert_eq!(
			parsed,
			Some(TimeZoneOffset {
				hours: 1,
				minutes: 0
			})
		);

		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("-0100", &mut position);

		assert_eq!(
			parsed,
			Some(TimeZoneOffset {
				hours: -1,
				minutes: 0
			})
		);

		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("+0100", &mut position);

		assert_eq!(
			parsed,
			Some(TimeZoneOffset {
				hours: 1,
				minutes: 0
			})
		);

		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("-0100", &mut position);

		assert_eq!(
			parsed,
			Some(TimeZoneOffset {
				hours: -1,
				minutes: 0
			})
		);

		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("+0100", &mut position);

		assert_eq!(
			parsed,
			Some(TimeZoneOffset {
				hours: 1,
				minutes: 0
			})
		);

		let mut position = 0usize;
		let parsed = parse_timezone_offset_component("-0100", &mut position);

		assert_eq!(
			parsed,
			Some(TimeZoneOffset {
				hours: -1,
				minutes: 0
			})
		);
	}

	#[test]
	fn test_parse_week_string() {
		assert_eq!(
			parse_week_string("2004-W53"),
			Some(YearWeek {
				year: 2004,
				week: 53
			})
		);
	}

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
