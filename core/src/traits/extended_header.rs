/// Extended header access
pub trait ExtendedHeader<Number, Hash, Digest, Extension> {
	/// Creates new header.
	fn new(
		number: Number,
		extrinsics_root: Hash,
		state_root: Hash,
		parent_hash: Hash,
		digest: Digest,
		extension: Extension,
	) -> Self;

	fn extension(&self) -> &Extension;

	fn set_extension(&mut self, extension: Extension);
}
