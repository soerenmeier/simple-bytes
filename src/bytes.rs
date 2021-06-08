
use crate::{BytesRead, BytesSeek, Cursor};

/// A slice wrapper that implements BytesRead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bytes<'a> {
	inner: Cursor<&'a [u8]>
}

impl<'a> Bytes<'a> {

	/// You should probably use:
	///
	/// ```
	/// # use simple_bytes::Bytes;
	/// # let slice = &[1u8, 2u8][..];
	/// let bytes: Bytes = slice.into();
	/// // or
	/// let bytes = Bytes::from(slice);
	/// ```
	///
	pub fn new(position: usize, inner: &'a [u8]) -> Self {
		let mut cursor = Cursor::new(inner);
		cursor.seek(position);
		Self { inner: cursor }
	}

}

impl BytesRead for Bytes<'_> {

	// returns the full slice
	#[inline]
	fn as_slice(&self) -> &[u8] {
		self.inner.as_slice()
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

impl BytesSeek for Bytes<'_> {
	/// Sets the internal position.
	/// 
	/// ## Panics
	/// If the position exceeds the slice.
	fn seek(&mut self, pos: usize) {
		self.inner.seek(pos)
	}
}

impl<'a> From<&'a [u8]> for Bytes<'a> {
	fn from(s: &'a [u8]) -> Self {
		Self::new(0, s)
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn read() {

		let bytes: Vec<u8> = (0..=255).collect();
		let mut bytes = Bytes::from(bytes.as_slice());
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

	#[test]
	#[should_panic]
	fn read_out_of_bound() {

		let bytes = [0u8; 100];
		let mut bytes = Bytes::from(&bytes[..]);

		bytes.seek(100);

		let _ = bytes.read_u8();
	}

}