// !!!!
// More info about how runtime interfaces work: (https://docs.rs/sp-runtime-interface/latest/sp_runtime_interface/attr.runtime_interface.html
// !!!!

use crate::{limits::BlockLength, Config, LOG_TARGET};
use avail_base::header_extension::SubmittedData;
use avail_core::HeaderVersion;
use avail_core::{header::HeaderExtension, traits::ExtendedHeader};
pub use kate::{
	metrics::{IgnoreMetrics, Metrics},
	Seed,
};

use frame_support::traits::Randomness;
use sp_core::H256;
use sp_runtime::traits::Hash;
use sp_runtime_interface::{
	pass_by::{AllocateAndReturnByCodec, PassFatPointerAndDecode},
	runtime_interface,
};
use sp_std::vec::Vec;

pub const MIN_WIDTH: usize = 4;

pub mod da {
	use core::marker::PhantomData;

	use avail_base::header_extension::SubmittedData;
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
		fn build_extension(
			submitted: Vec<SubmittedData>,
			data_root: H256,
			block_length: BlockLength,
			version: HeaderVersion,
		) -> HeaderExtension {
			super::hosted_header_builder::build_extension(
				submitted,
				data_root,
				block_length,
				version,
			)
		}
	}
}

/// Trait for header builder.
pub trait HeaderExtensionBuilder {
	type Header: ExtendedHeader<Extension = HeaderExtension>;

	/// Creates the header using the given parameters.
	fn build_extension(
		app_extrinsics: Vec<SubmittedData>,
		data_root: H256,
		block_length: BlockLength,
		version: HeaderVersion,
	) -> HeaderExtension;

	/// Generates a random seed using the _epoch seed_ and the _current block_ returned by
	/// `T::Randomness` type.
	fn random_seed<T: Config>() -> Seed {
		<T as Config>::Hash::default().into()
	}
}

/// Hosted function to build the header using `kate` commitments.
#[runtime_interface]
pub trait HostedHeaderBuilder {
	fn build_extension(
		submitted: PassFatPointerAndDecode<Vec<SubmittedData>>,
		data_root: PassFatPointerAndDecode<H256>,
		block_length: PassFatPointerAndDecode<BlockLength>,
		version: PassFatPointerAndDecode<HeaderVersion>,
	) -> AllocateAndReturnByCodec<HeaderExtension> {
		#[cfg(feature = "std")]
		{
			return crate::native::build_extension_v2::build_extension_v4(
				submitted.to_vec(),
				data_root,
				block_length,
				version,
			);
		}

		#[cfg(not(feature = "std"))]
		{
			return HeaderExtension::get_faulty_header(data_root, version);
		}
	}
}
