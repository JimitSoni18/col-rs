use col_rs::{Span, Weight};

fn main() {
    println!("colorterm: {}", std::env::var("COLORTERM").unwrap());
	let span1 = Span::new("Red on Blue".to_string())
		.fg((255, 0, 0))
		.bg((0, 0, 255));

	let span2 = Span::new("Green Italic".to_string())
		.fg((0, 255, 0))
		.italic();

	let span3 = Span::new("Yellow Bold Underlined".to_string())
		.fg((255, 255, 0))
		.weight(Weight::Bold)
		.underline();

	let span4 = Span::new("Yellow Bold Strike-Through".to_string())
		.fg((255, 255, 0))
		.weight(Weight::Bold)
		.strike_through();

	let span5 = Span::new("Red Faint Strike-Through".to_string())
		.fg((255, 0, 0))
		.weight(Weight::Faint)
		.strike_through();

	let span6 = Span::new("Normal text".to_string());

	println!("{}", span1);
	println!("{}", span2);
	println!("{}", span3);
	println!("{}", span4);
	println!("{}", span5);
	println!("{}", span6);
}
