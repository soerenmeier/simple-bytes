
use crate::{BytesRead, BytesWrite, BytesSeek, Bytes};

/// A generic struct implementing BytesRead, BytesWrite and BytesSeek
/// for different types.
/// 
/// In the background Bytes, BytesMut and
/// BytesOwned use this.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cursor<T> {
	/// always points to the next position
	position: usize,
	inner: T
}

impl<T> Cursor<T> {

	/// Creates a new Cursor.
	pub fn new(inner: T) -> Self {
		Self {
			position: 0,
			inner
		}
	}

	/// Returns the inner value as a reference.
	pub fn inner(&self) -> &T {
		&self.inner
	}

	/// Returns the inner value as a mutable reference.
	/// Shrinking the inner len may lead to panics when reading
	/// or writing.
	pub fn inner_mut(&mut self) -> &mut T {
		&mut self.inner
	}

	/// Returns the inner value, discarding how many bytes
	/// were read or written.
	pub fn into_inner(self) -> T {
		self.inner
	}

}

impl<T> BytesRead for Cursor<T>
where T: AsRef<[u8]> {

	#[inline]
	fn as_slice(&self) -> &[u8] {
		self.inner.as_ref()
	}

	#[inline]
	fn remaining(&self) -> &[u8] {
		&self.as_slice()[self.position..]
	}

	fn read(&mut self, len: usize) -> &[u8] {
		let slice = &self.inner.as_ref()[self.position..][..len];
		// the previous line did not panic
		// so let's increase our position
		self.position += len;
		slice
	}

	fn peek(&self, len: usize) -> Option<&[u8]> {
		self.remaining().get(..len)
	}

}

impl<'a> BytesSeek for Cursor<&'a [u8]> {
	/// Sets the internal position.
	/// 
	/// ## Panics
	/// If the position exceeds the slice.
	fn seek(&mut self, pos: usize) {
		let len = self.inner.len();
		assert!(self.position + len > pos);
		self.position = pos;
	}
}

impl<'a> BytesWrite for Cursor<&'a mut [u8]> {

	fn as_mut(&mut self) -> &mut [u8] {
		self.inner
	}

	fn as_bytes(&self) -> Bytes<'_> {
		Bytes::new(0, &*self.inner)
	}

	fn remaining_mut(&mut self) -> &mut [u8] {
		&mut self.inner[self.position..]
	}

	fn write(&mut self, slice: &[u8]) {
		self.remaining_mut()[..slice.len()].copy_from_slice(slice);
		self.position += slice.len();
	}

}

impl<'a> BytesSeek for Cursor<&'a mut [u8]> {
	/// Sets the internal position.
	/// 
	/// ## Panics
	/// If the position exceeds the slice.
	fn seek(&mut self, pos: usize) {
		let len = self.inner.len();
		assert!(self.position + len > pos);
		self.position = pos;
	}
}


impl BytesWrite for Cursor<Vec<u8>> {

	fn as_mut(&mut self) -> &mut [u8] {
		&mut self.inner
	}

	fn as_bytes(&self) -> Bytes<'_> {
		Bytes::new(0, &self.inner)
	}

	/// Returns the remaining mutable slice.
	/// 
	/// If an empty slice is returned, this does not mean
	/// you can't write anymore.
	fn remaining_mut(&mut self) -> &mut [u8] {
		&mut self.inner[self.position..]
	}

	/// Write a slice. Allocates more space if the slice is
	/// bigger than the `Vec`.
	fn write(&mut self, slice: &[u8]) {
		// if has enough space
		if slice.len() <= self.remaining_mut().len() {
			self.remaining_mut()[..slice.len()].copy_from_slice(slice);
			self.position += slice.len();
			return;
		}

		// not enough space
		let rem = self.remaining_mut().len();
		if rem > 0 {
			self.remaining_mut().copy_from_slice(&slice[..rem]);
		}

		self.inner.extend_from_slice(&slice[rem..]);
		self.position += slice.len();
	}

}

impl BytesSeek for Cursor<Vec<u8>> {
	/// Sets the internal position, allocating more space
	/// if the position is bigger than the `Vec`.
	fn seek(&mut self, pos: usize) {
		self.position = pos;
		let n_len = self.position + 1;
		if self.inner.len() < n_len {
			self.inner.resize(n_len, 0u8);
		}
	}
}