#![feature(min_const_generics)]

use regex::Regex;

#[cfg(test)]
mod tests {
	use super::Stringtools;

	#[test]
	fn slice() {
		assert_eq!("abcdefg".slice(3, None), Some("defg"));
		assert_eq!("abcdefg".slice(3, -1), Some("def"));
		assert_eq!("𝖆𝖇𝖈𝖉𝖊𝖋𝖌".slice(3, None), Some("𝖉𝖊𝖋𝖌"));
		assert_eq!("𝖆𝖇𝖈𝖉𝖊𝖋𝖌".slice(3, -1), Some("𝖉𝖊𝖋"));
	}

	#[test]
	fn match_regex() {
		assert_eq!(
			"shiny white bags contain 2 dark purple bags."
				.match_regex(&r#"(.*) contain (.*)\."#.parse().unwrap()),
			Some(["shiny white bags", "2 dark purple bags"])
		)
	}
}

pub trait Stringtools {
	fn as_str(&self) -> &str;

	fn slice(&self, start: isize, end: impl Into<Option<isize>>) -> Option<&str> {
		let mut s = self.as_str();

		let total_len = s.chars().count();

		let (start_index, _) = s.char_indices().nth(start as _)?;
		s = &s[start_index..];

		if let Some(mut end) = end.into() {
			if end < 0 {
				end = end.rem_euclid(total_len as _);
			}
			let len = end - start;
			let (end_index, _) = s.char_indices().nth(len as _)?;
			s = &s[0..end_index];
		}

		Some(s)
	}

	fn match_regex<const NUM: usize>(&self, re: &Regex) -> Option<[&str; NUM]> {
		let c = re.captures(self.as_str())?;
		let mut matches = [""; NUM];
		for (i, s) in c.iter().skip(1).enumerate() {
			let s = s?.as_str();
			matches[i] = s;
		}
		Some(matches)
	}
}

impl<T: AsRef<str>> Stringtools for T {
	fn as_str(&self) -> &str {
		self.as_ref()
	}
}
