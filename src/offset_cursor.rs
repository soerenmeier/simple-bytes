
use crate::{BytesRead, BytesWrite, BytesSeek, Bytes};

/// A Cursor which holds a specific offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OffsetCursor<T> {
	// the offset gets applied to the inner
	// position
	offset: usize,
	inner: T
}

impl<T> OffsetCursor<T> {

	/// Creates a new Cursor.
	///
	/// May panic if the offset is bigger than the inner len.
	/// Depending on inner.
	pub fn new(mut inner: T, offset: usize) -> Self
	where T: BytesRead + BytesSeek {
		inner.seek(inner.position() + offset);
		Self { inner, offset }
	}

	/// Updates the offset.
	///
	/// Maybe panic if there aren't enough bytes left.
	pub fn set_offset(&mut self, offset: usize)
	where T: BytesSeek {
		let prev_pos = self.inner.position() - self.offset;
		self.inner.seek(prev_pos + offset);
		self.offset = offset;
	}

	/// Returns the inner value as a reference.
	pub fn inner(&self) -> &T {
		&self.inner
	}

	/// Returns the inner value as a mutable reference.
	/// Shrinking the inner len or updating the position may lead
	/// to panics when reading or writing.
	pub fn inner_mut(&mut self) -> &mut T {
		&mut self.inner
	}

	/// Returns the inner value, discarding the offset.
	pub fn into_inner(self) -> T {
		self.inner
	}

}

impl<T> BytesRead for OffsetCursor<T>
where T: BytesRead {

	#[inline]
	fn as_slice(&self) -> &[u8] {
		&self.inner.as_slice()[self.offset..]
	}

	#[inline]
	fn remaining(&self) -> &[u8] {
		self.inner.remaining()
	}

	fn read(&mut self, len: usize) -> &[u8] {
		self.inner.read(len)
	}

	fn peek(&self, len: usize) -> Option<&[u8]> {
		self.inner.peek(len)
	}

}

impl<T> BytesSeek for OffsetCursor<T>
where T: BytesSeek {
	/// Returns the internal position.
	fn position(&self) -> usize {
		self.inner.position() - self.offset
	}

	/// Sets the internal position.
	/// 
	/// ## Panics
	/// Depending on the implementation.
	fn seek(&mut self, pos: usize) {
		self.inner.seek(self.offset + pos)
	}
}

impl<T> BytesWrite for OffsetCursor<T>
where T: BytesWrite {

	fn as_mut(&mut self) -> &mut [u8] {
		&mut self.inner.as_mut()[self.offset..]
	}

	fn as_bytes(&self) -> Bytes<'_> {
		self.inner.as_bytes().inner()[self.offset..].into()
	}

	fn remaining_mut(&mut self) -> &mut [u8] {
		self.inner.remaining_mut()
	}

	fn write(&mut self, slice: &[u8]) {
		self.inner.write(slice)
	}

}



#[cfg(test)]
mod tests {

	use super::*;
	use crate::Cursor;

	#[test]
	fn write() {

		let cursor = Cursor::new(vec![1, 2, 3, 4]);
		let mut offset_cursor = OffsetCursor::new(cursor, 2);
		assert_eq!(offset_cursor.remaining_mut().len(), 2);
		offset_cursor.write(&[1]);
		assert_eq!(offset_cursor.remaining_mut().len(), 1);
		offset_cursor.write(&[2]);
		assert_eq!(offset_cursor.remaining_mut().len(), 0);
		offset_cursor.write(&[1, 2]);

		assert_eq!(offset_cursor.as_mut(), &[1, 2, 1, 2]);

	}

	#[test]
	fn read() {

		let cursor = Cursor::new(vec![1, 2, 3, 4]);
		let mut offset_cursor = OffsetCursor::new(cursor, 2);
		assert_eq!(offset_cursor.position(), 0);
		offset_cursor.seek(1);
		assert_eq!(offset_cursor.position(), 1);
		assert_eq!(offset_cursor.as_slice(), &[3, 4]);
		assert_eq!(offset_cursor.remaining(), &[4]);

		offset_cursor.set_offset(1);
		assert_eq!(offset_cursor.position(), 1);
		assert_eq!(offset_cursor.as_slice(), &[2, 3, 4]);
		assert_eq!(offset_cursor.remaining(), &[3, 4]);

	}

}