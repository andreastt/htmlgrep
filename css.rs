extern crate htmlgrep;

use std::env;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::result;

use htmlgrep::flag;
use htmlgrep::path;

const EX_SUCCESS: i32 = 0;
const EX_USAGE: i32 = 64;
const EX_SOFTWARE: i32 = 70;
const EX_IOERR: i32 = 74;

enum FatalError {
	Usage(String),
	Parsing(flag::ParseError),
	Grep(htmlgrep::Error),
	Io(io::Error),
	IoFile((io::Error, PathBuf)),
}

impl FatalError {
	fn exit_code(&self) -> i32 {
		use FatalError::*;
		match *self {
			Usage(_) | Parsing(_) => EX_USAGE,
			Grep(_) => EX_SOFTWARE,
			Io(_) | IoFile(_) => EX_IOERR,
		}
	}
}

impl From<flag::ParseError> for FatalError {
	fn from(err: flag::ParseError) -> Self {
		FatalError::Parsing(err)
	}
}

impl From<htmlgrep::Error> for FatalError {
	fn from(err: htmlgrep::Error) -> Self {
		FatalError::Grep(err)
	}
}

impl From<io::Error> for FatalError {
	fn from(err: io::Error) -> Self {
		FatalError::Io(err)
	}
}

impl From<(io::Error, PathBuf)> for FatalError {
	fn from((err, path): (io::Error, PathBuf)) -> Self {
		FatalError::IoFile((err, path))
	}
}

impl fmt::Display for FatalError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use FatalError::*;
		let s = match *self {
			Usage(ref s) => s.clone(),
			Parsing(ref err) => err.to_string(),
			Grep(ref err) => err.to_string(),
			Io(ref err) => err.to_string(),
			IoFile((ref err, ref path)) => format!("{}: {}", path.display(), err),
		};
		write!(f, "error: {}", s)
	}
}

macro_rules! usage {
	($msg:expr) => {
		return Err(FatalError::Usage($msg.to_string()));
	};

	($fmt:expr, $($arg:tt)+) => {
		return Err(FatalError::Usage(format!($fmt, $($arg)+)));
	};
}

type ProgramResult<T> = result::Result<T, FatalError>;

#[derive(Clone, Copy, Debug)]
struct Options {
	separator: char,
	count: bool,
	hide_filename: bool,
	max: Option<usize>,
}

#[derive(Debug)]
enum Operation {
	GrepFiles {
		selector: String,
		files: Vec<PathBuf>,
		options: Options,
	},
	GrepStdin {
		selector: String,
		input: String,
		options: Options,
	},
}

macro_rules! parse {
	($opt_arg:expr, $msg:expr) => {{
		match $opt_arg {
			Some(flag::Argument::Option(_, s)) => match s.parse() {
				Ok(num) => Some(num),
				Err(e) => usage!("conversion problem: {}", e),
			},
			_ => None,
			}
		}};
}

fn determine_operation(flags: &mut flag::Arguments) -> ProgramResult<Operation> {
	let mut options = Options {
		separator: if flags.present('0') { '\0' } else { '\n' },
		count: flags.present('c'),
		hide_filename: flags.present('h'),
		max: parse!(flags.first('m'), "invalid max count"),
	};

	let mut free = flags.free();
	free.reverse();

	let selector = match free.pop() {
		Some(s) => s.clone(),
		None => usage!("expected expression"),
	};

	let op = if free.is_empty() {
		let mut input = String::new();
		io::stdin().read_to_string(&mut input)?;
		options.hide_filename = true;
		Operation::GrepStdin {
			selector,
			input,
			options,
		}
	} else {
		let files: Vec<PathBuf> = free
			.into_iter()
			.map(|s| {
				let p = PathBuf::from(s);
				p.canonicalize().unwrap_or(p)
			})
			.collect();
		options.hide_filename = files.len() == 1;
		Operation::GrepFiles {
			selector,
			files,
			options,
		}
	};

	Ok(op)
}

fn inner_main(argv: env::Args) -> ProgramResult<()> {
	let mut flags = flag::parse(argv.skip(1))?;

	match determine_operation(&mut flags)? {
		Operation::GrepStdin {
			selector,
			input,
			options,
		} => {
			let stdin = Path::new("/dev/stdin");
			let matches = htmlgrep::select(&selector, input.as_bytes())?;

			if options.count {
				print_count(matches, options);
			} else {
				print_matches(&stdin, matches, options);
			}

			Ok(())
		}

		Operation::GrepFiles {
			selector,
			files,
			options,
		} => {
			for path in files {
				let file = File::open(path.clone()).map_err(|e| (e, path.clone()))?;
				let matches = htmlgrep::select(&selector, file)?;

				if options.count {
					print_count(matches, options);
				} else {
					print_matches(&path, matches, options);
				}
			}

			Ok(())
		}
	}
}

fn print_count(matches: htmlgrep::Matches, opts: Options) {
	let count = matches.count();
	match opts.max {
		Some(n) if count > n => print!("{}{}", n, opts.separator),
		Some(_) | None => print!("{}{}", count, opts.separator),
	}
}

fn print_matches(path: &Path, matches: htmlgrep::Matches, opts: Options) {
	for (n, res) in matches.enumerate() {
		if let Some(max) = opts.max {
			if n == max {
				break;
			}
		}
		print_result(path, res, opts);
	}
}

fn print_result(path: &Path, node: htmlgrep::Match, opts: Options) {
	if opts.hide_filename {
		print!("{}{}", node.source, opts.separator);
	} else {
		let cwd = env::current_dir().unwrap();
		let rel_path = { path::relative(&path, &cwd).unwrap() };
		print!("{}\t{}{}", rel_path.display(), node.source, opts.separator);
	}
}

fn main() {
	use std::process::exit;

	// use std::process::Termination when it graduates
	exit(match inner_main(env::args()) {
		Ok(_) => EX_SUCCESS,

		Err(e) => {
			print_error(&e);
			print_usage();

			e.exit_code()
		}
	});
}

fn print_error(err: &FatalError) {
	eprintln!("{}: {}", prog(), err);
}

fn print_usage() {
	eprintln!("usage: {} [-c0h] [-m <num>] <selector> [<file>...]", prog());
}

fn prog() -> String {
	env::args().next().unwrap()
}
