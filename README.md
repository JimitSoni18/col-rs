# col-rs

a small rust library to print 24 bit colored terminal text

---

## Usage

```rs
use col_rs::{Span, Weight};

fn main() {
	let span1 = Span::new("colo".to_string())
		.fg((234, 156, 128))
		.bg((90, 157, 255))
		.italic();

	let span2 = Span::new('-'.to_string());

	let span3 = Span::new("rs".to_string())
		.fg((255, 56, 56))
		.bg((68, 234, 158))
		.weight(Weight::Bold);

	println!("{span1}{span2}{span3}");
}
```
