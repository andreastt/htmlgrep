extern crate htmlgrep;

fn main() {
	let input = r#"
		<!doctype html>
		<meta charset=utf>
		<title>My first blog post</title>
		<meta name=keywords content=blog,first,hello>
		<meta name=description content="First entry to blog.">
	"#;

	let matches = htmlgrep::select("meta[name=keywords]", input.as_bytes()).unwrap();

	for node in matches {
		println!("{}", node.source);
	}
}
