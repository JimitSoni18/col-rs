pub type RGB = (u8, u8, u8);

#[derive(Debug)]
pub enum Weight {
	Bold,
	Light,
}

#[derive(Default, Debug)]
pub struct Span {
	text: String,
	fg: RGB,
	bg: RGB,
	weight: Option<Weight>,
	underline: bool,
	strike_through: bool,
	italic: bool,
}

impl Span {
	pub fn new(text: String) -> Self {
		Self {
			text,
			fg: (255, 255, 255), // Default to white foreground
			bg: (0, 0, 0),       // Default to black background
			weight: None,
			underline: false,
			strike_through: false,
			italic: false,
		}
	}

	pub fn fg(mut self, color: RGB) -> Self {
		self.fg = color;
		self
	}

	pub fn bg(mut self, color: RGB) -> Self {
		self.bg = color;
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
		codes.push(format!("38;2;{};{};{}", self.fg.0, self.fg.1, self.fg.2));

		// Add background color
		codes.push(format!("48;2;{};{};{}", self.bg.0, self.bg.1, self.bg.2));

		// Add other styles
		if let Some(weight) = &self.weight {
			codes.push(match weight {
				Weight::Bold => "1".to_string(),
				Weight::Light => "2".to_string(),
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
            let map_err = |_| {
                Error::ParseIntError
            };
            let r = u8::from_str_radix(&hex[0..1], 16).map_err(map_err)?;
            let g = u8::from_str_radix(&hex[1..2], 16).map_err(map_err)?;
            let b = u8::from_str_radix(&hex[2..3], 16).map_err(map_err)?;
            Ok((r, g, b))
        }
		6 => {
            let map_err = |_| {
                Error::ParseIntError
            };
            let r = u8::from_str_radix(&hex[0..2], 16).map_err(map_err)?;
            let g = u8::from_str_radix(&hex[2..4], 16).map_err(map_err)?;
            let b = u8::from_str_radix(&hex[4..6], 16).map_err(map_err)?;
            Ok((r, g, b))
        }
		_ => Err(Error::LengthMismatch),
	}
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unable to convert, hex code should be of length 3 or 6")]
    LengthMismatch,
    #[error("unable to parse into integer")]
    ParseIntError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb_conversion() -> Result<(), Error> {
        let hex = "22fa31";
        let rgb = hex_to_rgb(hex)?;
        println!("{:?}", rgb);

        let hex = "22Fa31";
        let rgb = hex_to_rgb(hex)?;
        println!("{:?}", rgb);

        // let hex = "2r1";
        // let rgb = hex_to_rgb(hex)?;
        // println!("{:?}", rgb);

        let hex = "AAA";
        let rgb = hex_to_rgb(hex)?;
        println!("{:?}", rgb);

        Ok(())
    }
}
