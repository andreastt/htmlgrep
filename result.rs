use std::error;
use std::fmt;
use std::io;
use std::result;

#[derive(Debug)]
pub enum Error {
	Parse,
	Io(io::Error),
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match self {
			Error::Parse => "invalid selector",
			Error::Io(_) => "i/o error",
		}
	}

	fn cause(&self) -> Option<&error::Error> {
		match self {
			Error::Parse => None,
			Error::Io(ref e) => Some(e),
		}
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let s = match * self {
			Error::Parse => {
				use std::error::Error;
				self.description().to_string()
			}
			Error::Io(ref err) => err.to_string(),
		};
		write!(f, "{}", s)
	}
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Self {
		Error::Io(err)
	}
}

pub type Result<T> = result::Result<T, Error>;
