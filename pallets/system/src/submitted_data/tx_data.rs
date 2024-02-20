use avail_core::data_proof_v2::AddressedMessage;
use derive_more::Constructor;
use sp_std::vec::Vec;

#[derive(Debug, Clone, Default, Constructor)]
pub struct TxData {
	pub submitted: Vec<Vec<u8>>,
	pub bridged: Vec<AddressedMessage>,
}

impl FromIterator<TxData> for TxData {
	fn from_iter<I: IntoIterator<Item = TxData>>(iter: I) -> Self {
		let (submitted, bridge): (Vec<_>, Vec<_>) = iter
			.into_iter()
			.map(|tx| (tx.submitted, tx.bridged))
			.unzip();
		let submitted = submitted.into_iter().flatten().collect::<Vec<_>>();
		let bridge = bridge.into_iter().flatten().collect::<Vec<_>>();
		Self::new(submitted, bridge)
	}
}
