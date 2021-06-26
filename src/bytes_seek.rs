
/// Sets the internal position for writing or reading.
pub trait BytesSeek {

	/// Returns the internal position.
	fn position(&self) -> usize;

	/// Sets the internal position.
	fn seek(&mut self, pos: usize);

	/// Advances the internal position.
	/// 
	/// ## Panic
	/// May panic depending on the `BytesSeek::seek` implementation.
	fn advance(&mut self, adv: usize) {
		self.seek(self.position() + adv);
	}

}