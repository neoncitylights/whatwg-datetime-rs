use crate::tokens::{TOKEN_DOT, TOKEN_P, TOKEN_T};
use crate::utils::collect_ascii_digits;
use chrono::Duration;
use whatwg_infra::collect_codepoints;

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

	if input.chars().nth(position) != Some(TOKEN_P) {
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

		if input.chars().nth(position) == Some(TOKEN_T) {
			m_disambig = MDisambig::Minute;
			let _ = skip_ascii_whitespace(input, &mut position);
		}

		next_char = input.chars().nth(position);
		let mut n: u32;

		if next_char == Some(TOKEN_DOT) {
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

		if next_char == Some(TOKEN_DOT) {
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

#[inline]
fn skip_ascii_whitespace(s: &str, position: &mut usize) -> String {
	collect_codepoints(s, position, |c| c.is_ascii_whitespace())
}

#[cfg(test)]
mod tests {
	use super::skip_ascii_whitespace;

	#[test]
	fn test_skip_ascii_whitespace_empty() {
		let mut position = 0usize;
		assert_eq!(skip_ascii_whitespace("", &mut position), String::new());
	}

	#[test]
	fn test_skip_ascii_whitespace() {
		let mut position = 0usize;
		let s = "   test";
		let skip = skip_ascii_whitespace(s, &mut position);
		assert_eq!(skip, "   ");
		assert_eq!(position, 3);
		assert_eq!(&s[position..], "test");
	}
}
