use sp_runtime::traits::Member;

pub trait ExtrinsicsRoot:
	Member + MaybeSerializeDeserialize + Default + Debug + Codec + MaybeMallocSizeOf
{
	type HashOutput: Member
		+ MaybeSerializeDeserialize
		+ Debug
		+ sp_std::hash::Hash
		+ Ord
		+ Copy
		+ MaybeDisplay
		+ Default
		+ SimpleBitOps
		+ Codec
		+ AsRef<[u8]>
		+ AsMut<[u8]>
		+ MaybeMallocSizeOf;

	fn hash(&self) -> &Self::HashOutput;
	fn commitment(&self) -> &Vec<u8>;

	fn new(hash: Self::HashOutput) -> Self;

	fn new_with_commitment(
		hash: Self::HashOutput,
		commitment: Vec<u8>,
		rows: u16,
		cols: u16,
	) -> Self;
}
