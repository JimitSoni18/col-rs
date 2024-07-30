#[derive(Debug, Clone, Default)]
pub enum Weight {
    #[default]
    Normal,
	Bold,
	Faint,
}

pub type RGB = [u8;3];

#[derive(Debug, Default, Clone)]
pub struct Style {
	pub(crate) weight: Weight,
	pub(crate) fg: Option<[u8; 3]>,
	pub(crate) bg: Option<[u8; 3]>,
	pub(crate) underline: bool,
	pub(crate) strike_through: bool,
	pub(crate) italic: bool,
}

impl Style {
	#[inline]
	pub fn fg(mut self, r: u8, g: u8, b: u8) -> Self {
		self.fg = Some([r, g, b]);
		self
	}

	#[inline]
	pub fn set_fg(&mut self, r: u8, g: u8, b: u8) {
		self.fg = Some([r, g, b]);
	}

	#[inline]
	pub fn reset_fg(&mut self) {
		self.fg = None;
	}

	#[inline]
	pub fn get_fg(&self) -> &Option<RGB> {
		&self.fg
	}

	#[inline]
	pub fn bg(mut self, r: u8, g: u8, b: u8) -> Self {
		self.bg = Some([r,g,b]);
		self
	}

	#[inline]
	pub fn set_bg(&mut self, r: u8, g: u8, b: u8) {
		self.bg = Some([r, g, b]);
	}

	#[inline]
	pub fn reset_bg(&mut self) {
		self.bg = None;
	}

	#[inline]
	pub fn get_bg(&self) -> &Option<RGB> {
		&self.bg
	}

	// TODO: add `.bold()` method
	#[inline]
	pub fn weight(mut self, weight: Weight) -> Self {
		self.weight = weight;
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
