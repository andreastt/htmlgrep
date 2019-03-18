use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

fn doc(out: PathBuf) {
	let result = Command::new("pandoc")
		.arg("doc/htmlgrep.1.md")
		.arg("-t")
		.arg("man")
		.arg("-o")
		.arg(out.join("htmlgrep.1"))
		.output();

	match result {
		Err(ref e) if e.kind() == io::ErrorKind::NotFound => eprintln!("      missing pandoc"),
		_ => {
			result.expect("could not run pandoc");
		}
	}
}

fn main() {
	let profile = env::var("PROFILE").unwrap();
	let out = Path::new("target").join(&profile);

	println!("     Generating man pages");
	doc(out.to_path_buf());
}
