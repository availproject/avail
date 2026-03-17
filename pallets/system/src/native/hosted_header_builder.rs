//! Runtime interface and abstraction for building DA header extensions.
//!
//! More info about runtime interfaces:
//! https://docs.rs/sp-runtime-interface/latest/sp_runtime_interface/attr.runtime_interface.html

use crate::{limits::BlockLength, Config};
use avail_base::header_extension::SubmittedData;
use avail_core::{
	header::{
		extension::{fri::FriHeaderVersion, kzg::KzgHeaderVersion},
		ExtendedHeader, HeaderExtension,
	},
	FriParamsVersion,
};

pub use kate::{
	metrics::{IgnoreMetrics, Metrics},
	Seed,
};

use sp_core::H256;
use sp_runtime_interface::{
	pass_by::{AllocateAndReturnByCodec, PassFatPointerAndDecode},
	runtime_interface,
};
use sp_std::vec::Vec;

pub const MIN_WIDTH: usize = 4;

/// Trait used by the runtime to build the DA header extension.
///
/// This is a thin abstraction over the host call exposed via
/// the `HostedHeaderBuilder` runtime interface.
pub trait HeaderExtensionBuilder {
	type Header: ExtendedHeader<Extension = HeaderExtension>;

	/// Build the KZG header extension.
	fn build_kzg_extension(
		submitted: Vec<SubmittedData>,
		data_root: H256,
		block_length: BlockLength,
		kzg_version: KzgHeaderVersion,
	) -> HeaderExtension;

	/// Build the FRI header extension.
	fn build_fri_extension(
		submitted: Vec<SubmittedData>,
		data_root: H256,
		params_version: FriParamsVersion,
		fri_version: FriHeaderVersion,
	) -> HeaderExtension;
}

/// Runtime-side DA header builder.
pub mod da {
	use super::*;
	use avail_core::header::runtime::Header as DaHeader;
	use core::marker::PhantomData;
	use sp_runtime::traits::BlakeTwo256;

	pub type BlockNumber = u32;

	/// Runtime-facing header extension builder.
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
			super::hosted_header_builder::build_kzg_extension(
				submitted,
				data_root,
				block_length,
				kzg_version,
			)
			.into()
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
			.into()
		}
	}
}

/// Runtime interface forwarding header construction to the host.
///
/// The actual implementation lives in `crate::native::build_extension`.
#[runtime_interface]
pub trait HostedHeaderBuilder {
	fn build_kzg_extension(
		submitted: PassFatPointerAndDecode<Vec<SubmittedData>>,
		data_root: PassFatPointerAndDecode<H256>,
		block_length: PassFatPointerAndDecode<BlockLength>,
		kzg_version: PassFatPointerAndDecode<KzgHeaderVersion>,
	) -> AllocateAndReturnByCodec<HeaderExtension> {
		crate::native::build_extension::build_kzg_extension(
			submitted.to_vec(),
			data_root,
			block_length,
			kzg_version,
		)
	}

	fn build_fri_extension(
		submitted: PassFatPointerAndDecode<Vec<SubmittedData>>,
		data_root: PassFatPointerAndDecode<H256>,
		params_version: PassFatPointerAndDecode<FriParamsVersion>,
		fri_version: PassFatPointerAndDecode<FriHeaderVersion>,
	) -> AllocateAndReturnByCodec<HeaderExtension> {
		crate::native::build_extension::build_fri_extension(
			submitted.to_vec(),
			data_root,
			params_version,
			fri_version,
		)
	}
}
