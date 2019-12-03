use std::error::Error;
use std::ffi::OsStr;
use std::fmt;

pub type FlagResult<T> = Result<T, ParseError>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParseError {
	pub kind: ErrorKind,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ErrorKind {
	Overflow,
	Unexpected(char),
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use self::ErrorKind::*;
		let s = match self.kind {
			Overflow => self.description().to_string(),
			Unexpected(ch) => format!("{}: {:?}", self.description(), ch),
		};
		write!(f, "{}", s)
	}
}

impl Error for ParseError {
	fn description(&self) -> &str {
		use self::ErrorKind::*;
		match self.kind {
			Unexpected(_) => "unexpected",
			Overflow => "overflow",
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum Argument {
	/// +42
	Line(i64),

	/// -f bar
	Option(char, String),

	/// -h
	Flag(char),

	Free(String),

	Stdin,
}

impl Argument {
	pub fn as_str(&self) -> Option<&str> {
		match *self {
			Argument::Free(ref s) => Some(s),
			Argument::Option(_, ref s) => Some(s),
			_ => None,
		}
	}
}

#[derive(Debug)]
pub struct Arguments(Vec<Argument>);

impl Arguments {
	pub fn first(&self, flag: char) -> Option<&Argument> {
		for arg in self.0.iter() {
			match arg {
				Argument::Flag(ch) if ch == &flag => return Some(arg),
				Argument::Option(ch, _) if ch == &flag => return Some(arg),
				_ => {}
			}
		}
		None
	}

	pub fn present(&self, flag: char) -> bool {
		self.first(flag).is_some()
	}

	pub fn free(&self) -> Vec<String> {
		self.0
			.iter()
			.filter_map(|arg| match arg {
				Argument::Free(s) => Some(s.clone()),
				_ => None,
			})
			.collect()
	}
}

impl IntoIterator for Arguments {
	type Item = Argument;
	type IntoIter = ::std::vec::IntoIter<Argument>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

struct Lexer<'a> {
	input: &'a str,
	idx: usize,
}

impl<'a> Lexer<'a> {
	fn new(input: &'a str) -> Lexer<'a> {
		Lexer { input, idx: 0 }
	}

	fn byte(&self, idx: usize) -> u8 {
		if idx + self.idx < self.input.len() {
			self.input.as_bytes()[idx + self.idx]
		} else {
			0
		}
	}

	fn rest(&self) -> &str {
		&self.input[self.idx..]
	}

	fn peek_char(&self) -> Option<char> {
		self.rest().chars().next()
	}

	fn take_char(&mut self) -> char {
		if let Some(ch) = self.peek_char() {
			self.idx += 1;
			ch
		} else {
			'\0'
		}
	}

	fn take_rest(&mut self) -> String {
		let mut s = String::new();
		while self.peek_char().is_some() {
			s.push(self.take_char());
		}
		s
	}

	fn is_integer(&self, ns: &str) -> bool {
		for b in ns.bytes() {
			match b {
				b'0'..=b'9' => {}
				_ => return false,
			}
		}
		true
	}

	fn integer(&mut self) -> FlagResult<i64> {
		let base = 10;
		let mut value = 0i64;

		loop {
			let b = self.byte(0);
			let digit = match b {
				b'0'..=b'9' => i64::from(b - b'0'),
				_ => break,
			};

			if let Some(v) = value.checked_mul(base).and_then(|v| v.checked_add(digit)) {
				value = v;
			} else {
				return Err(ParseError {
					kind: ErrorKind::Overflow,
				});
			}
			self.idx += 1;
		}

		Ok(value)
	}

	fn is_alphanumeric(&self, ch: Option<char>) -> bool {
		if let Some(ch) = ch {
			ch.is_alphanumeric()
		} else {
			false
		}
	}

	fn tokenize(&mut self) -> FlagResult<Argument> {
		Ok(match self.take_char() {
			'+' if self.is_integer(self.rest()) => Argument::Line(self.integer()?),
			'-' if self.is_alphanumeric(self.peek_char()) && self.rest().len() > 1 => {
				Argument::Option(self.take_char(), self.take_rest())
			}
			'-' if self.is_alphanumeric(self.peek_char()) => Argument::Flag(self.take_char()),
			'-' => Argument::Stdin,
			x => Argument::Free(format!("{}{}", x, self.take_rest())),
		})
	}
}

pub fn parse<C: IntoIterator>(args: C) -> FlagResult<Arguments>
where
	C::Item: AsRef<OsStr>,
{
	let args: Vec<String> = args
		.into_iter()
		.map(|arg| arg.as_ref().to_str().unwrap().to_owned())
		.collect();

	let mut toks = Arguments(Vec::new());
	for arg in args {
		let tok = Lexer::new(&arg).tokenize()?;
		toks.0.push(tok);
	}

	Ok(toks)
}

#[cfg(test)]
mod tests {
	use super::{parse, Argument};

	#[test]
	fn test_empty() {
		let empty: Vec<&str> = vec![];
		let flags = parse(empty).unwrap();
		let mut iter = flags.into_iter();
		assert!(iter.next().is_none());
	}

	#[test]
	fn test_line() {
		let flags = parse(vec!["+42"]).unwrap();
		let mut iter = flags.into_iter();
		assert_eq!(Some(Argument::Line(42)), iter.next());
	}

	#[test]
	fn test_flag() {
		let flags = parse(vec!["-x"]).unwrap();
		let mut iter = flags.into_iter();
		assert_eq!(Some(Argument::Flag('x')), iter.next());
	}

	#[test]
	fn test_option() {
		let flags = parse(vec!["-fvalue"]).unwrap();
		let mut iter = flags.into_iter();
		assert_eq!(
			Some(Argument::Option('f', "value".to_string())),
			iter.next()
		);
	}

	#[test]
	fn test_free() {
		let flags = parse(vec!["foo"]).unwrap();
		let mut iter = flags.into_iter();
		assert_eq!(Some(Argument::Free("foo".to_string())), iter.next());
	}

	#[test]
	fn test_stdin() {
		let flags = parse(vec!["-"]).unwrap();
		let mut iter = flags.into_iter();
		assert_eq!(Some(Argument::Stdin), iter.next());
	}

	#[test]
	fn test_present() {
		let flags = parse(vec!["-x", "-fvalue"]).unwrap();
		assert!(flags.present('x'));
		assert!(flags.present('f'));
		assert!(!flags.present('v'));
	}

	#[test]
	fn test_first() {
		let flags = parse(vec!["-x", "-ffoo", "-fbar"]).unwrap();
		assert_eq!(Some(&Argument::Flag('x')), flags.first('x'));
		assert_eq!(
			Some(&Argument::Option('f', "foo".to_string())),
			flags.first('f')
		);
		assert_eq!(None, flags.first('v'));
	}

	#[test]
	fn test_line_overflow() {
		assert!(parse(vec!["+9999999999999999999"]).is_err());
	}
}
