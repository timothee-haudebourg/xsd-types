/// Returns the quotient and reminder of `a/b`.
pub fn div_rem(a: u32, b: u32) -> (u32, u32) {
	(a / b, a % b)
}

/// Find the earliest position of the given pattern `pattern` in `bytes` that is
/// greater or equal to `offset`.
pub fn byte_index_of(bytes: &[u8], mut offset: usize, pattern: impl BytePattern) -> Option<usize> {
	while offset < bytes.len() {
		if pattern.matches(bytes[offset]) {
			return Some(offset);
		}

		offset += 1
	}

	None
}

pub trait BytePattern {
	fn matches(&self, b: u8) -> bool;
}

impl BytePattern for u8 {
	fn matches(&self, b: u8) -> bool {
		*self == b
	}
}

impl<const N: usize> BytePattern for [u8; N] {
	fn matches(&self, b: u8) -> bool {
		for &a in self {
			if a == b {
				return true;
			}
		}

		false
	}
}
