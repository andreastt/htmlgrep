use std::io::Read;

use kuchiki;
use kuchiki::iter::{Descendants, Elements, Select};
use kuchiki::traits::*;

use result::{Error, Result};

#[derive(Debug, Clone)]
pub struct Match {
	pub file: String,
	pub source: String,
}

pub struct Matches(Select<Elements<Descendants>>);

impl Iterator for Matches {
	type Item = Match;

	fn next(&mut self) -> Option<Self::Item> {
		match self.0.next() {
			Some(css_match) => {
				let node = css_match.as_node();
				let mut raw_source = node.to_string();
				let source = raw_source.replace("\n", "");

				let entry = Match {
					file: "foo.html".into(),
					source,
				};

				Some(entry)
			}
			None => None,
		}
	}
}

pub fn select<R: Read>(selector: &str, mut reader: R) -> Result<Matches> {
	let mut html = String::new();
	reader.read_to_string(&mut html)?;

	// TODO(ato): Provide kuchiki::ParseOpts
	let document = kuchiki::parse_html().one(html);

	// Ok(Matches(document.select(&selector).map_err(|_| Error::Parse)?))

	match document.select(selector) {
		Ok(iter) => Ok(Matches(iter)),
		Err(_) => Err(Error::Parse),
	}
}

pub fn select_first<R: Read>(selector: &str, reader: R) -> Result<Option<Match>> {
	// TODO(ato): Use select_first
	Ok(select(selector, reader)?.next())
}
