
use crate::Bytes;


macro_rules! write_fn {
	($name:ident, $type:ident) => (
		#[inline]
		fn $name(&mut self, num: $type) {
			let bytes = num.to_be_bytes();
			self.write(&bytes);
		}
	)
}

/// Write bytes or numbers
///
/// ## Panics
/// See safety notes in crate root
///
pub trait BytesWrite {

	/// returns the entire slice as mut
	fn as_mut(&mut self) -> &mut [u8];

	/// returns the entire slice as a bytes type with a position of 0
	fn as_bytes(&self) -> Bytes<'_>;

	fn remaining_mut(&mut self) -> &mut [u8];

	/// Sets the internal position
	///
	/// called seek_mut and not seek to cause no confusion between BytesRead::seek
	/// and this implementation
	///
	/// # Panics
	/// if pos >= self.len()
	/// this is implementation dependent, BytesOwned never panics
	fn seek_mut(&mut self, pos: usize);

	fn write(&mut self, slice: &[u8]);

	write_fn!(write_u8, u8);
	write_fn!(write_u16, u16);
	write_fn!(write_u32, u32);
	write_fn!(write_u64, u64);
	write_fn!(write_u128, u128);

	write_fn!(write_i8, i8);
	write_fn!(write_i16, i16);
	write_fn!(write_i32, i32);
	write_fn!(write_i64, i64);
	write_fn!(write_i128, i128);

	write_fn!(write_f32, f32);
	write_fn!(write_f64, f64);

}