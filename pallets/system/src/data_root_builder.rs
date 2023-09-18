use codec::Decode;
use avail_core::AppExtrinsic;
use frame_support::traits::ExtrinsicCall;
use sp_core::H256;
use sp_runtime::traits::Extrinsic;
use sp_std::vec::Vec;

const LOG_TARGET: &str = "runtime::system::data_root_builder";

pub type DRFOutput = Option<Vec<Vec<u8>>>;
pub type DRFCallOf<T> = <T as Extrinsic>::Call;

pub trait DataRootFilter {
	type UncheckedExtrinsic: Decode + ExtrinsicCall;

	fn filter(call: &DRFCallOf<Self::UncheckedExtrinsic>) -> DRFOutput;
}

pub trait DataRootBuilder<F: DataRootFilter> {
	fn build<'a, I>(app_extrinsics: I) -> H256
	where
		I: IntoIterator<Item = &'a AppExtrinsic>,
	{
		let mut used_extrinsics_count = 0u32;
		let mut total_extrinsics_count = 0u32;

		let filtered_iter = app_extrinsics
			.into_iter()
			.enumerate()
			.filter_map(|(idx, app_extrinsic)| {
				// Decode call and log any failure.
				total_extrinsics_count += 1;
				match F::UncheckedExtrinsic::decode(&mut app_extrinsic.data.as_slice()) {
					Ok(app_unchecked_extrinsic) => Some(app_unchecked_extrinsic),
					Err(err) => {
						// NOTE: Decodification issue is like a `unrecheable` because this
						// extrinsic was decoded previously, when node executed the block.
						// We will keep this here just to have a trace if we update
						// `System::note_extrinsic` in the future.
						log::error!(
							target: LOG_TARGET,
							"Extrinsic {} cannot be decoded: {:?}",
							idx,
							err
						);
						None
					},
				}
			})
			.filter_map(|app_unchecked_extrinsic| {
				// Filter calls and traces removed calls
				// @TODO: We could avoid the the copy of data from filter
				// once `binary_merkle_tree::merkelize` becomes public. In that case,
				// we could work with iterator and lifescopes, having something like:
				//
				// ```Rust
				// pub trait DataRootFilter {
				//	type UncheckedExtrinsic: Decode + ExtrinsicCall;
				//	fn filter<'a>(call: &'a <Self::UncheckedExtrinsic as Extrinsic>::Call) -> Option<&'a [u8]>;
				//	}
				// ```
				let maybe_data = F::filter(app_unchecked_extrinsic.call());
				if maybe_data.is_some() {
					used_extrinsics_count += 1;
				}

				maybe_data
			});

		let root = Self::merkle_root(filtered_iter);

		log::debug!(
			target: LOG_TARGET,
			"Used {} extrinsics of {}",
			used_extrinsics_count,
			total_extrinsics_count
		);

		root.into()
	}

	fn merkle_root<I>(leaves: I) -> H256
	where
		I: Iterator<Item = Vec<u8>>,
	{
		use binary_merkle_tree::{merkle_root, Hash, Hasher};
		use sp_io::hashing::sha2_256;

		#[derive(Copy, Clone)]
		struct Sha2_256 {}

		impl Hasher for Sha2_256 {
			fn hash(data: &[u8]) -> Hash { sha2_256(data) }
		}

		merkle_root::<Sha2_256, _, _>(leaves).into()
	}
}

impl<F: DataRootFilter> DataRootBuilder<F> for F {}