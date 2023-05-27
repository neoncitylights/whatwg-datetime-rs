use crate::tokens::TOKEN_HYPHEN;
use crate::utils::is_valid_month;
use crate::{collect_day_and_validate, collect_month_and_validate, parse_format};
use whatwg_infra::collect_codepoints;

/// A yearless date, consisting of a gregorian month and a day
/// within the month, without an associated year.
///
/// # Examples
///
/// ```
/// use whatwg_datetime::{parse_yearless_date, YearlessDate};
///
/// assert_eq!(parse_yearless_date("11-18"), YearlessDate::new_opt(11, 18));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearlessDate {
	pub(crate) month: u32,
	pub(crate) day: u32,
}

impl YearlessDate {
	#[inline]
	pub(crate) fn new(month: u32, day: u32) -> Self {
		Self { month, day }
	}

	/// Creates a new `YearlessDate` from a month and a day.
	///
	/// This asserts that the month is in between 1 through 12,
	/// inclusive, and that the day is in the valid range for
	/// the month. Specifically:
	/// - February must be between 1 and 29, inclusive
	/// - April, June, September, and November must be between 1 and 30, inclusive
	/// - All other months must be between 1 and 31, inclusive
	///
	/// # Examples
	/// ```
	/// use whatwg_datetime::YearlessDate;
	///
	/// assert!(YearlessDate::new_opt(11, 18).is_some());
	/// assert!(YearlessDate::new_opt(2, 29).is_some());
	/// assert!(YearlessDate::new_opt(2, 30).is_none()); // February never has 30 days
	/// assert!(YearlessDate::new_opt(4, 31).is_none()); // April only has 30 days
	/// assert!(YearlessDate::new_opt(13, 1).is_none()); // There are only 12 months
	/// assert!(YearlessDate::new_opt(12, 32).is_none()); // December only has 31 days
	/// ```
	#[rustfmt::skip]
	pub fn new_opt(month: u32, day: u32) -> Option<Self> {
		if !is_valid_month(&month) {
			return None;
		}

		match month {
			2 => if day > 29 { return None; },
			4 | 6 | 9 | 11 => if day > 30 { return None; },
			_ => if day > 31 { return None; },
		}

		Some(Self::new(month, day))
	}

	/// A month component. This is a number from 1 to 12, inclusive.
	///
	/// # Examples
	/// ```
	/// use whatwg_datetime::YearlessDate;
	///
	/// let yearless_date = YearlessDate::new_opt(11, 18).unwrap();
	/// assert_eq!(yearless_date.month(), 11);
	/// ```
	#[inline]
	pub const fn month(&self) -> u32 {
		self.month
	}

	/// A day component. This is a number from 1 to the max number
	/// of days in the month, inclusive.
	///
	/// # Examples
	/// ```
	/// use whatwg_datetime::YearlessDate;
	///
	/// let yearless_date = YearlessDate::new_opt(11, 18).unwrap();
	/// assert_eq!(yearless_date.day(), 18);
	/// ```
	#[inline]
	pub const fn day(&self) -> u32 {
		self.day
	}
}

/// Parses a string consisting of a gregorian month and a day
/// within the month, without an associated year
///
/// This follows the rules for [parsing a yearless date string][whatwg-html-parse]
/// per [WHATWG HTML Standard § 2.3.5.3 Yearless dates][whatwg-html-yearless].
///
/// # Examples
/// ```
/// use whatwg_datetime::{parse_yearless_date, YearlessDate};
///
/// assert_eq!(parse_yearless_date("11-18"), YearlessDate::new_opt(11, 18));
/// assert_eq!(parse_yearless_date("02-29"), YearlessDate::new_opt(2, 29));
/// assert_eq!(parse_yearless_date("02-30"), None); // February never has 30 days
/// assert_eq!(parse_yearless_date("04-31"), None); // April only has 30 days
/// assert_eq!(parse_yearless_date("13-01"), None);
/// ```
///
/// [whatwg-html-yearless]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#yearless-dates
/// [whatwg-html-parse]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#parse-a-yearless-date-string
#[inline]
pub fn parse_yearless_date(s: &str) -> Option<YearlessDate> {
	parse_format(s, parse_yearless_date_component)
}

/// Low-level function for parsing an individual yearless date component
/// at a given position
///
/// This follows the rules for [parsing a yearless date component][whatwg-html-parse]
/// per [WHATWG HTML Standard § 2.3.5.3 Yearless dates][whatwg-html-yearless].
///
/// > **Note**:
/// > This function exposes a lower-level API than [`parse_yearless_date`].
/// > More than likely, you will want to use [`parse_yearless_date`] instead.
///
/// # Examples
/// ```
/// use whatwg_datetime::{parse_yearless_date_component, YearlessDate};
///
/// let mut position = 0usize;
/// let date = parse_yearless_date_component("11-18", &mut position);
///
/// assert_eq!(date, YearlessDate::new_opt(11, 18));
/// ```
///
/// [whatwg-html-yearless]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#yearless-dates
/// [whatwg-html-parse]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#parse-a-yearless-date-component
pub fn parse_yearless_date_component(s: &str, position: &mut usize) -> Option<YearlessDate> {
	let collected = collect_codepoints(s, position, |c| c == TOKEN_HYPHEN);
	if !matches!(collected.len(), 0 | 2) {
		return None;
	}

	let month = collect_month_and_validate(s, position)?;
	if *position > s.len() || s.chars().nth(*position) != Some(TOKEN_HYPHEN) {
		return None;
	} else {
		*position += 1;
	}

	let day = collect_day_and_validate(s, position, month)?;
	Some(YearlessDate::new(month, day))
}

#[cfg(test)]
mod tests {
	#[rustfmt::skip]
	use super::{
		parse_yearless_date,
		parse_yearless_date_component,
		YearlessDate,
	};

	#[test]
	fn test_parse_yearless_date() {
		assert_eq!(
			parse_yearless_date("11-18"),
			Some(YearlessDate::new(11, 18))
		);
	}

	#[test]
	fn test_parse_yearless_date_fails_empty_string() {
		assert_eq!(parse_yearless_date(""), None);
	}

	#[test]
	fn test_parse_yearless_date_fails_separator() {
		assert_eq!(parse_yearless_date("11/18"), None);
	}

	#[test]
	fn test_parse_yearless_date_fails_month_upper_bound() {
		assert_eq!(parse_yearless_date("13-01"), None);
	}

	#[test]
	fn test_parse_yearless_date_fails_month_length() {
		assert_eq!(parse_yearless_date("1-01"), None);
	}

	#[test]
	fn test_parse_yearless_date_fails_day_lower_bound() {
		assert_eq!(parse_yearless_date("01-00"), None);
	}

	#[test]
	fn test_parse_yearless_date_fails_day_upper_bound() {
		assert_eq!(parse_yearless_date("01-32"), None);
	}

	#[test]
	fn test_parse_yearless_date_fails_day_length() {
		assert_eq!(parse_yearless_date("01-9"), None);
	}

	#[test]
	fn test_parse_yearless_date_component() {
		let mut position = 0usize;
		let parsed = parse_yearless_date_component("12-31", &mut position);

		assert_eq!(parsed, Some(YearlessDate::new(12, 31)));
	}

	#[test]
	fn test_parse_yearless_date_component_fails_empty_string() {
		let mut position = 0usize;
		let parsed = parse_yearless_date_component("", &mut position);

		assert_eq!(parsed, None);
	}

	#[test]
	fn test_parse_yearless_date_only_one_separator() {
		let mut position = 0usize;
		let parsed = parse_yearless_date_component("-", &mut position);

		assert_eq!(parsed, None);
	}
}
