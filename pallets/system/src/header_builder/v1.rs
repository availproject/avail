use crate::{limits::BlockLength, header_builder::v2};
use avail_core::{
	app_extrinsic::AppExtrinsic,
	header::HeaderExtension, 
	HeaderVersion,
	SubmittedData,
};
use kate::Seed;

use sp_core::H256;


pub fn build_extension(
	app_extrinsics: &[AppExtrinsic],
	data_root: H256,
	block_length: BlockLength,
	block_number: u32,
	seed: Seed,
) -> HeaderExtension {
	let submitted = app_extrinsics
		.iter()
		.map(|e| SubmittedData::new(e.app_id, e.data.as_slice()))
		.collect::<Vec<_>>();

	v2::build_extension(
		&submitted,
		data_root,
		block_length,
		block_number,
		seed,
		HeaderVersion::V1,
	)
}
