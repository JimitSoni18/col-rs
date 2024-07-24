#[derive(Debug)]
pub enum Weight {
	Bold,
	Faint,
}

pub type RGB = (u8, u8, u8);

#[derive(Debug, Default)]
pub struct Style {
    weight: Option<crate::Weight>,
    fg: Option<crate::RGB>,
    bg: Option<crate::RGB>,
	underline: bool,
	strike_through: bool,
	italic: bool,
}

impl Style {
	// TODO: add separate r, g, b parameters instead of tuple (BREAKING CHANGE)
	#[inline]
	pub fn fg(mut self, color: RGB) -> Self {
		self.fg = Some(color);
		self
	}

	// TODO: add separate r, g, b parameters instead of tuple (BREAKING CHANGE)
	#[inline]
	pub fn bg(mut self, color: RGB) -> Self {
		self.bg = Some(color);
		self
	}

	// TODO: add `.bold()` method
	#[inline]
	pub fn weight(mut self, weight: Weight) -> Self {
		self.weight = Some(weight);
		self
	}

	#[inline]
	pub fn underline(mut self) -> Self {
		self.underline = true;
		self
	}

	#[inline]
	pub fn strike_through(mut self) -> Self {
		self.strike_through = true;
		self
	}

	#[inline]
	pub fn italic(mut self) -> Self {
		self.italic = true;
		self
	}

	#[inline]
	pub fn reset(&mut self) {
		*self = Self::default();
	}
}

