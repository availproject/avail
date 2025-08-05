use super::{native::hosted_kate, Error, GCellBlock, GDataProof, GMultiProof, GRow};
use avail_base::header_extension::SubmittedData;
use avail_core::AppExtrinsic;
use da_control::LOG_TARGET as DALOG_TARGET;

use frame_system::{limits::BlockLength, Config as SystemConfig};
use kate::Seed;

use frame_support::traits::Randomness;
use sp_runtime::traits::Hash;
use sp_std::vec::Vec;

fn random_seed<T: SystemConfig>() -> Seed {
	let seed = if cfg!(feature = "secure_padding_fill") {
		let (epoch_seed, block_number) = T::Randomness::random_seed();
		let seed = T::Hashing::hash_of(&(&epoch_seed, &block_number));
		log::trace!(
            target: DALOG_TARGET,
            "RTKate seed {seed:?} from epoch seed {epoch_seed:?} and block {block_number:?}");
		seed
	} else {
		T::Hash::default()
	};

	seed.into()
}

pub fn grid<T: SystemConfig>(
	app_extrinsics: Vec<AppExtrinsic>,
	block_length: BlockLength,
	selected_rows: Vec<u32>,
) -> Result<Vec<GRow>, Error> {
	let seed = random_seed::<T>();
	hosted_kate::grid(app_extrinsics, block_length, seed, selected_rows)
}

pub fn proof<T: SystemConfig>(
	app_extrinsics: Vec<AppExtrinsic>,
	block_len: BlockLength,
	cells: Vec<(u32, u32)>,
) -> Result<Vec<GDataProof>, Error> {
	let seed = random_seed::<T>();
	hosted_kate::proof(app_extrinsics, block_len, seed, cells)
}

pub fn grid_v4<T: SystemConfig>(
	app_extrinsics: Vec<SubmittedData>,
	block_length: BlockLength,
	selected_rows: Vec<u32>,
) -> Result<Vec<GRow>, Error> {
	let seed = random_seed::<T>();
	hosted_kate::grid_v4(app_extrinsics, block_length, seed, selected_rows)
}

pub fn proof_v4<T: SystemConfig>(
	da_extrinsics: Vec<SubmittedData>,
	block_len: BlockLength,
	cells: Vec<(u32, u32)>,
) -> Result<Vec<GDataProof>, Error> {
	let seed = random_seed::<T>();
	hosted_kate::proof_v4(da_extrinsics, block_len, seed, cells)
}

pub fn multiproof<T: SystemConfig>(
	app_extrinsics: Vec<AppExtrinsic>,
	block_len: BlockLength,
	cells: Vec<(u32, u32)>,
) -> Result<Vec<(GMultiProof, GCellBlock)>, Error> {
	let seed = random_seed::<T>();
	hosted_kate::multiproof(app_extrinsics, block_len, seed, cells)
}
