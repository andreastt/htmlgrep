use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn doc(out: PathBuf) {
	Command::new("pandoc")
		.arg("doc/htmlgrep.1.md")
		.arg("-t")
		.arg("man")
		.arg("-o")
		.arg(out.join("htmlgrep.1"))
		.output()
		.expect("missing pandoc");
}

fn main() {
	let profile = env::var("PROFILE").unwrap();
	let out = Path::new("target").join(&profile);

	println!("     Generating man pages");
	doc(out.to_path_buf());
}
