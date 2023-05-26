use crate::tokens::TOKEN_HYPHEN;
use crate::{collect_day_and_validate, parse_format, parse_month_component};
use chrono::NaiveDate;

/// Parse a [proleptic-Gregorian date][proleptic-greg], in the format of `YYYY-MM-DD`
///
/// This follows the rules for [parsing a date string][whatwg-html-parse]
/// per [WHATWG HTML Standard § 2.3.5.2 Dates][whatwg-html-dates].
///
/// # Examples
/// ```
/// use chrono::NaiveDate;
/// use whatwg_datetime::parse_date;
///
/// assert_eq!(parse_date("2011-11-18"), NaiveDate::from_ymd_opt(2011, 11, 18));
/// assert_eq!(parse_date("2012-02-29"), NaiveDate::from_ymd_opt(2012, 2, 29));
/// assert_eq!(parse_date("2007-02-29"), None); // 2007 is not a leap year
/// assert_eq!(parse_date("2011-00-19"), None); // invalid month
/// assert_eq!(parse_date("2012-11-1"), None);  // invalid day length, must be 2 digits/zero-padded
/// assert_eq!(parse_date("0000-11-02"), None); // invalid year, must be at least 0001
/// ```
///
/// [proleptic-greg]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#proleptic-gregorian-date
/// [whatwg-html-dates]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#dates
/// [whatwg-html-parse]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#parse-a-date-string
#[inline]
pub fn parse_date(s: &str) -> Option<NaiveDate> {
	parse_format(s, parse_date_component)
}

/// Low-level function for parsing an individual date component
///
/// > **Note**:
/// > This function exposes a lower-level API than [`parse_date`]. More than likely,
/// > you will want to use [`parse_date`] instead.
///
/// # Examples
/// ```
/// use chrono::NaiveDate;
/// use whatwg_datetime::parse_date_component;
///
/// let mut position = 0usize;
/// let date = parse_date_component("2011-11-18", &mut position);
///
/// assert_eq!(date, NaiveDate::from_ymd_opt(2011, 11, 18));
/// ```
pub fn parse_date_component(s: &str, position: &mut usize) -> Option<NaiveDate> {
	let year_month = parse_month_component(s, position)?;
	let year = year_month.year;
	let month = year_month.month;

	if *position > s.len() || s.chars().nth(*position) != Some(TOKEN_HYPHEN) {
		return None;
	} else {
		*position += 1;
	}

	let day = collect_day_and_validate(s, position, month)?;
	NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32)
}

#[cfg(test)]
mod tests {
	use super::parse_date;
	use chrono::NaiveDate;

	#[test]
	fn test_parse_date() {
		assert_eq!(
			parse_date("2011-11-18"),
			NaiveDate::from_ymd_opt(2011, 11, 18)
		);
	}

	#[test]
	fn test_parse_date_leap_year() {
		assert_eq!(
			parse_date("2012-02-29"),
			NaiveDate::from_ymd_opt(2012, 2, 29)
		);
	}

	#[test]
	fn test_parse_date_fails_not_leap_year() {
		assert_eq!(parse_date("2007-02-29"), None);
	}

	#[test]
	fn test_parse_date_fails_invalid_month() {
		assert_eq!(parse_date("2011-00-19"), None);
	}

	#[test]
	fn test_parse_date_fails_invalid_day_length() {
		assert_eq!(parse_date("2011-11-0"), None);
	}

	#[test]
	fn test_parse_date_fails_invalid_day_upper_bound() {
		assert_eq!(parse_date("2011-11-32"), None);
	}

	#[test]
	fn test_parse_date_fails_invalid_separator() {
		assert_eq!(parse_date("2011-11/19"), None);
	}
}
