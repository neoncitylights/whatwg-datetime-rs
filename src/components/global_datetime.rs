use crate::tokens::{TOKEN_SPACE, TOKEN_T};
use crate::{parse_date_component, parse_time_component, parse_timezone_offset_component};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};

/// Parse a [proleptic-Gregorian date][proleptic-greg] consisting
/// of a date, time, and an optional time-zone offset
///
/// This follows the rules for [parsing a global datetime string][whatwg-html-parse]
/// per [WHATWG HTML Standard § 2.3.5.7 Global dates and times][whatwg-html-global-datetime].
///
/// # Examples
/// A global date-time string with a time (hours and minutes):
/// ```
/// use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
/// use whatwg_datetime::parse_global_datetime;
///
/// assert_eq!(
///     parse_global_datetime("2011-11-18T14:54Z"),
///     Some(DateTime::<Utc>::from_utc(
///         NaiveDateTime::new(
///             NaiveDate::from_ymd_opt(2011, 11, 18).unwrap(),
///             NaiveTime::from_hms_opt(14, 54, 0).unwrap(),
///         ),
///         Utc,
///     ))
/// );
/// ```
///
/// [proleptic-greg]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#proleptic-gregorian-date
/// [whatwg-html-global-datetime]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#global-dates-and-times
/// [whatwg-html-parse]: https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#parse-a-global-date-and-time-string
pub fn parse_global_datetime(s: &str) -> Option<DateTime<Utc>> {
	let mut position = 0usize;
	let date = parse_date_component(s, &mut position)?;

	let last_char = s.chars().nth(position);
	if position > s.len() || !matches!(last_char, Some(TOKEN_T) | Some(TOKEN_SPACE)) {
		return None;
	} else {
		position += 1;
	}

	let time = parse_time_component(s, &mut position)?;
	if position > s.len() {
		return None;
	}

	let timezone_offset = parse_timezone_offset_component(s, &mut position)?;
	if position < s.len() {
		return None;
	}

	let timezone_offset_as_duration =
		Duration::minutes(timezone_offset.minute as i64 + timezone_offset.hour as i64 * 60);
	let naive_datetime = NaiveDateTime::new(
		date,
		time.overflowing_sub_signed(timezone_offset_as_duration).0,
	);

	Some(DateTime::<Utc>::from_utc(naive_datetime, Utc))
}

#[cfg(test)]
mod tests {
	use super::parse_global_datetime;
	use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};

	#[test]
	fn test_parse_global_datetime_t_hm() {
		assert_eq!(
			parse_global_datetime("2004-12-31T12:31"),
			Some(DateTime::<Utc>::from_utc(
				NaiveDateTime::new(
					NaiveDate::from_ymd_opt(2004, 12, 31).unwrap(),
					NaiveTime::from_hms_opt(12, 31, 0).unwrap(),
				),
				Utc
			))
		);
	}

	#[test]
	fn test_parse_global_datetime_t_hms() {
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

	#[test]
	fn test_parse_global_datetime_t_hms_milliseconds() {
		assert_eq!(
			parse_global_datetime("2027-11-29T12:31:59.123"),
			Some(DateTime::<Utc>::from_utc(
				NaiveDateTime::new(
					NaiveDate::from_ymd_opt(2027, 11, 29).unwrap(),
					NaiveTime::from_hms_milli_opt(12, 31, 59, 123).unwrap(),
				),
				Utc
			))
		);
	}

	#[test]
	fn test_parse_global_datetime_t_hms_z() {
		assert_eq!(
			parse_global_datetime("2004-12-31T12:31:59Z"),
			Some(DateTime::<Utc>::from_utc(
				NaiveDateTime::new(
					NaiveDate::from_ymd_opt(2004, 12, 31).unwrap(),
					NaiveTime::from_hms_opt(12, 31, 59).unwrap(),
				),
				Utc
			))
		);
	}

	#[test]
	fn test_parse_global_datetime_space_hm() {
		assert_eq!(
			parse_global_datetime("2004-12-31 12:31"),
			Some(DateTime::<Utc>::from_utc(
				NaiveDateTime::new(
					NaiveDate::from_ymd_opt(2004, 12, 31).unwrap(),
					NaiveTime::from_hms_opt(12, 31, 0).unwrap(),
				),
				Utc
			))
		);
	}

	#[test]
	fn test_parse_global_datetime_space_hms() {
		assert_eq!(
			parse_global_datetime("2004-12-31 12:31:59"),
			Some(DateTime::<Utc>::from_utc(
				NaiveDateTime::new(
					NaiveDate::from_ymd_opt(2004, 12, 31).unwrap(),
					NaiveTime::from_hms_opt(12, 31, 59).unwrap(),
				),
				Utc
			))
		);
	}

	#[test]
	fn test_parse_global_datetime_space_hms_milliseconds() {
		assert_eq!(
			parse_global_datetime("2004-12-31 12:31:59.123"),
			Some(DateTime::<Utc>::from_utc(
				NaiveDateTime::new(
					NaiveDate::from_ymd_opt(2004, 12, 31).unwrap(),
					NaiveTime::from_hms_milli_opt(12, 31, 59, 123).unwrap(),
				),
				Utc
			))
		);
	}

	#[test]
	fn test_parse_global_datetime_fails_invalid_date() {
		assert_eq!(parse_global_datetime("2004/13/31T12:31"), None);
	}

	#[test]
	fn test_parse_global_datetime_fails_invalid_delimiter() {
		assert_eq!(parse_global_datetime("1986-08-14/12-31"), None);
	}

	#[test]
	fn test_parse_global_datetime_fails_invalid_time() {
		assert_eq!(parse_global_datetime("2006-06-05T24:31"), None);
	}

	#[test]
	fn test_parse_global_datetime_fails_invalid_time_long_pos() {
		assert_eq!(parse_global_datetime("2006-06-05T24:31:5999"), None);
	}

	#[test]
	fn test_parse_global_datetime_fails_invalid_timezone_offset_1() {
		assert_eq!(parse_global_datetime("2019-12-31T11:17+24:00"), None);
	}

	#[test]
	fn test_parse_global_datetime_fails_invalid_timezone_offset_2() {
		assert_eq!(parse_global_datetime("1456-02-24T11:17C"), None);
	}
}
