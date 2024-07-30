mod style;
pub use style::{Style, Weight, RGB};

#[derive(Default, Debug, Clone)]
pub struct Span {
	text: String,
	style: Style,
}

impl Span {
	#[inline]
	pub fn new(text: String) -> Self {
		Self {
			text,
			..Default::default()
		}
	}

	#[inline]
	pub fn with_style(text: String, style: Style) -> Self {
		Self { text, style }
	}

	#[inline]
	pub fn add_style(&mut self, style: Style) {
        self.style = style;
	}

	#[inline]
	pub fn fg(mut self, r: u8, g: u8, b: u8) -> Self {
		self.style.fg = Some([r, g, b]);
		self
	}

	#[inline]
	pub fn set_fg(&mut self, r: u8, g: u8, b: u8) {
		self.style.set_fg(r, g, b)
	}

	#[inline]
	pub fn reset_fg(&mut self) {
		self.style.reset_fg()
	}

	#[inline]
	pub fn get_fg(&self) -> &Option<RGB> {
		self.style.get_fg()
	}

	#[inline]
	pub fn bg(mut self, r: u8, g: u8, b: u8) -> Self {
		self.style.bg = Some([r, g, b]);
		self
	}

	#[inline]
	pub fn set_bg(&mut self, r: u8, g: u8, b: u8) {
		self.style.set_bg(r, g, b)
	}

	#[inline]
	pub fn reset_bg(&mut self) {
		self.style.reset_bg()
	}

	#[inline]
	pub fn get_bg(&self) -> &Option<RGB> {
		self.style.get_bg()
	}

	// TODO: add `.bold()` method
	#[inline]
	pub fn weight(mut self, weight: Weight) -> Self {
		self.style.weight = weight;
		self
	}

	#[inline]
	pub fn underline(mut self) -> Self {
		self.style.underline = true;
		self
	}

	#[inline]
	pub fn strike_through(mut self) -> Self {
		self.style.strike_through = true;
		self
	}

	#[inline]
	pub fn italic(mut self) -> Self {
		self.style.italic = true;
		self
	}

	#[inline]
	pub fn reset(&mut self) {
		*self = Self::default();
	}
}

// TODO: add css colors enum, strum_macros, helper methods like
// "text".blue()

// courtesy, claude.ai
impl std::fmt::Display for Span {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut codes = String::new();

		// Add foreground color
		if let Some(fg) = self.style.fg {
			if !codes.is_empty() {
				codes.push(';');
			}
			codes.push_str(&format!("38;2;{};{};{}", fg[0], fg[1], fg[2]));
		}

		// Add background color
		if let Some(bg) = self.style.bg {
			if !codes.is_empty() {
				codes.push(';');
			}
			codes.push_str(&format!("48;2;{};{};{}", bg[0], bg[1], bg[2]));
		}

		// Add other styles
		match self.style.weight {
			Weight::Bold => {
				if !codes.is_empty() {
					codes.push(';');
				}
				codes.push_str("1");
			}
			Weight::Faint => {
				if !codes.is_empty() {
					codes.push(';');
				}
				codes.push_str("2");
			}
			Weight::Normal => {}
		};

		if self.style.underline {
			if !codes.is_empty() {
				codes.push(';');
			}
			codes.push_str("4");
		}

		if self.style.strike_through {
			if !codes.is_empty() {
				codes.push(';');
			}
			codes.push_str("9");
		}

		if self.style.italic {
			if !codes.is_empty() {
				codes.push(';');
			}
			codes.push_str("3");
		}

		// Write the escape sequence
		write!(f, "\x1b[{}m", codes)?;

		// Write the text
		write!(f, "{}", self.text)?;

		// Reset all attributes
		write!(f, "\x1b[0m")
	}

	// old implementation using vec
	//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	//     let mut codes = Vec::new();

	//     // Add foreground color

	//     if let Some(fg) = self.style.fg {

	//         codes.push(format!("38;2;{};{};{}", fg[0], fg[1], fg[2]));

	//     }

	//     // Add background color

	//     if let Some(bg) = self.style.bg {

	//         codes.push(format!("48;2;{};{};{}", bg[0], bg[1], bg[2]));

	//     }

	//     // Add other styles

	//     match self.style.weight {

	//         Weight::Bold => codes.push("1".to_string()),

	//         Weight::Faint => codes.push("2".to_string()),

	//         Weight::Normal => {},

	//     };

	//     if self.style.underline {

	//         codes.push("4".to_string());

	//     }

	//     if self.style.strike_through {

	//         codes.push("9".to_string());

	//     }

	//     if self.style.italic {

	//         codes.push("3".to_string());

	//     }

	//     // Write the escape sequence

	//     write!(f, "\x1b[{}m", codes.join(";"))?;

	//     // Write the text

	//     write!(f, "{}", self.text)?;

	//     // Reset all attributes

	//     write!(f, "\x1b[0m")

	// }
}

pub fn hex_to_rgb(hex: &str) -> Result<RGB, Error> {
	let hex = hex.strip_prefix('#').unwrap_or(hex);
	match hex.len() {
		3 => {
			let map_err = |_| Error::ParseIntError;
			let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(map_err)?;
			let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(map_err)?;
			let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(map_err)?;
			Ok([r, g, b])
		}
		6 => {
			let map_err = |_| Error::ParseIntError;
			let r = u8::from_str_radix(&hex[0..2], 16).map_err(map_err)?;
			let g = u8::from_str_radix(&hex[2..4], 16).map_err(map_err)?;
			let b = u8::from_str_radix(&hex[4..6], 16).map_err(map_err)?;
			Ok([r, g, b])
		}
		_ => Err(Error::LengthInvalid),
	}
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
	#[error("unable to convert, hex code should be of length 3 or 6")]
	LengthInvalid,
	#[error("unable to parse into integer")]
	ParseIntError,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_hex_to_rgb_conversion_6_no_hash() -> Result<(), Error> {
		let hex = "22fa31";
		let rgb = hex_to_rgb(hex)?;
		assert_eq!(rgb, [34, 250, 49]); // Expected RGB tuple

		Ok(())
	}

	#[test]
	fn test_hex_to_rgb_conversion_6_no_hash_caps() -> Result<(), Error> {
		let hex = "22FA31";
		let rgb = hex_to_rgb(hex)?;
		assert_eq!(rgb, [34, 250, 49]); // Expected RGB tuple

		Ok(())
	}

	#[test]
	fn test_hex_to_rgb_conversion_6_hash_caps() -> Result<(), Error> {
		let hex = "#22FA31";
		let rgb = hex_to_rgb(hex)?;
		assert_eq!(rgb, [34, 250, 49]); // Expected RGB tuple

		Ok(())
	}

	#[test]
	fn test_hex_to_rgb_conversion_6_hash() -> Result<(), Error> {
		let hex = "#22fa31";
		let rgb = hex_to_rgb(hex)?;
		assert_eq!(rgb, [34, 250, 49]); // Expected RGB tuple

		Ok(())
	}

	#[test]
	fn test_hex_to_rgb_conversion_3_hash() -> Result<(), Error> {
		let hex = "#2aa";
		let rgb = hex_to_rgb(hex)?;
		assert_eq!(rgb, [34, 170, 170]); // Expected RGB tuple

		Ok(())
	}

	#[test]
	fn test_hex_to_rgb_conversion_3_no_hash() -> Result<(), Error> {
		let hex = "2ac";
		let rgb = hex_to_rgb(hex)?;
		assert_eq!(rgb, [34, 170, 204]); // Expected RGB tuple

		Ok(())
	}

	#[test]
	fn test_hex_to_rgb_conversion_3_hash_caps() -> Result<(), Error> {
		let hex = "#2aa";
		let rgb = hex_to_rgb(hex)?;
		assert_eq!(rgb, [34, 170, 170]); // Expected RGB tuple

		Ok(())
	}

	#[test]
	fn test_hex_to_rgb_conversion_3_caps() -> Result<(), Error> {
		let hex = "2AA";
		let rgb = hex_to_rgb(hex)?;
		assert_eq!(rgb, [34, 170, 170]); // Expected RGB tuple

		Ok(())
	}

	#[test]
	fn test_hex_to_rgb_invalid_length() {
		let hex = "#4faf";
		let rgb = hex_to_rgb(hex);

		assert!(matches!(rgb, Err(Error::LengthInvalid)));
	}

	#[test]
	fn test_hex_to_rgb_invalid_hex_code() {
		let hex = "#rfa";
		let rgb = hex_to_rgb(hex);

		assert!(matches!(rgb, Err(Error::ParseIntError)));
	}
}
