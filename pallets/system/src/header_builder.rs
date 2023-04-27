use da_primitives::{asdr::AppExtrinsic, traits::ExtendedHeader, HeaderExtension};
#[cfg(feature = "std")]
use da_primitives::{asdr::DataLookup, KateCommitment};
use frame_support::traits::Randomness;
pub use kate::{
	metrics::{IgnoreMetrics, Metrics},
	Seed,
};
use sp_core::H256;
use sp_runtime::traits::Hash;
#[cfg(feature = "std")]
use sp_runtime::SaturatedConversion;
use sp_runtime_interface::runtime_interface;
use sp_std::vec::Vec;

use crate::{limits::BlockLength, Config, LOG_TARGET};

pub mod da {
	use core::marker::PhantomData;

	use da_primitives::Header as DaHeader;
	use sp_runtime::traits::BlakeTwo256;

	use super::{AppExtrinsic, BlockLength, Config, HeaderExtension, Vec, H256};

	pub type Hash = sp_core::H256;
	// @todo Miguel: Link this type with `Config::BlockNumber`.
	pub type BlockNumber = u32;

	/// Data-Avail Header builder.
	pub struct HeaderExtensionBuilder<T: Config>(PhantomData<T>);

	impl<T: Config> super::HeaderExtensionBuilder for HeaderExtensionBuilder<T> {
		type Header = DaHeader<BlockNumber, BlakeTwo256>;

		#[inline]
		fn build(
			app_extrinsics: Vec<AppExtrinsic>,
			data_root: H256,
			block_length: BlockLength,
		) -> HeaderExtension {
			let seed = Self::random_seed::<T>();

			let unused_block_number = 0u32;
			super::hosted_header_builder::build(
				app_extrinsics,
				data_root,
				block_length,
				unused_block_number,
				seed,
			)
		}
	}
}

/// Trait for header builder.
pub trait HeaderExtensionBuilder {
	type Header: sp_runtime::traits::Header + ExtendedHeader;

	/// Creates the header using the given parameters.
	fn build(
		app_extrinsics: Vec<AppExtrinsic>,
		data_root: H256,
		block_length: BlockLength,
	) -> HeaderExtension;

	/// Generates a random seed using the _epoch seed_ and the _current block_ returned by
	/// `T::Randomness` type.
	fn random_seed<T: Config>() -> Seed {
		let (epoch_seed, block_number) = <T as Config>::Randomness::random_seed();
		let seed = <T as Config>::Hashing::hash_of(&(&epoch_seed, &block_number));

		log::trace!(
			target: LOG_TARGET,
			"Header builder seed {:?} from epoch seed {:?} and block {:?}",
			seed,
			epoch_seed,
			block_number
		);

		seed.into()
	}
}

#[allow(dead_code)]
#[cfg(all(feature = "std", feature = "header-compatibility-test"))]
fn build_extension_v_test(
	app_extrinsics: &[AppExtrinsic],
	data_root: H256,
	block_length: BlockLength,
	seed: Seed,
) -> HeaderExtension {
	let extension_v1 = build_extension_v1(app_extrinsics, data_root, block_length, seed);
	match extension_v1 {
		HeaderExtension::V1(extension) => HeaderExtension::VTest(extension.into()),
		r @ HeaderExtension::VTest(_) => r,
	}
}

#[cfg(feature = "std")]
fn build_extension<M: Metrics>(
	app_extrinsics: &[AppExtrinsic],
	data_root: H256,
	block_length: BlockLength,
	seed: Seed,
	metrics: &M,
) -> HeaderExtension {
	use da_primitives::header::extension::v1;
	use kate::Serializable;
	use once_cell::sync::Lazy;
	static PMP: Lazy<kate::pmp::m1_blst::M1NoPrecomp> =
		once_cell::sync::Lazy::new(|| kate::testnet::multiproof_params(256, 256));

	let grid = kate::gridgen::EvaluationGrid::from_extrinsics(
		app_extrinsics.to_vec(),
		4, //TODO: where should this minimum grid width be specified
		block_length.cols.as_usize(),
		block_length.rows.as_usize(),
		seed,
	)
	.expect("Grid construction cannot fail");

	let commitment = grid
		.make_polynomial_grid()
		.expect("Make polynomials cannot fail")
		.extended_commitments(&*PMP, 2)
		.expect("Extended commitments cannot fail")
		.iter()
		.flat_map(|c| c.0.to_bytes())
		.collect::<Vec<u8>>();

    // We must put the un-extended dimensions into this commitment object!
	let commitment = KateCommitment {
		rows: grid.dims.height().saturated_into::<u16>(),
		cols: grid.dims.width().saturated_into::<u16>(),
		commitment,
		data_root,
	};

	HeaderExtension::V1(v1::HeaderExtension {
		commitment,
		app_lookup: grid.lookup,
	})
}

/// Hosted function to build the header using `kate` commitments.
#[runtime_interface]
pub trait HostedHeaderBuilder {
	/// Creates the header using the given parameters.
	/// *NOTE:* Version 1 uses `dusk-plonk v0.8.2`
	#[version(1)]
	fn build(
		app_extrinsics: Vec<AppExtrinsic>,
		data_root: H256,
		block_length: BlockLength,
		_block_number: u32,
		seed: Seed,
	) -> HeaderExtension {
		let metrics = avail_base::metrics::MetricAdapter {};
		build_extension(&app_extrinsics, data_root, block_length, seed, &metrics)
	}

	/*
	// @TODO Miguel: Substrate v0.9.29 supports deactivated new version of hosted functions.
	// NOTE: It is just for testing the forward compatibility in header extension.
	#[cfg(feature = "header-compatibility-test")]
	#[version(2)]
	fn build(
		app_extrinsics: Vec<AppExtrinsic>,
		data_root: H256,
		block_length: BlockLength,
		block_number: u32,
		seed: Seed,
	) -> HeaderExtension {
		// Genesis HAS TO use the legacy header extension
		let build_extension_fn = if block_number > 1 {
			build_extension_v_test
		} else {
			build_extension_v1
		};

		build_extension_fn(app_extrinsics.as_slice(), data_root, block_length, seed)
	}*/
}
