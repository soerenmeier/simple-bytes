
macro_rules! read_fn {
	($name:ident, $type:ident, $num:expr) => (
		read_fn!($name, $type, $num, stringify!($type), stringify!($num));
	);
	($name:ident, $type:ident, $num:expr,
	$type_str:expr, $num_str:expr) => {
		#[inline]
		#[doc = "Reads "]
		#[doc = $num_str]
		#[doc = " bytes converting them into an `"]
		#[doc = $type_str]
		#[doc = "`."]
		///
		/// ## Panics
		/// If there aren't enough bytes left.
		fn $name(&mut self) -> $type {
			let mut bytes = [0u8; $num];
			bytes.copy_from_slice(self.read($num));

			$type::from_be_bytes(bytes)
		}
	}
}


/// Read bytes or numbers.
pub trait BytesRead {

	/// Returns the entire slice.
	fn as_slice(&self) -> &[u8];

	/// Returns the length of the entire slice.
	#[inline]
	fn len(&self) -> usize {
		self.as_slice().len()
	}

	/// Returns all remaining bytes.
	fn remaining(&self) -> &[u8];

	/// Reads a given length of bytes.
	/// 
	/// ## Panics
	/// If len exceeds `self.remaining().len()`.
	fn read(&mut self, len: usize) -> &[u8];

	read_fn!(read_u8, u8, 1);
	read_fn!(read_u16, u16, 2);
	read_fn!(read_u32, u32, 4);
	read_fn!(read_u64, u64, 8);
	read_fn!(read_u128, u128, 16);

	read_fn!(read_i8, i8, 1);
	read_fn!(read_i16, i16, 2);
	read_fn!(read_i32, i32, 4);
	read_fn!(read_i64, i64, 8);
	read_fn!(read_i128, i128, 16);

	read_fn!(read_f32, f32, 4);
	read_fn!(read_f64, f64, 8);

	/// Tries to read a given length without updating
	/// the internal position. Returns `None` if there are not enought
	/// bytes remaining.
	fn peek(&self, len: usize) -> Option<&[u8]>;

}