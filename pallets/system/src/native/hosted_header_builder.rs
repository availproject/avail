// !!!!
// More info about how runtime interfaces work: (https://docs.rs/sp-runtime-interface/latest/sp_runtime_interface/attr.runtime_interface.html
// !!!!

use crate::{limits::BlockLength, Config};
use avail_base::header_extension::SubmittedData;
use avail_core::{
	header::{extension::fri::FriHeaderVersion, extension::kzg::KzgHeaderVersion, HeaderExtension},
	traits::ExtendedHeader,
	FriParamsVersion,
};
pub use kate::{
	metrics::{IgnoreMetrics, Metrics},
	Seed,
};
use sp_core::H256;
use sp_runtime_interface::runtime_interface;
use sp_std::vec::Vec;

pub const MIN_WIDTH: usize = 4;

/// Trait used by the runtime to build the DA header extension.
///
/// This is a thin abstraction over the host call exposed below via
/// the `HostedHeaderBuilder` runtime interface.
pub trait HeaderExtensionBuilder {
	type Header: ExtendedHeader<Extension = HeaderExtension>;

	/// Build the KZG header extension for the given data.
	fn build_kzg_extension(
		submitted: Vec<SubmittedData>,
		data_root: H256,
		block_length: BlockLength,
		kzg_version: KzgHeaderVersion,
	) -> HeaderExtension;

	/// Build the Fri header extension for the given data.
	fn build_fri_extension(
		submitted: Vec<SubmittedData>,
		data_root: H256,
		params_version: FriParamsVersion,
		fri_version: FriHeaderVersion,
	) -> HeaderExtension;
}

/// Header builder which is actually called by the Avail runtime.
pub mod da {
	use avail_base::header_extension::SubmittedData;
	use avail_core::header::{Header as DaHeader, HeaderExtension};
	use core::marker::PhantomData;
	use sp_runtime::traits::BlakeTwo256;

	use super::*;

	pub type Hash = sp_core::H256;
	pub type BlockNumber = u32;

	/// avail-node Header builder implementation for a given `Config`.
	pub struct HeaderExtensionBuilder<T: Config>(PhantomData<T>);

	impl<T: Config> super::HeaderExtensionBuilder for HeaderExtensionBuilder<T> {
		type Header = DaHeader<BlockNumber, BlakeTwo256>;

		#[inline]
		fn build_kzg_extension(
			submitted: Vec<SubmittedData>,
			data_root: H256,
			block_length: BlockLength,
			kzg_version: KzgHeaderVersion,
		) -> HeaderExtension {
			// Delegate to the host (native) implementation through the
			// runtime interface shim below.
			super::hosted_header_builder::build_kzg_extension(
				submitted,
				data_root,
				block_length,
				kzg_version,
			)
		}

		#[inline]
		fn build_fri_extension(
			submitted: Vec<SubmittedData>,
			data_root: H256,
			params_version: FriParamsVersion,
			fri_version: FriHeaderVersion,
		) -> HeaderExtension {
			super::hosted_header_builder::build_fri_extension(
				submitted,
				data_root,
				params_version,
				fri_version,
			)
		}
	}
}

/// Hosted function to build the header using KZG commitments.
///
/// This is the runtime interface that forwards to the native implementation
/// in `crate::native::build_extension::build_kzg_extension`.
#[runtime_interface]
pub trait HostedHeaderBuilder {
	fn build_kzg_extension(
		submitted: Vec<SubmittedData>,
		data_root: H256,
		block_length: BlockLength,
		kzg_version: KzgHeaderVersion,
	) -> HeaderExtension {
		crate::native::build_extension::build_kzg_extension(
			submitted,
			data_root,
			block_length,
			kzg_version,
		)
	}

	fn build_fri_extension(
		submitted: Vec<SubmittedData>,
		data_root: H256,
		params_version: FriParamsVersion,
		fri_version: FriHeaderVersion,
	) -> HeaderExtension {
		crate::native::build_extension::build_fri_extension(
			submitted,
			data_root,
			params_version,
			fri_version,
		)
	}
}
