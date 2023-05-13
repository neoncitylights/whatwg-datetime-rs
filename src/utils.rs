use chrono::{Datelike, NaiveDate, Weekday};
use whatwg_infra::collect_codepoints;

#[inline]
pub(crate) fn is_valid_month(month: &u8) -> bool {
	(1..=12).contains(month)
}

#[inline]
pub(crate) fn is_valid_hour(hour: &u8) -> bool {
	(0..=23).contains(hour)
}

#[inline]
pub(crate) fn is_valid_min_or_sec(val: &u8) -> bool {
	(0..60).contains(val)
}

#[inline]
pub(crate) fn collect_ascii_digits(s: &str, position: &mut usize) -> String {
	collect_codepoints(s, position, |c| c.is_ascii_digit())
}

#[inline]
pub(crate) fn skip_ascii_whitespace(s: &str, position: &mut usize) -> String {
	collect_codepoints(s, position, |c| c.is_ascii_whitespace())
}

pub fn max_days_in_month_year(month: u8, year: u32) -> Option<u8> {
	match month {
		1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
		4 | 6 | 9 | 11 => Some(30),
		2 => {
			if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
				Some(29)
			} else {
				Some(28)
			}
		}
		_ => None,
	}
}

// https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#weeks
pub fn week_number_of_year(year: i32) -> Option<u8> {
	let naive_date = NaiveDate::from_ymd_opt(year, 1u32, 1u32)?;
	let weekday = naive_date.weekday();

	match weekday {
		Weekday::Thu => Some(53u8),
		Weekday::Wed => {
			if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
				Some(53u8)
			} else {
				Some(52u8)
			}
		}
		_ => Some(52u8),
	}
}

#[cfg(test)]
mod tests {
	#[rustfmt::skip]
	use super::{
		max_days_in_month_year,
		week_number_of_year,
	};

	#[test]
	fn test_max_days_in_month_28_days() {
		assert_eq!(max_days_in_month_year(2, 2021), Some(28));
		assert_eq!(max_days_in_month_year(2, 2022), Some(28));
		assert_eq!(max_days_in_month_year(2, 2023), Some(28));
	}

	#[test]
	fn test_max_days_in_month_29_days() {
		assert_eq!(max_days_in_month_year(2, 2020), Some(29));
		assert_eq!(max_days_in_month_year(2, 2024), Some(29));
		assert_eq!(max_days_in_month_year(2, 2028), Some(29));
		assert_eq!(max_days_in_month_year(2, 2400), Some(29));
	}

	#[test]
	fn test_max_days_in_month_30_days() {
		assert_eq!(max_days_in_month_year(4, 2021), Some(30));
		assert_eq!(max_days_in_month_year(6, 2021), Some(30));
		assert_eq!(max_days_in_month_year(9, 2021), Some(30));
		assert_eq!(max_days_in_month_year(11, 2021), Some(30));
	}

	#[test]
	fn test_max_days_in_month_31_days() {
		assert_eq!(max_days_in_month_year(1, 2021), Some(31));
		assert_eq!(max_days_in_month_year(3, 2019), Some(31));
		assert_eq!(max_days_in_month_year(5, 2000), Some(31));
		assert_eq!(max_days_in_month_year(7, 3097), Some(31));
		assert_eq!(max_days_in_month_year(8, 1985), Some(31));
		assert_eq!(max_days_in_month_year(10, 1426), Some(31));
		assert_eq!(max_days_in_month_year(12, 1953), Some(31));
	}

	#[test]
	fn test_max_days_in_month_nothing() {
		assert_eq!(max_days_in_month_year(13, 2022), None);
	}

	// https://www.epochconverter.com/years
	#[test]
	fn test_week_number_of_year_is_52() {
		assert_eq!(week_number_of_year(2012), Some(52));
		assert_eq!(week_number_of_year(2017), Some(52));
		assert_eq!(week_number_of_year(2018), Some(52));
		assert_eq!(week_number_of_year(2019), Some(52));
		assert_eq!(week_number_of_year(2021), Some(52));
		assert_eq!(week_number_of_year(2022), Some(52));
		assert_eq!(week_number_of_year(2023), Some(52));
	}

	// https://www.epochconverter.com/years
	#[test]
	fn test_week_number_of_year_is_53() {
		assert_eq!(week_number_of_year(1801), Some(53));
		assert_eq!(week_number_of_year(2004), Some(53));
		assert_eq!(week_number_of_year(2009), Some(53));
		assert_eq!(week_number_of_year(2015), Some(53));
		assert_eq!(week_number_of_year(2020), Some(53));
	}

	/// Test for the corner case where the first day of the year is a Wednesday,
	/// but the year isn't a leap year
	#[test]
	fn test_week_number_of_year_starts_on_wednesday_and_not_leap_year_is_52() {
		assert_eq!(week_number_of_year(2014), Some(52));
		assert_eq!(week_number_of_year(2025), Some(52));
	}
}
