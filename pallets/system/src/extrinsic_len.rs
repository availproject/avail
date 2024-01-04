use avail_core::AppId;
use kate::config::DATA_CHUNK_SIZE;

use bounded_collections::{BoundedBTreeMap, BoundedVec};
use codec::{Compact, CompactLen as _, Decode, Encode, MaxEncodedLen};
use core::mem::swap;
use frame_support::traits::Get;
use scale_info::TypeInfo;
use sp_std::iter::once;
use static_assertions::assert_eq_size_val;

/// TODO
/// - [ ] Deduplicate from `kate` & `kate_recovery` libs.
/// Will be removed soon when https://github.com/availproject/avail-core/tree/ghali/remove-unused-padding is merged
const PADDING_TAIL_VALUE: u8 = 0x80;

#[derive(Debug, Decode, Encode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(S))]
pub struct PaddedExtrinsicLen<S: Get<u32>> {
	tx_lens: BoundedVec<u32, S>,
	num_scalars: u32,
}

impl<S: Get<u32>> PaddedExtrinsicLen<S> {
	/// It adds a new lenght of the Tx and recalculate the total number of scalars.
	pub fn add(&mut self, len: u32) -> Option<u32> {
		// Calculate new data length: each len + its vec prefix
		let next_tx_lens = self.tx_lens.iter().chain(once(&len));
		let data_len = next_tx_lens
			.map(|len| len.checked_add(compact_len(len)?))
			.sum::<Option<u32>>()?;

		// Calculate next encoded len: its data + vec prefix
		let next_tx_count = u32::try_from(self.tx_lens.len()).ok()?.checked_add(1)?;
		let data_vec_prefix = compact_len(&next_tx_count)?;
		let next_encoded_len = data_len.checked_add(data_vec_prefix)?;

		// Add padding tail and calculate next num_scalars.
		// It simulates `ceil( (next_encoded_len +1)/ DATA_CHUNK_SIZE )` using integer division,
		// like `(next_encode_len + 1 /*TAIL*/ + (DATA_CHUNK_SIZE -1)/*ceil*/ ) / DATA_CHUNK_SIZE`
		// NOTE: We check at compile time that `PADDING_TAIL_VALUE` is just one byte.
		assert_eq_size_val!(0u8, PADDING_TAIL_VALUE);
		let data_chunk_size = u32::try_from(DATA_CHUNK_SIZE).ok()?;
		let next_num_scalars = next_encoded_len
			.checked_add(data_chunk_size)?
			.checked_div(data_chunk_size)?;

		// Update state
		self.tx_lens.try_push(len).ok()?;
		self.num_scalars = next_num_scalars;
		Some(next_num_scalars)
	}

	pub fn num_scalars(&self) -> u32 {
		self.num_scalars
	}
}

impl<S: Get<u32>> Default for PaddedExtrinsicLen<S> {
	fn default() -> Self {
		Self {
			tx_lens: BoundedVec::new(),
			num_scalars: 0,
		}
	}
}

#[derive(Debug, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(SID, STX))]
pub struct ExtrinsicLen<SID: Get<u32>, STX: Get<u32>> {
	raw: u32,
	// Track the padded lengths by AppId
	padded: BoundedBTreeMap<AppId, PaddedExtrinsicLen<STX>, SID>,
}

impl<SID: Get<u32>, STX: Get<u32>> ExtrinsicLen<SID, STX> {
	pub fn add_padded(&mut self, id: AppId, len: u32) -> Option<u32> {
		match self.padded.get_mut(&id) {
			Some(padded) => padded.add(len),
			None => {
				let mut padded = PaddedExtrinsicLen::default();
				let num_scalars = padded.add(len)?;
				self.padded.try_insert(id, padded).ok()?;
				Some(num_scalars)
			},
		}
	}

	pub fn raw(&self) -> u32 {
		self.raw
	}

	pub fn add_raw(&mut self, len: u32) -> Option<u32> {
		let mut next_raw = self.raw.checked_add(len)?;
		swap(&mut self.raw, &mut next_raw);
		Some(self.raw)
	}

	pub fn total_num_scalars(&self) -> Option<u32> {
		self.padded
			.values()
			.map(PaddedExtrinsicLen::num_scalars)
			.try_fold(0u32, |acc, num_scalars| acc.checked_add(num_scalars))
	}
}

impl<SID: Get<u32>, STX: Get<u32>> Default for ExtrinsicLen<SID, STX> {
	fn default() -> Self {
		Self {
			raw: <_>::default(),
			padded: BoundedBTreeMap::new(),
		}
	}
}

fn compact_len(value: &u32) -> Option<u32> {
	let len = Compact::<u32>::compact_len(value);
	u32::try_from(len).ok()
}
