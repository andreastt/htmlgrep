htmlgrep
========

__htmlgrep__ is a suite of command-line tools for searching HTML documents,
that allows selecting elements by various types of selectors.

The suite consists of the following programs:

  - [__css(1)__](./doc/css.1.md)

The tools are built with the HTML tree manipulation library [Kuchiki] (朽木),
which uses the same HTML parser as the [Servo] browser engine.

[grep(1)]: https://man.openbsd.org/grep.1
[kuchiki]: https://github.com/kuchiki-rs/kuchiki
[Servo]: https://servo.org/


Installation
============

Using the [cargo] package manager:

	% cargo install htmlgrep

[cargo]: https://doc.rust-lang.org/cargo/


Usage
=====

As a CLI tool, given the followig HTML document, _first.html_:

	<!doctype html>
	<meta charset=utf>
	<title>My first blog post</title>
	<meta name=keywords content=blog,first,hello>
	<meta name=description content="First entry to blog.">

To find all occurrences of `<meta>` elements:

	% css first.html meta
	first.html	<meta content="blog,first,hello" name="keywords">
	first.html	<meta content="First entry to blog." name="description">

And to only look for `<meta>` elements with a `name` attribute equal
to `keywords` and a `content` attribute containing `blog` in a
space-separated list:

	% css meta[name=keywords][content~=blog] first.html
	first.html	<meta content="blog,first,hello" name="keywords">

It can also receive streaming content from stdin:

	% curl -L https://sny.no/ | css title
	/dev/stdin	<title>Andreas Tolfsen</title>


Library
=======

Programmatically, with the [htmlgrep] crate:

	use htmlgrep;

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

[htmlgrep]: https://crates.io/crates/htmlgrep
