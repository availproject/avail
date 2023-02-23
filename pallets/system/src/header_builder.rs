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

#[cfg(feature = "std")]
pub(crate) mod metrics {
	use avail_base::metrics::AVAIL_METRICS;
	use kate::BlockDimensions;
	use sp_std::time::Duration;

	use super::*;

	/// Helper macro to implement methods from `MetricAdapter`.
	macro_rules! may_observe {
		($($metric:tt, $observation:expr),+) => {
			if let Some(avail) = AVAIL_METRICS.get() {
				observe!(avail, $($metric, $observation),+)
			}
		};
	}

	/// Variadic macro to support multiple pairs of `metric` and `observation`
	macro_rules! observe {
		($avail: tt, $metric:tt, $observation: expr) => {
			$avail.$metric.observe($observation as f64)
		};

		($avail: tt, $metric:tt, $observation: expr, $($m:tt, $o: expr),+) => {{
			$avail.$metric.observe($observation as f64);
			observe!($avail, $($m, $o),+)
		}};
	}

	/// Adapter which implements `Metrics` using `AVAIL_METRIC` signleton.
	pub(crate) struct MetricAdapter {}

	impl Metrics for MetricAdapter {
		fn extended_block_time(&self, elapsed: Duration) {
			may_observe!(kate_extend_block_time, elapsed.as_micros());
		}

		fn preparation_block_time(&self, elapsed: Duration) {
			may_observe!(kate_preparation_block_time, elapsed.as_micros());
		}

		fn commitment_build_time(&self, elapsed: Duration) {
			may_observe!(kate_commitment_time, elapsed.as_micros());
		}

		fn proof_build_time(&self, elapsed: Duration, cells: u32) {
			may_observe!(
				kate_proof_build_time,
				elapsed.as_micros(),
				kate_proof_cells,
				cells
			);
		}

		fn block_dims_and_size(&self, block_dims: &BlockDimensions, block_len: u32) {
			may_observe!(
				block_dims_rows,
				block_dims.rows.0,
				block_dims_cols,
				block_dims.cols.0,
				block_len,
				block_len
			);
		}
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

	let (xts_layout, commitment, block_dims, _data_matrix) = kate::com::par_build_commitments(
		block_length.rows,
		block_length.cols,
		block_length.chunk_size(),
		app_extrinsics,
		seed,
		metrics,
	)
	.expect("Build commitments cannot fail .qed");
	let app_lookup =
		DataLookup::try_from(xts_layout.as_slice()).expect("Extrinsic size cannot overflow .qed");

	let commitment = KateCommitment {
		rows: block_dims.rows.0.saturated_into::<u16>(),
		cols: block_dims.cols.0.saturated_into::<u16>(),
		commitment,
		data_root,
	};

	HeaderExtension::V1(v1::HeaderExtension {
		commitment,
		app_lookup,
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
		let metrics = metrics::MetricAdapter {};
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
