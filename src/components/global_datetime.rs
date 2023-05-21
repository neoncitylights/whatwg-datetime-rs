use crate::tokens::TOKEN_T;
use crate::{parse_date_component, parse_time_component, parse_timezone_offset_component};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};

pub fn parse_global_datetime(s: &str) -> Option<DateTime<Utc>> {
	let mut position = 0usize;
	let date = parse_date_component(s, &mut position)?;

	let last_char = s.chars().nth(position);
	if position > s.len() || !matches!(last_char, Some(TOKEN_T) | Some(' ')) {
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

	let timezone_offset_as_duration = Duration::minutes(
		timezone_offset.minutes as i64 + timezone_offset.hours as i64 * 60,
	);
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
