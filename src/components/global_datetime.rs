use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};

use crate::{parse_date_component, parse_time_component, parse_timezone_offset_component};

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

#[cfg(test)]
mod tests {
	use super::parse_global_datetime;
	use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};

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
