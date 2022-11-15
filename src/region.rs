use std::io::{Result, Write};

#[derive(Default, Clone, Copy, Debug)]
pub struct Region {
	data: [i16; 4],
}

impl Region {
	pub fn sizebox(size: [i16; 2]) -> Self {
		Self {
			data: [0, 0, size[0], size[1]]
		}
	}

	pub fn new(data: [i16; 4]) -> Self {
		Self { data }
	}

	pub fn len(&self) -> usize {
		if self.is_empty() { return 0 }
		(self.data[2] - self.data[0]) as usize *
			(self.data[3] - self.data[1]) as usize 
	}

	pub fn is_empty(&self) -> bool {
		self.data[0] >= self.data[2] ||
			self.data[1] >= self.data[3]
	}

	pub fn union(&self, other: &Self) -> Self {
		if self.is_empty() { return *other }
		if other.is_empty() { return *self }
		let mut result = Self::default();
		for idx in 0..2 {
			result.data[idx] = self.data[idx].min(other.data[idx])
		}
		for idx in 2..4 {
			result.data[idx] = self.data[idx].max(other.data[idx])
		}
		result
	}

	pub fn intersect(&self, other: &Self) -> Self {
		let mut result = Self::default();
		for idx in 0..2 {
			result.data[idx] = self.data[idx].max(other.data[idx])
		}
		for idx in 2..4 {
			result.data[idx] = self.data[idx].min(other.data[idx])
		}
		result
	}

	pub fn write_le_bytes<W: Write>(&self, mut w: W) -> Result<()> {
		for i in 0..4 {
			w.write(&self.data[i].to_le_bytes())?;
		}
		Ok(())
	}

	pub fn from_le_bytes(bytes: &[u8]) -> Self {
		let data: [i16; 4] = core::array::from_fn(|idx| {
			i16::from_le_bytes(bytes[idx * 2..idx * 2 + 2].try_into().unwrap())
		});
		Self { data }
	}

	pub fn data(&self) -> [i16; 4] {
		self.data
	}
}
