use avail_core::{header::HeaderExtension, traits::ExtendedHeader, AppExtrinsic, HeaderVersion};
use frame_support::traits::Randomness;
#[cfg(feature = "std")]
use kate::gridgen::EvaluationGrid;
pub use kate::{
	metrics::{IgnoreMetrics, Metrics},
	Seed,
};
use sp_core::H256;
#[cfg(feature = "std")]
use sp_runtime::SaturatedConversion;
use sp_runtime::{generic::Digest, traits::Hash};
use sp_runtime_interface::runtime_interface;
use sp_std::vec::Vec;

use crate::{limits::BlockLength, Config, LOG_TARGET};

pub mod da {
	use core::marker::PhantomData;

	use avail_core::header::{Header as DaHeader, HeaderExtension};
	use sp_runtime::traits::BlakeTwo256;

	use super::*;

	pub type Hash = sp_core::H256;
	pub type BlockNumber = u32;

	/// avail-node Header builder.
	pub struct HeaderExtensionBuilder<T: Config>(PhantomData<T>);

	impl<T: Config> super::HeaderExtensionBuilder for HeaderExtensionBuilder<T> {
		type Header = DaHeader<BlockNumber, BlakeTwo256>;

		#[inline]
		fn build(
			app_extrinsics: Vec<AppExtrinsic>,
			data_root: H256,
			block_length: BlockLength,
			block_number: u32,
			version: HeaderVersion,
		) -> HeaderExtension {
			let seed = Self::random_seed::<T>();

			super::hosted_header_builder::build(
				app_extrinsics,
				data_root,
				block_length,
				block_number,
				seed,
				version,
			)
		}
	}
}

/// Trait for header builder.
pub trait HeaderExtensionBuilder {
	type Header: sp_runtime::traits::Header + ExtendedHeader<u32, H256, Digest, HeaderExtension>;

	/// Creates the header using the given parameters.
	fn build(
		app_extrinsics: Vec<AppExtrinsic>,
		data_root: H256,
		block_length: BlockLength,
		block_number: u32,
		version: HeaderVersion,
	) -> HeaderExtension;

	/// Generates a random seed using the _epoch seed_ and the _current block_ returned by
	/// `T::Randomness` type.
	fn random_seed<T: Config>() -> Seed {
		let seed = if cfg!(feature = "secure_padding_fill") {
			let (epoch_seed, block_number) = <T as Config>::Randomness::random_seed();
			let seed = <T as Config>::Hashing::hash_of(&(&epoch_seed, &block_number));
			log::trace!(
				target: LOG_TARGET,
				"Header builder seed {seed:?} from epoch seed {epoch_seed:?} and block {block_number:?}");
			seed
		} else {
			<T as Config>::Hash::default()
		};

		seed.into()
	}
}

#[allow(unused)]
#[cfg(feature = "header_commitment_corruption")]
fn corrupt_commitment(block_number: u32, commitment: &mut Vec<u8>) {
	if let Some(ref_byte) = commitment.get_mut(0) {
		log::trace!(
			target: LOG_TARGET,
			"Block {block_number}, corrupting 1st byte of commitment from {ref_byte:x} to {:x}",
			*ref_byte ^ 0xffu8
		);

		*ref_byte ^= 0xffu8;
	} else {
		log::trace!(
			target: LOG_TARGET,
			"Block {block_number}, corrupting commitment by adding one `0xFF` byte "
		);
		commitment.push(0xffu8)
	}
}

#[cfg(feature = "std")]
pub fn build_grid(
	app_extrinsics: &[AppExtrinsic],
	block_length: BlockLength,
	seed: Seed,
) -> Result<EvaluationGrid, String> {
	const MIN_WIDTH: usize = 4;
	let grid = EvaluationGrid::from_extrinsics(
		app_extrinsics.to_vec(),
		MIN_WIDTH,
		block_length.cols.0.saturated_into(), // even if we run on a u16 target this is fine
		block_length.rows.0.saturated_into(),
		seed,
	)
	.map_err(|e| format!("Grid construction failed: {e:?}"))?;

	Ok(grid)
}

#[cfg(feature = "std")]
pub fn build_commitment(grid: &EvaluationGrid) -> Result<Vec<u8>, String> {
	use kate::gridgen::AsBytes;
	use once_cell::sync::Lazy;

	// couscous has pp for degree upto 1024
	static PMP: Lazy<kate::pmp::m1_blst::M1NoPrecomp> =
		Lazy::new(kate::couscous::multiproof_params);

	let poly_grid = grid
		.make_polynomial_grid()
		.map_err(|e| format!("Make polynomial grid failed: {e:?}"))?;

	let extended_grid = poly_grid
		.extended_commitments(&*PMP, 2)
		.map_err(|e| format!("Grid extension failed: {e:?}"))?;

	let mut commitment = Vec::new();
	for c in extended_grid.iter() {
		match c.to_bytes() {
			Ok(bytes) => commitment.extend(bytes),
			Err(e) => return Err(format!("Commitment serialization failed: {:?}", e)),
		}
	}

	Ok(commitment)
}

#[cfg(feature = "std")]
pub fn get_empty_header(data_root: H256, version: HeaderVersion) -> HeaderExtension {
	use avail_core::{header::extension::v3, DataLookup};
	let empty_commitment: Vec<u8> = vec![0];
	let empty_app_lookup = DataLookup::new_empty();

	match version {
		HeaderVersion::V3 => {
			use avail_core::kate_commitment::v3::KateCommitment;
			let kate = KateCommitment::new(1, 4, data_root, empty_commitment);
			v3::HeaderExtension {
				app_lookup: empty_app_lookup,
				commitment: kate,
			}
			.into()
		},
	}
}

#[cfg(feature = "std")]
pub fn build_extension(
	app_extrinsics: &[AppExtrinsic],
	data_root: H256,
	block_length: BlockLength,
	_block_number: u32,
	seed: Seed,
	version: HeaderVersion,
) -> HeaderExtension {
	use avail_base::metrics::avail::HeaderExtensionBuilderMetrics;
	use avail_core::header::extension::v3;

	let build_extension_start = std::time::Instant::now();

	// Build the grid
	let timer = std::time::Instant::now();
	let maybe_grid = build_grid(app_extrinsics, block_length, seed);
	// Evaluation Grid Build Time Metrics
	HeaderExtensionBuilderMetrics::observe_evaluation_grid_build_time(timer.elapsed());
	let grid = match maybe_grid {
		Ok(res) => res,
		Err(message) => {
			log::error!("NODE_CRITICAL_ERROR_001 - A critical error has occurred: {message:?}.");
			log::error!("NODE_CRITICAL_ERROR_001 - If you see this, please warn Avail team and raise an issue.");
			HeaderExtensionBuilderMetrics::observe_total_execution_time(
				build_extension_start.elapsed(),
			);
			return get_empty_header(data_root, version);
		},
	};

	// Build the commitment
	let timer = std::time::Instant::now();
	let maybe_commitment = build_commitment(&grid);
	// Commitment Build Time Metrics
	HeaderExtensionBuilderMetrics::observe_commitment_build_time(timer.elapsed());
	let commitment = match maybe_commitment {
		Ok(res) => res,
		Err(message) => {
			log::error!("NODE_CRITICAL_ERROR_002 - A critical error has occurred: {message:?}.");
			log::error!("NODE_CRITICAL_ERROR_002 - If you see this, please warn Avail team and raise an issue.");
			HeaderExtensionBuilderMetrics::observe_total_execution_time(
				build_extension_start.elapsed(),
			);
			return get_empty_header(data_root, version);
		},
	};

	// Note that this uses the original dims, _not the extended ones_
	let rows = grid.dims().rows().get();
	let cols = grid.dims().cols().get();

	// Grid Metrics
	HeaderExtensionBuilderMetrics::observe_grid_rows(rows as f64);
	HeaderExtensionBuilderMetrics::observe_grid_cols(cols as f64);

	let app_lookup = grid.lookup().clone();

	let header_extension = match version {
		HeaderVersion::V3 => {
			use avail_core::kate_commitment::v3::KateCommitment;
			let kate = KateCommitment::new(rows, cols, data_root, commitment);
			v3::HeaderExtension {
				app_lookup,
				commitment: kate,
			}
			.into()
		},
	};

	// Total Execution Time Metrics
	HeaderExtensionBuilderMetrics::observe_total_execution_time(build_extension_start.elapsed());

	header_extension
}

/// Hosted function to build the header using `kate` commitments.
#[runtime_interface]
pub trait HostedHeaderBuilder {
	fn build(
		app_extrinsics: Vec<AppExtrinsic>,
		data_root: H256,
		block_length: BlockLength,
		block_number: u32,
		seed: Seed,
		version: HeaderVersion,
	) -> HeaderExtension {
		build_extension(
			&app_extrinsics,
			data_root,
			block_length,
			block_number,
			seed,
			version,
		)
	}
}
