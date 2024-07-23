pub type RGB = (u8, u8, u8);

#[derive(Debug)]
pub enum Weight {
	Bold,
	Faint,
}

#[derive(Default, Debug)]
pub struct Span {
	text: String,
	fg: Option<RGB>,
	bg: Option<RGB>,
	weight: Option<Weight>,
	underline: bool,
	strike_through: bool,
	italic: bool,
}

impl Span {
	pub fn new(text: String) -> Self {
		Self {
			text,
			fg: None,
			bg: None,
			weight: None,
			underline: false,
			strike_through: false,
			italic: false,
		}
	}

	pub fn fg(mut self, color: RGB) -> Self {
		self.fg = Some(color);
		self
	}

	pub fn bg(mut self, color: RGB) -> Self {
		self.bg = Some(color);
		self
	}

	pub fn weight(mut self, weight: Weight) -> Self {
		self.weight = Some(weight);
		self
	}

	pub fn underline(mut self) -> Self {
		self.underline = true;
		self
	}

	pub fn strike_through(mut self) -> Self {
		self.strike_through = true;
		self
	}

	pub fn italic(mut self) -> Self {
		self.italic = true;
		self
	}

	pub fn reset(&mut self) {
		*self = Self::default();
	}
}

impl std::fmt::Display for Span {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut codes = Vec::new();

		// Add foreground color
        if let Some(fg) = self.fg {
            codes.push(format!("38;2;{};{};{}", fg.0, fg.1, fg.2));
        }

		// Add background color
        if let Some(bg) = self.bg {
            codes.push(format!("48;2;{};{};{}", bg.0, bg.1, bg.2));
        }

		// Add other styles
		if let Some(weight) = &self.weight {
			codes.push(match weight {
				Weight::Bold => "1".to_string(),
				Weight::Faint => "2".to_string(),
			});
		}
		if self.underline {
			codes.push("4".to_string());
		}
		if self.strike_through {
			codes.push("9".to_string());
		}
		if self.italic {
			codes.push("3".to_string());
		}

		// Write the escape sequence
		write!(f, "\x1b[{}m", codes.join(";"))?;

		// Write the text
		write!(f, "{}", self.text)?;

		// Reset all attributes
		write!(f, "\x1b[0m")
	}
}

pub fn hex_to_rgb(hex: &str) -> Result<RGB, Error> {
	let hex = hex.strip_prefix('#').unwrap_or(hex);
	match hex.len() {
		3 => {
			let map_err = |_| Error::ParseIntError;
			let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(map_err)?;
			let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(map_err)?;
			let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(map_err)?;
			Ok((r, g, b))
		}
		6 => {
			let map_err = |_| Error::ParseIntError;
			let r = u8::from_str_radix(&hex[0..2], 16).map_err(map_err)?;
			let g = u8::from_str_radix(&hex[2..4], 16).map_err(map_err)?;
			let b = u8::from_str_radix(&hex[4..6], 16).map_err(map_err)?;
			Ok((r, g, b))
		}
		_ => Err(Error::LengthInvalid),
	}
}

#[derive(Debug, thiserror::Error)]
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
		assert_eq!(rgb, (34, 250, 49)); // Expected RGB tuple

		Ok(())
	}

	#[test]
	fn test_hex_to_rgb_conversion_6_no_hash_caps() -> Result<(), Error> {
		let hex = "22FA31";
		let rgb = hex_to_rgb(hex)?;
		assert_eq!(rgb, (34, 250, 49)); // Expected RGB tuple

		Ok(())
	}

	#[test]
	fn test_hex_to_rgb_conversion_6_hash_caps() -> Result<(), Error> {
		let hex = "#22FA31";
		let rgb = hex_to_rgb(hex)?;
		assert_eq!(rgb, (34, 250, 49)); // Expected RGB tuple

		Ok(())
	}

	#[test]
	fn test_hex_to_rgb_conversion_6_hash() -> Result<(), Error> {
		let hex = "#22fa31";
		let rgb = hex_to_rgb(hex)?;
		assert_eq!(rgb, (34, 250, 49)); // Expected RGB tuple

		Ok(())
	}

	#[test]
	fn test_hex_to_rgb_conversion_3_hash() -> Result<(), Error> {
		let hex = "#2aa";
		let rgb = hex_to_rgb(hex)?;
		assert_eq!(rgb, (34, 170, 170)); // Expected RGB tuple

		Ok(())
	}

	#[test]
	fn test_hex_to_rgb_conversion_3_no_hash() -> Result<(), Error> {
		let hex = "2ac";
		let rgb = hex_to_rgb(hex)?;
		assert_eq!(rgb, (34, 170, 204)); // Expected RGB tuple

		Ok(())
	}

	#[test]
	fn test_hex_to_rgb_conversion_3_hash_caps() -> Result<(), Error> {
		let hex = "#2aa";
		let rgb = hex_to_rgb(hex)?;
		assert_eq!(rgb, (34, 170, 170)); // Expected RGB tuple

		Ok(())
	}

	#[test]
	fn test_hex_to_rgb_conversion_3_caps() -> Result<(), Error> {
		let hex = "2AA";
		let rgb = hex_to_rgb(hex)?;
		assert_eq!(rgb, (34, 170, 170)); // Expected RGB tuple

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
