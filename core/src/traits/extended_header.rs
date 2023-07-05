/// Extended header access
pub trait ExtendedHeader<D, E> {
	/// Header number.
	type Number;

	/// Header hash type
	type Hash;

	/// Creates new header.
	fn new(
		number: Self::Number,
		extrinsics_root: Self::Hash,
		state_root: Self::Hash,
		parent_hash: Self::Hash,
		digest: D,
		extension: E,
	) -> Self;

	fn extension(&self) -> &E;

	fn set_extension(&mut self, extension: E);
}
