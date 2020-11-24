
use crate::{ Bytes, BytesWrite };

/// A mutable slice wrapper that implements BytesWrite
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BytesMut<'a> {
	position: usize,
	inner: &'a mut [u8]
}

impl<'a> BytesMut<'a> {

	/// You should probably use
	///
	/// ```
	/// # use simple_bytes::BytesMut;
	/// # let slice = &mut [1u8, 2u8][..];
	/// let bytes: BytesMut = slice.into();
	/// // or
	/// let bytes = BytesMut::from( slice );
	/// ```
	///
	pub fn new( position: usize, inner: &'a mut [u8] ) -> Self {
		Self { position, inner }
	}

	/// returns the full slice len
	pub fn len( &self ) -> usize {
		self.inner.len()
	}

	pub fn remaining_len( &self ) -> usize {
		self.inner.len() - self.position
	}

}

impl BytesWrite for BytesMut<'_> {

	#[inline]
	fn as_mut( &mut self ) -> &mut [u8] {
		self.inner
	}

	#[inline]
	fn as_bytes( &self ) -> Bytes<'_> {
		(&*self.inner).into()
	}

	#[inline]
	fn remaining_mut( &mut self ) -> &mut [u8] {
		&mut self.inner[self.position..]
	}

	/// Sets the internal position
	///
	/// # Panics
	/// if pos >= self.len()
	#[inline]
	fn seek_mut( &mut self, pos: usize ) {
		assert!( pos < self.len(), "new position exceeds slice length" );
		self.position = pos;
	}

	#[inline]
	fn write( &mut self, slice: &[u8] ) {
		let end = self.position + slice.len();
		self.inner[self.position..end].copy_from_slice( slice );
		self.position = end;
	}

}

impl<'a> From<&'a mut [u8]> for BytesMut<'a> {
	fn from( s: &'a mut [u8] ) -> Self {
		Self::new( 0, s )
	}
}


#[cfg(test)]
mod tests {

	use super::BytesMut;
	use crate::BytesWrite;
	use crate::BytesRead;

	#[test]
	fn write() {

		let mut bytes = [0u8; 100];
		let mut bytes = BytesMut::from( &mut bytes[..] );
		assert_eq!( bytes.len(), 100 );

		let to_write: Vec<u8> = (0..10).collect();
		bytes.write( &to_write );
		bytes.write( &to_write );

		assert_eq!( bytes.remaining_len(), 100 - 20 );
		assert_eq!( bytes.remaining_len(), bytes.remaining_mut().len() );

		assert_eq!( &bytes.as_mut()[..10], to_write.as_slice() );
		assert_eq!( &bytes.as_bytes().peek(20)[10..], to_write.as_slice() );

		bytes.write_u8( 5u8 );
		bytes.write_u16( 20u16 );

		assert_eq!( bytes.remaining_len(), 100 - 23 );

		// seek
		bytes.seek_mut( 99 );
		// should now write to the 99 byte // this is the last byte
		bytes.write_u8( 5u8 );
		assert_eq!( bytes.remaining_len(), 0 );
		assert_eq!( bytes.as_mut()[99], 5u8 );

	}

	#[test]
	#[should_panic]
	fn write_overflow() {

		let mut bytes = [0u8; 100];
		let mut bytes = BytesMut::from( &mut bytes[..] );

		// seek
		bytes.seek_mut( 100 );

		bytes.write_u8( 5u8 );
	}

}