use crate::{migrations::v1, AllExtrinsicsLen, Config, ExtrinsicLenOf};

use frame_support::weights::Weight;
use sp_core::Get;

pub fn migrate<T: Config>() -> Weight {
	// Update extrinsic length.
	let _ = <AllExtrinsicsLen<T>>::translate(
		|maybe_len: Option<v1::ExtrinsicLen>| -> Option<ExtrinsicLenOf<T>> {
			maybe_len.map(|_| ExtrinsicLenOf::<T>::default())
		},
	);

	T::DbWeight::get().reads_writes(1, 1)
}
