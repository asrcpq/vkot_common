pub mod cell {
	fn read_u32(bytes: &[u8]) -> u32 {
		u32::from_le_bytes(bytes[0..4].try_into().unwrap())
	}

	#[derive(Clone, Copy, Debug)]
	pub struct Cell {
		pub ch: u32,
		pub fg: u32,
		pub bg: u32,
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

	impl Cell {
		pub fn with_unic(mut self, unic: u32) -> Self {
			self.ch = unic;
			self
		}

		pub fn to_le_bytes(&self) -> Vec<u8> {
			[
				self.ch.to_le_bytes(),
				self.fg.to_le_bytes(),
				self.bg.to_le_bytes(),
				self.de.to_le_bytes(),
			].concat()
		}

		pub fn from_le_bytes(bytes: &[u8]) -> Self {
			let ch = read_u32(&bytes[0..]);
			let fg = read_u32(&bytes[4..]);
			let bg = read_u32(&bytes[8..]);
			let de = read_u32(&bytes[12..]);
			Self { ch, fg, bg, de }
		}
	}
}
