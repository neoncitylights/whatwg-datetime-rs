mod date;
mod global_datetime;
mod local_datetime;
mod month;
mod time;
mod timezone_offset;
pub(crate) mod tokens;
mod week;
mod yearless_date;

pub use self::date::*;
pub use self::global_datetime::*;
pub use self::local_datetime::*;
pub use self::month::*;
pub use self::time::*;
pub use self::timezone_offset::*;
pub use self::week::*;
pub use self::yearless_date::*;
use crate::utils::collect_ascii_digits;
use crate::utils::is_valid_month;
use crate::utils::max_days_in_month_year;

pub(crate) fn parse_format<T, F>(s: &str, parse_fn: F) -> Option<T>
where
	F: FnOnce(&str, &mut usize) -> Option<T>,
{
	let mut position = 0usize;
	let parsed = parse_fn(s, &mut position)?;
	if position < s.len() {
		return None;
	}

	Some(parsed)
}

pub(crate) fn collect_day_and_validate(s: &str, position: &mut usize, month: u32) -> Option<u32> {
	let parsed_day = collect_ascii_digits(s, position);
	if parsed_day.len() != 2 {
		return None;
	}

	let day = parsed_day.parse::<u32>().ok()?;
	let max_days = max_days_in_month_year(month, 4).unwrap();
	if !(1..=max_days).contains(&day) {
		return None;
	}

	Some(day)
}

pub(crate) fn collect_month_and_validate(s: &str, position: &mut usize) -> Option<u32> {
	let parsed_month = collect_ascii_digits(s, position);
	if parsed_month.len() != 2 {
		return None;
	}

	let month = parsed_month.parse::<u32>().ok()?;
	if !is_valid_month(&month) {
		return None;
	}

	Some(month)
}
