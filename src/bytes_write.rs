
use crate::Bytes;


macro_rules! write_fn {
	($name:ident, $type:ident) => (
		write_fn!($name, $type, stringify!($type));
	);
	($name:ident, $type:ident, $type_str:expr) => {
		#[inline]
		#[doc = "Writes an `"]
		#[doc = $type_str]
		#[doc = "`."]
		/// 
		/// ## Panics
		/// If there aren't enough remaining bytes left.
		fn $name(&mut self, num: $type) {
			let bytes = num.to_be_bytes();
			self.write(&bytes);
		}
	}
}

/// Write bytes or numbers.
pub trait BytesWrite {

	/// Returns the entire slice mutably.
	fn as_mut(&mut self) -> &mut [u8];

	/// Returns the entire slice as a bytes struct
	/// setting the position of the new Bytes to `0`.
	fn as_bytes(&self) -> Bytes<'_>;

	/// Returns the remaining bytes mutably.
	fn remaining_mut(&mut self) -> &mut [u8];

	/// Writes a slice.
	/// 
	/// ## Panics
	/// If there aren't enough remaining bytes left.
	fn write(&mut self, slice: impl AsRef<[u8]>);

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