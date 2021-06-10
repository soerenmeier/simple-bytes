

//! # Note
//!	Internally there exists only one position is saved
//! So if you read and write you should keep this in mind


use crate::{Bytes, Cursor, BytesRead, BytesWrite, BytesSeek};

/// A Vec wrapper that implements BytesWrite and BytesRead
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BytesOwned {
	inner: Cursor<Vec<u8>>
}

impl BytesOwned {

	/// Creates an empty Vec.
	pub fn new() -> Self {
		Self {
			inner: Cursor::new(vec![])
		}
	}

	/// Creates a new Vec with the given capacity.
	pub fn with_capacity(cap: usize) -> Self {
		Self {
			inner: Cursor::new(Vec::with_capacity(cap))
		}
	}

	/// Creates a BytesOwned struct.
	pub fn new_raw(position: usize, inner: Vec<u8>) -> Self {
		let mut cursor = Cursor::new(inner);
		cursor.seek(position);
		Self { inner: cursor }
	}

	/// Returns the underlying Vec mutably.
	/// 
	/// Removing items can lead to panics while
	/// reading or writing.
	#[inline]
	pub fn as_mut_vec(&mut self) -> &mut Vec<u8> {
		self.inner.inner_mut()
	}

	/// Returns the underlying Vec.
	#[inline]
	pub fn into_vec(self) -> Vec<u8> {
		self.inner.into_inner()
	}

}

impl BytesRead for BytesOwned {

	#[inline]
	fn as_slice(&self) -> &[u8] {
		self.inner.as_slice()
	}

	#[inline]
	fn len(&self) -> usize {
		self.inner.len()
	}

	#[inline]
	fn remaining(&self) -> &[u8] {
		self.inner.remaining()
	}

	#[inline]
	fn read(&mut self, len: usize) -> &[u8] {
		self.inner.read(len)
	}

	#[inline]
	fn peek(&self, len: usize) -> Option<&[u8]> {
		self.inner.peek(len)
	}

}

impl BytesWrite for BytesOwned {

	#[inline]
	fn as_mut(&mut self) -> &mut [u8] {
		self.inner.as_mut()
	}

	#[inline]
	fn as_bytes(&self) -> Bytes<'_> {
		self.inner.as_bytes()
	}

	/// Returns the remaining mutable slice.
	/// 
	/// If an empty slice is returned, this does not mean
	/// you can't write anymore.
	#[inline]
	fn remaining_mut(&mut self) -> &mut [u8] {
		self.inner.remaining_mut()
	}

	/// Writes a slice. Allocates more space if the slice is
	/// bigger than the `Vec`.
	#[inline]
	fn write(&mut self, slice: &[u8]) {
		self.inner.write(slice)
	}

}

impl BytesSeek for BytesOwned {
	/// Returns the internal position.
	fn position(&self) -> usize {
		self.inner.position()
	}

	/// Sets the internal position, allocating more space
	/// if the position is bigger than the `Vec`.
	fn seek(&mut self, pos: usize) {
		self.inner.seek(pos)
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

	use super::*;


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
		bytes.seek(20);
		assert_eq!(bytes.len(), 23);

		// seek
		bytes.seek(99);
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
		assert_eq!(bytes.remaining().len(), 256);

		let to_read: Vec<u8> = (0..10).collect();
		assert_eq!(to_read.as_slice(), bytes.read(10));
		assert_eq!(bytes.remaining().len(), 256 - 10);

		assert_eq!(10u8, bytes.read_u8());

		// peek
		let to_peek: Vec<u8> = (11..=20).collect();
		assert_eq!(to_peek.as_slice(), bytes.peek(10).unwrap());

		bytes.seek(255);
		assert_eq!(255u8, bytes.read_u8());

	}

}