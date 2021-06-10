
/// Sets the internal position for writing or reading.
pub trait BytesSeek {

	/// Returns the internal position.
	fn position(&self) -> usize;

	/// Sets the internal position.
	fn seek(&mut self, pos: usize);

}