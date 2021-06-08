

//! # Note
//!	Internally there exists only one position is saved
//! So if you read and write you should keep this in mind


use crate::{ Bytes, BytesRead, BytesWrite };

/// A Vec wrapper that implements BytesWrite and BytesRead
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BytesOwned {
	position: usize,
	inner: Vec<u8>
}

impl BytesOwned {

	/// Creates an empty Vec
	pub fn new() -> Self {
		Self {
			position: 0,
			inner: Vec::new()
		}
	}

	pub fn with_capacity(cap: usize) -> Self {
		Self {
			position: 0,
			inner: Vec::with_capacity(cap)
		}
	}

	pub fn new_raw(position: usize, inner: Vec<u8>) -> Self {
		Self { position, inner }
	}

	#[inline]
	pub fn as_mut_vec(&mut self) -> &mut Vec<u8> {
		&mut self.inner
	}

	#[inline]
	pub fn into_vec(self) -> Vec<u8> {
		self.inner
	}

}

impl BytesRead for BytesOwned {

	#[inline]
	fn as_slice(&self) -> &[u8] {
		&self.inner
	}

	#[inline]
	fn len(&self) -> usize {
		self.inner.len()
	}

	#[inline]
	fn remaining(&self) -> &[u8] {
		&self.inner[self.position..]
	}

	#[inline]
	fn seek(&mut self, pos: usize) {
		assert!(pos < self.len(), "new position exceeds slice length");
		self.position = pos;
	}

	#[inline]
	fn read(&mut self, len: usize) -> &[u8] {
		let n_len = self.position + len;
		let slice = &self.inner[self.position..n_len];
		self.position = n_len;
		slice
	}

	#[inline]
	fn peek(&self, len: usize) -> &[u8] {
		let n_len = self.position + len;
		&self.inner[self.position..n_len]
	}

}

impl BytesWrite for BytesOwned {

	#[inline]
	fn as_mut(&mut self) -> &mut [u8] {
		&mut self.inner
	}

	#[inline]
	fn as_bytes(&self) -> Bytes<'_> {
		(&*self.inner).into()
	}

	/// Be aware that this does not return the entire possible remaining mut slice
	/// because this is a vec
	#[inline]
	fn remaining_mut(&mut self) -> &mut [u8] {
		&mut self.inner[self.position..]
	}

	/// Sets the internal position
	///
	/// if pos >= self.len()
	/// resizes the vector to the appropriate len
	#[inline]
	fn seek_mut(&mut self, pos: usize) {
		self.position = pos;// at this position we write in next step
		// so self.len() == self.position + 1
		let n_len = self.position + 1;
		if self.inner.len() < n_len {
			self.inner.resize(n_len, 0u8)
		}
	}

	#[inline]
	fn write(&mut self, slice: &[u8]) {
		// use optimized extend from slice
		if self.inner.len() == self.position {
			// so position is not included in slice
			self.inner.extend_from_slice(slice);
			self.position += slice.len();
			return;
		}

		let n_len = self.position + slice.len();
		if n_len > self.inner.len() {
			self.inner.resize(n_len, 0u8);
		}

		let pos = self.position;
		self.as_mut()[pos..n_len].copy_from_slice(slice);
		self.position = n_len;
	}

}

impl From<Vec<u8>> for BytesOwned {
	fn from(b: Vec<u8>) -> Self {
		Self::new_raw(0, b)
	}
}

impl From<BytesOwned> for Vec<u8> {
	fn from(b: BytesOwned) -> Self {
		b.into_vec()
	}
}


#[cfg(test)]
mod tests {

	use super::BytesOwned;
	use crate::{ BytesRead, BytesWrite };


	#[test]
	fn write() {

		let mut bytes = BytesOwned::new();
		assert_eq!(bytes.len(), 0);

		let to_write: Vec<u8> = (0..10).collect();
		bytes.write(&to_write);
		bytes.write(&to_write);

		assert_eq!(bytes.len(), 20);

		assert_eq!(&bytes.as_mut()[..10], to_write.as_slice());
		assert_eq!(&bytes.as_mut()[10..20], to_write.as_slice());

		bytes.write_u8(5u8);
		bytes.write_u16(20u16);

		assert_eq!(bytes.len(), 23);

		// seek
		bytes.seek_mut(20);
		assert_eq!(bytes.len(), 23);

		// seek
		bytes.seek_mut(99);
		assert_eq!(bytes.len(), 100);
		// should now write to the 99 byte // this is the last byte
		bytes.write_u8(5u8);
		assert_eq!(bytes.as_mut()[99], 5u8);
		assert_eq!(bytes.len(), 100);

	}

	#[test]
	fn read() {
		use crate::BytesRead;

		let bytes: Vec<u8> = (0..=255).collect();
		let mut bytes: BytesOwned = bytes.into();

		assert_eq!(bytes.as_slice(), bytes.as_slice());
		assert_eq!(bytes.len(), 256);
		assert_eq!(bytes.remaining_len(), 256);

		let to_read: Vec<u8> = (0..10).collect();
		assert_eq!(to_read.as_slice(), bytes.read(10));
		assert_eq!(bytes.remaining_len(), 256 - 10);

		assert_eq!(10u8, bytes.read_u8());

		// peek
		let to_peek: Vec<u8> = (11..=20).collect();
		assert_eq!(to_peek.as_slice(), bytes.peek(10));

		bytes.seek(255);
		assert_eq!(255u8, bytes.read_u8());

	}

}