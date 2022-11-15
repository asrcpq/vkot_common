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

	pub fn union(&self, other: &Self) -> Self {
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

	pub fn to_le_bytes(&self) {}

	pub fn from_le_bytes() {}
}
