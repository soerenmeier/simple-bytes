


macro_rules! read_fn {
	($name:ident, $type:ident, $num:expr) => (
		#[inline]
		fn $name( &mut self ) -> $type {
			let mut bytes = [0u8; $num];
			bytes.copy_from_slice( self.read( $num ) );

			$type::from_be_bytes( bytes )
		}
	)
}


/// Read bytes or numbers
///
/// ## Panics
/// See safety notes in crate root
///
pub trait BytesRead {

	/// Returns the entire slice
	fn as_slice( &self ) -> &[u8];

	/// Returns the length of the entire slice
	#[inline]
	fn len( &self ) -> usize {
		self.as_slice().len()
	}

	fn remaining( &self ) -> &[u8];

	#[inline]
	fn remaining_len( &self ) -> usize {
		self.remaining().len()
	}

	/// Sets the internal position
	///
	/// # Panics (see crate root)
	/// if pos >= self.len()
	fn seek( &mut self, pos: usize );

	fn read( &mut self, len: usize ) -> &[u8];

	read_fn!( read_u8, u8, 1 );
	read_fn!( read_u16, u16, 2 );
	read_fn!( read_u32, u32, 4 );
	read_fn!( read_u64, u64, 8 );
	read_fn!( read_u128, u128, 16 );

	read_fn!( read_i8, i8, 1 );
	read_fn!( read_i16, i16, 2 );
	read_fn!( read_i32, i32, 4 );
	read_fn!( read_i64, i64, 8 );
	read_fn!( read_i128, i128, 16 );

	read_fn!( read_f32, f32, 4 );
	read_fn!( read_f64, f64, 8 );

	/// Reads the next {len} bytes
	/// without updating the internal position
	fn peek( &self, len: usize ) -> &[u8];

}