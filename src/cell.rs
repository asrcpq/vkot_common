use std::io::{Result, Write};

fn read_u32(bytes: &[u8]) -> u32 {
	u32::from_le_bytes(bytes[0..4].try_into().unwrap())
}

#[derive(Clone, Copy)]
pub struct Cell {
	pub ch: u32,
	pub fg: u32,
	pub bg: u32,
	// bold italic underline
	pub de: u32,
}

impl Default for Cell {
	fn default() -> Self {
		Self {
			ch: 0x20,
			fg: u32::MAX,
			bg: 0,
			de: 0,
		}
	}
}

impl std::fmt::Debug for Cell {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Cell {{ ch: {}, fg: {:x}, bg: {:x}, de: {:x} }}",
			self.ch,
			self.fg,
			self.bg,
			self.de,
		)
	}
}

impl Cell {
	pub fn with_unic(mut self, unic: u32) -> Self {
		self.ch = unic;
		self
	}

	pub fn write_le_bytes<W: Write>(&self, mut w: W) -> Result<()> {
		w.write(&self.ch.to_le_bytes())?;
		w.write(&self.fg.to_le_bytes())?;
		w.write(&self.bg.to_le_bytes())?;
		w.write(&self.de.to_le_bytes())?;
		Ok(())
	}

	pub fn from_le_bytes(bytes: &[u8]) -> Self {
		let ch = read_u32(&bytes[0..]);
		let fg = read_u32(&bytes[4..]);
		let bg = read_u32(&bytes[8..]);
		let de = read_u32(&bytes[12..]);
		Self { ch, fg, bg, de }
	}
}
