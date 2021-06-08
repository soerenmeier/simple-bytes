
/// Sets the internal position for writing or reading.
pub trait BytesSeek {

	/// Sets the internal position.
	fn seek(&mut self, pos: usize);

}