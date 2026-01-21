// use super::{native::hosted_kate, Error, GCellBlock, GDataProof, GMultiProof, GRow};
// use da_control::LOG_TARGET as DALOG_TARGET;

// use avail_core::AppExtrinsic;
// use frame_system::{limits::BlockLength, Config as SystemConfig};
// use kate::Seed;

// use frame_support::traits::Randomness;
// use sp_runtime::traits::Hash;
// use sp_std::vec::Vec;

// fn random_seed<T: SystemConfig>() -> Seed {
// 	T::Hash::default().into()
// }

// pub fn grid<T: SystemConfig>(
// 	app_extrinsics: Vec<AppExtrinsic>,
// 	block_length: BlockLength,
// 	selected_rows: Vec<u32>,
// ) -> Result<Vec<GRow>, Error> {
// 	let seed = random_seed::<T>();
// 	hosted_kate::grid(app_extrinsics, block_length, seed, selected_rows)
// }

// pub fn proof<T: SystemConfig>(
// 	app_extrinsics: Vec<AppExtrinsic>,
// 	block_len: BlockLength,
// 	cells: Vec<(u32, u32)>,
// ) -> Result<Vec<GDataProof>, Error> {
// 	let seed = random_seed::<T>();
// 	hosted_kate::proof(app_extrinsics, block_len, seed, cells)
// }

// pub fn multiproof<T: SystemConfig>(
// 	app_extrinsics: Vec<AppExtrinsic>,
// 	block_len: BlockLength,
// 	cells: Vec<(u32, u32)>,
// ) -> Result<Vec<(GMultiProof, GCellBlock)>, Error> {
// 	let seed = random_seed::<T>();
// 	hosted_kate::multiproof(app_extrinsics, block_len, seed, cells)
// }
