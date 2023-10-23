#[allow(clippy::all)]
#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
pub mod api {
	#[allow(unused_imports)]
	mod root_mod {
		pub use super::*;
	}
	pub static PALLETS: [&str; 34usize] = [
		"System",
		"Utility",
		"Babe",
		"Timestamp",
		"Authorship",
		"Indices",
		"Balances",
		"TransactionPayment",
		"ElectionProviderMultiPhase",
		"Staking",
		"Session",
		"TechnicalCommittee",
		"TechnicalMembership",
		"Grandpa",
		"Treasury",
		"Sudo",
		"ImOnline",
		"AuthorityDiscovery",
		"Offences",
		"Historical",
		"Scheduler",
		"Bounties",
		"Tips",
		"Mmr",
		"DataAvailability",
		"NomadUpdaterManager",
		"NomadHome",
		"NomadDABridge",
		"Preimage",
		"Multisig",
		"VoterList",
		"NominationPools",
		"Identity",
		"Mandate",
	];
	#[doc = r" The error type returned when there is a runtime issue."]
	pub type DispatchError = runtime_types::sp_runtime::DispatchError;
	#[derive(
		:: subxt :: ext :: codec :: Decode,
		:: subxt :: ext :: codec :: Encode,
		:: subxt :: ext :: scale_decode :: DecodeAsType,
		:: subxt :: ext :: scale_encode :: EncodeAsType,
		Clone,
		Debug,
		Eq,
		PartialEq,
	)]
	# [codec (crate = :: subxt :: ext :: codec)]
	#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
	#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
	pub enum Event {
		#[codec(index = 0)]
		System(system::Event),
		#[codec(index = 1)]
		Utility(utility::Event),
		#[codec(index = 5)]
		Indices(indices::Event),
		#[codec(index = 6)]
		Balances(balances::Event),
		#[codec(index = 7)]
		TransactionPayment(transaction_payment::Event),
		#[codec(index = 9)]
		ElectionProviderMultiPhase(election_provider_multi_phase::Event),
		#[codec(index = 10)]
		Staking(staking::Event),
		#[codec(index = 11)]
		Session(session::Event),
		#[codec(index = 14)]
		TechnicalCommittee(technical_committee::Event),
		#[codec(index = 16)]
		TechnicalMembership(technical_membership::Event),
		#[codec(index = 17)]
		Grandpa(grandpa::Event),
		#[codec(index = 18)]
		Treasury(treasury::Event),
		#[codec(index = 19)]
		Sudo(sudo::Event),
		#[codec(index = 20)]
		ImOnline(im_online::Event),
		#[codec(index = 22)]
		Offences(offences::Event),
		#[codec(index = 24)]
		Scheduler(scheduler::Event),
		#[codec(index = 25)]
		Bounties(bounties::Event),
		#[codec(index = 26)]
		Tips(tips::Event),
		#[codec(index = 29)]
		DataAvailability(data_availability::Event),
		#[codec(index = 30)]
		NomadUpdaterManager(nomad_updater_manager::Event),
		#[codec(index = 31)]
		NomadHome(nomad_home::Event),
		#[codec(index = 32)]
		NomadDABridge(nomad_da_bridge::Event),
		#[codec(index = 33)]
		Preimage(preimage::Event),
		#[codec(index = 34)]
		Multisig(multisig::Event),
		#[codec(index = 35)]
		VoterList(voter_list::Event),
		#[codec(index = 36)]
		NominationPools(nomination_pools::Event),
		#[codec(index = 37)]
		Identity(identity::Event),
		#[codec(index = 38)]
		Mandate(mandate::Event),
	}
	impl ::subxt::events::RootEvent for Event {
		fn root_event(
			pallet_bytes: &[u8],
			pallet_name: &str,
			pallet_ty: u32,
			metadata: &::subxt::Metadata,
		) -> Result<Self, ::subxt::Error> {
			use ::subxt::metadata::DecodeWithMetadata;
			if pallet_name == "System" {
				return Ok(Event::System(system::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Utility" {
				return Ok(Event::Utility(utility::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Indices" {
				return Ok(Event::Indices(indices::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Balances" {
				return Ok(Event::Balances(balances::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "TransactionPayment" {
				return Ok(Event::TransactionPayment(
					transaction_payment::Event::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				));
			}
			if pallet_name == "ElectionProviderMultiPhase" {
				return Ok(Event::ElectionProviderMultiPhase(
					election_provider_multi_phase::Event::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				));
			}
			if pallet_name == "Staking" {
				return Ok(Event::Staking(staking::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Session" {
				return Ok(Event::Session(session::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "TechnicalCommittee" {
				return Ok(Event::TechnicalCommittee(
					technical_committee::Event::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				));
			}
			if pallet_name == "TechnicalMembership" {
				return Ok(Event::TechnicalMembership(
					technical_membership::Event::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				));
			}
			if pallet_name == "Grandpa" {
				return Ok(Event::Grandpa(grandpa::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Treasury" {
				return Ok(Event::Treasury(treasury::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Sudo" {
				return Ok(Event::Sudo(sudo::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "ImOnline" {
				return Ok(Event::ImOnline(im_online::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Offences" {
				return Ok(Event::Offences(offences::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Scheduler" {
				return Ok(Event::Scheduler(scheduler::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Bounties" {
				return Ok(Event::Bounties(bounties::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Tips" {
				return Ok(Event::Tips(tips::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "DataAvailability" {
				return Ok(Event::DataAvailability(
					data_availability::Event::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				));
			}
			if pallet_name == "NomadUpdaterManager" {
				return Ok(Event::NomadUpdaterManager(
					nomad_updater_manager::Event::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				));
			}
			if pallet_name == "NomadHome" {
				return Ok(Event::NomadHome(nomad_home::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "NomadDABridge" {
				return Ok(Event::NomadDABridge(
					nomad_da_bridge::Event::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				));
			}
			if pallet_name == "Preimage" {
				return Ok(Event::Preimage(preimage::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Multisig" {
				return Ok(Event::Multisig(multisig::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "VoterList" {
				return Ok(Event::VoterList(voter_list::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "NominationPools" {
				return Ok(Event::NominationPools(
					nomination_pools::Event::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				));
			}
			if pallet_name == "Identity" {
				return Ok(Event::Identity(identity::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Mandate" {
				return Ok(Event::Mandate(mandate::Event::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			Err(::subxt::ext::scale_decode::Error::custom(format!(
				"Pallet name '{}' not found in root Event enum",
				pallet_name
			))
			.into())
		}
	}
	#[derive(
		:: subxt :: ext :: codec :: Decode,
		:: subxt :: ext :: codec :: Encode,
		:: subxt :: ext :: scale_decode :: DecodeAsType,
		:: subxt :: ext :: scale_encode :: EncodeAsType,
		Clone,
		Debug,
		Eq,
		PartialEq,
	)]
	# [codec (crate = :: subxt :: ext :: codec)]
	#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
	#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
	pub enum Call {
		#[codec(index = 0)]
		System(system::Call),
		#[codec(index = 1)]
		Utility(utility::Call),
		#[codec(index = 2)]
		Babe(babe::Call),
		#[codec(index = 3)]
		Timestamp(timestamp::Call),
		#[codec(index = 5)]
		Indices(indices::Call),
		#[codec(index = 6)]
		Balances(balances::Call),
		#[codec(index = 9)]
		ElectionProviderMultiPhase(election_provider_multi_phase::Call),
		#[codec(index = 10)]
		Staking(staking::Call),
		#[codec(index = 11)]
		Session(session::Call),
		#[codec(index = 14)]
		TechnicalCommittee(technical_committee::Call),
		#[codec(index = 16)]
		TechnicalMembership(technical_membership::Call),
		#[codec(index = 17)]
		Grandpa(grandpa::Call),
		#[codec(index = 18)]
		Treasury(treasury::Call),
		#[codec(index = 19)]
		Sudo(sudo::Call),
		#[codec(index = 20)]
		ImOnline(im_online::Call),
		#[codec(index = 24)]
		Scheduler(scheduler::Call),
		#[codec(index = 25)]
		Bounties(bounties::Call),
		#[codec(index = 26)]
		Tips(tips::Call),
		#[codec(index = 29)]
		DataAvailability(data_availability::Call),
		#[codec(index = 30)]
		NomadUpdaterManager(nomad_updater_manager::Call),
		#[codec(index = 31)]
		NomadHome(nomad_home::Call),
		#[codec(index = 32)]
		NomadDABridge(nomad_da_bridge::Call),
		#[codec(index = 33)]
		Preimage(preimage::Call),
		#[codec(index = 34)]
		Multisig(multisig::Call),
		#[codec(index = 35)]
		VoterList(voter_list::Call),
		#[codec(index = 36)]
		NominationPools(nomination_pools::Call),
		#[codec(index = 37)]
		Identity(identity::Call),
		#[codec(index = 38)]
		Mandate(mandate::Call),
	}
	impl ::subxt::blocks::RootExtrinsic for Call {
		fn root_extrinsic(
			pallet_bytes: &[u8],
			pallet_name: &str,
			pallet_ty: u32,
			metadata: &::subxt::Metadata,
		) -> Result<Self, ::subxt::Error> {
			use ::subxt::metadata::DecodeWithMetadata;
			if pallet_name == "System" {
				return Ok(Call::System(system::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Utility" {
				return Ok(Call::Utility(utility::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Babe" {
				return Ok(Call::Babe(babe::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Timestamp" {
				return Ok(Call::Timestamp(timestamp::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Indices" {
				return Ok(Call::Indices(indices::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Balances" {
				return Ok(Call::Balances(balances::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "ElectionProviderMultiPhase" {
				return Ok(Call::ElectionProviderMultiPhase(
					election_provider_multi_phase::Call::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				));
			}
			if pallet_name == "Staking" {
				return Ok(Call::Staking(staking::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Session" {
				return Ok(Call::Session(session::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "TechnicalCommittee" {
				return Ok(Call::TechnicalCommittee(
					technical_committee::Call::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				));
			}
			if pallet_name == "TechnicalMembership" {
				return Ok(Call::TechnicalMembership(
					technical_membership::Call::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				));
			}
			if pallet_name == "Grandpa" {
				return Ok(Call::Grandpa(grandpa::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Treasury" {
				return Ok(Call::Treasury(treasury::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Sudo" {
				return Ok(Call::Sudo(sudo::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "ImOnline" {
				return Ok(Call::ImOnline(im_online::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Scheduler" {
				return Ok(Call::Scheduler(scheduler::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Bounties" {
				return Ok(Call::Bounties(bounties::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Tips" {
				return Ok(Call::Tips(tips::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "DataAvailability" {
				return Ok(Call::DataAvailability(
					data_availability::Call::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				));
			}
			if pallet_name == "NomadUpdaterManager" {
				return Ok(Call::NomadUpdaterManager(
					nomad_updater_manager::Call::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				));
			}
			if pallet_name == "NomadHome" {
				return Ok(Call::NomadHome(nomad_home::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "NomadDABridge" {
				return Ok(Call::NomadDABridge(
					nomad_da_bridge::Call::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				));
			}
			if pallet_name == "Preimage" {
				return Ok(Call::Preimage(preimage::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Multisig" {
				return Ok(Call::Multisig(multisig::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "VoterList" {
				return Ok(Call::VoterList(voter_list::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "NominationPools" {
				return Ok(Call::NominationPools(
					nomination_pools::Call::decode_with_metadata(
						&mut &*pallet_bytes,
						pallet_ty,
						metadata,
					)?,
				));
			}
			if pallet_name == "Identity" {
				return Ok(Call::Identity(identity::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			if pallet_name == "Mandate" {
				return Ok(Call::Mandate(mandate::Call::decode_with_metadata(
					&mut &*pallet_bytes,
					pallet_ty,
					metadata,
				)?));
			}
			Err(::subxt::ext::scale_decode::Error::custom(format!(
				"Pallet name '{}' not found in root Call enum",
				pallet_name
			))
			.into())
		}
	}
	#[derive(
		:: subxt :: ext :: codec :: Decode,
		:: subxt :: ext :: codec :: Encode,
		:: subxt :: ext :: scale_decode :: DecodeAsType,
		:: subxt :: ext :: scale_encode :: EncodeAsType,
		Clone,
		Debug,
		Eq,
		PartialEq,
	)]
	# [codec (crate = :: subxt :: ext :: codec)]
	#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
	#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
	pub enum Error {
		#[codec(index = 0)]
		System(system::Error),
		#[codec(index = 1)]
		Utility(utility::Error),
		#[codec(index = 2)]
		Babe(babe::Error),
		#[codec(index = 5)]
		Indices(indices::Error),
		#[codec(index = 6)]
		Balances(balances::Error),
		#[codec(index = 9)]
		ElectionProviderMultiPhase(election_provider_multi_phase::Error),
		#[codec(index = 10)]
		Staking(staking::Error),
		#[codec(index = 11)]
		Session(session::Error),
		#[codec(index = 14)]
		TechnicalCommittee(technical_committee::Error),
		#[codec(index = 16)]
		TechnicalMembership(technical_membership::Error),
		#[codec(index = 17)]
		Grandpa(grandpa::Error),
		#[codec(index = 18)]
		Treasury(treasury::Error),
		#[codec(index = 19)]
		Sudo(sudo::Error),
		#[codec(index = 20)]
		ImOnline(im_online::Error),
		#[codec(index = 24)]
		Scheduler(scheduler::Error),
		#[codec(index = 25)]
		Bounties(bounties::Error),
		#[codec(index = 26)]
		Tips(tips::Error),
		#[codec(index = 29)]
		DataAvailability(data_availability::Error),
		#[codec(index = 30)]
		NomadUpdaterManager(nomad_updater_manager::Error),
		#[codec(index = 31)]
		NomadHome(nomad_home::Error),
		#[codec(index = 32)]
		NomadDABridge(nomad_da_bridge::Error),
		#[codec(index = 33)]
		Preimage(preimage::Error),
		#[codec(index = 34)]
		Multisig(multisig::Error),
		#[codec(index = 35)]
		VoterList(voter_list::Error),
		#[codec(index = 36)]
		NominationPools(nomination_pools::Error),
		#[codec(index = 37)]
		Identity(identity::Error),
	}
	impl ::subxt::error::RootError for Error {
		fn root_error(
			pallet_bytes: &[u8],
			pallet_name: &str,
			metadata: &::subxt::Metadata,
		) -> Result<Self, ::subxt::Error> {
			use ::subxt::metadata::DecodeWithMetadata;
			let cursor = &mut &pallet_bytes[..];
			if pallet_name == "System" {
				let variant_error = system::Error::decode_with_metadata(cursor, 120u32, metadata)?;
				return Ok(Error::System(variant_error));
			}
			if pallet_name == "Utility" {
				let variant_error = utility::Error::decode_with_metadata(cursor, 312u32, metadata)?;
				return Ok(Error::Utility(variant_error));
			}
			if pallet_name == "Babe" {
				let variant_error = babe::Error::decode_with_metadata(cursor, 328u32, metadata)?;
				return Ok(Error::Babe(variant_error));
			}
			if pallet_name == "Indices" {
				let variant_error = indices::Error::decode_with_metadata(cursor, 330u32, metadata)?;
				return Ok(Error::Indices(variant_error));
			}
			if pallet_name == "Balances" {
				let variant_error =
					balances::Error::decode_with_metadata(cursor, 345u32, metadata)?;
				return Ok(Error::Balances(variant_error));
			}
			if pallet_name == "ElectionProviderMultiPhase" {
				let variant_error = election_provider_multi_phase::Error::decode_with_metadata(
					cursor, 358u32, metadata,
				)?;
				return Ok(Error::ElectionProviderMultiPhase(variant_error));
			}
			if pallet_name == "Staking" {
				let variant_error = staking::Error::decode_with_metadata(cursor, 379u32, metadata)?;
				return Ok(Error::Staking(variant_error));
			}
			if pallet_name == "Session" {
				let variant_error = session::Error::decode_with_metadata(cursor, 384u32, metadata)?;
				return Ok(Error::Session(variant_error));
			}
			if pallet_name == "TechnicalCommittee" {
				let variant_error =
					technical_committee::Error::decode_with_metadata(cursor, 387u32, metadata)?;
				return Ok(Error::TechnicalCommittee(variant_error));
			}
			if pallet_name == "TechnicalMembership" {
				let variant_error =
					technical_membership::Error::decode_with_metadata(cursor, 389u32, metadata)?;
				return Ok(Error::TechnicalMembership(variant_error));
			}
			if pallet_name == "Grandpa" {
				let variant_error = grandpa::Error::decode_with_metadata(cursor, 393u32, metadata)?;
				return Ok(Error::Grandpa(variant_error));
			}
			if pallet_name == "Treasury" {
				let variant_error =
					treasury::Error::decode_with_metadata(cursor, 399u32, metadata)?;
				return Ok(Error::Treasury(variant_error));
			}
			if pallet_name == "Sudo" {
				let variant_error = sudo::Error::decode_with_metadata(cursor, 400u32, metadata)?;
				return Ok(Error::Sudo(variant_error));
			}
			if pallet_name == "ImOnline" {
				let variant_error =
					im_online::Error::decode_with_metadata(cursor, 403u32, metadata)?;
				return Ok(Error::ImOnline(variant_error));
			}
			if pallet_name == "Scheduler" {
				let variant_error =
					scheduler::Error::decode_with_metadata(cursor, 415u32, metadata)?;
				return Ok(Error::Scheduler(variant_error));
			}
			if pallet_name == "Bounties" {
				let variant_error =
					bounties::Error::decode_with_metadata(cursor, 419u32, metadata)?;
				return Ok(Error::Bounties(variant_error));
			}
			if pallet_name == "Tips" {
				let variant_error = tips::Error::decode_with_metadata(cursor, 421u32, metadata)?;
				return Ok(Error::Tips(variant_error));
			}
			if pallet_name == "DataAvailability" {
				let variant_error =
					data_availability::Error::decode_with_metadata(cursor, 423u32, metadata)?;
				return Ok(Error::DataAvailability(variant_error));
			}
			if pallet_name == "NomadUpdaterManager" {
				let variant_error =
					nomad_updater_manager::Error::decode_with_metadata(cursor, 424u32, metadata)?;
				return Ok(Error::NomadUpdaterManager(variant_error));
			}
			if pallet_name == "NomadHome" {
				let variant_error =
					nomad_home::Error::decode_with_metadata(cursor, 429u32, metadata)?;
				return Ok(Error::NomadHome(variant_error));
			}
			if pallet_name == "NomadDABridge" {
				let variant_error =
					nomad_da_bridge::Error::decode_with_metadata(cursor, 430u32, metadata)?;
				return Ok(Error::NomadDABridge(variant_error));
			}
			if pallet_name == "Preimage" {
				let variant_error =
					preimage::Error::decode_with_metadata(cursor, 434u32, metadata)?;
				return Ok(Error::Preimage(variant_error));
			}
			if pallet_name == "Multisig" {
				let variant_error =
					multisig::Error::decode_with_metadata(cursor, 437u32, metadata)?;
				return Ok(Error::Multisig(variant_error));
			}
			if pallet_name == "VoterList" {
				let variant_error =
					voter_list::Error::decode_with_metadata(cursor, 441u32, metadata)?;
				return Ok(Error::VoterList(variant_error));
			}
			if pallet_name == "NominationPools" {
				let variant_error =
					nomination_pools::Error::decode_with_metadata(cursor, 461u32, metadata)?;
				return Ok(Error::NominationPools(variant_error));
			}
			if pallet_name == "Identity" {
				let variant_error =
					identity::Error::decode_with_metadata(cursor, 473u32, metadata)?;
				return Ok(Error::Identity(variant_error));
			}
			Err(::subxt::ext::scale_decode::Error::custom(format!(
				"Pallet name '{}' not found in root Error enum",
				pallet_name
			))
			.into())
		}
	}
	pub fn constants() -> ConstantsApi {
		ConstantsApi
	}
	pub fn storage() -> StorageApi {
		StorageApi
	}
	pub fn tx() -> TransactionApi {
		TransactionApi
	}
	pub fn apis() -> runtime_apis::RuntimeApi {
		runtime_apis::RuntimeApi
	}
	pub mod runtime_apis {
		use super::root_mod;
		use super::runtime_types;
		use ::subxt::ext::codec::Encode;
		pub struct RuntimeApi;
		impl RuntimeApi {}
	}
	pub struct ConstantsApi;
	impl ConstantsApi {
		pub fn system(&self) -> system::constants::ConstantsApi {
			system::constants::ConstantsApi
		}
		pub fn utility(&self) -> utility::constants::ConstantsApi {
			utility::constants::ConstantsApi
		}
		pub fn babe(&self) -> babe::constants::ConstantsApi {
			babe::constants::ConstantsApi
		}
		pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
			timestamp::constants::ConstantsApi
		}
		pub fn indices(&self) -> indices::constants::ConstantsApi {
			indices::constants::ConstantsApi
		}
		pub fn balances(&self) -> balances::constants::ConstantsApi {
			balances::constants::ConstantsApi
		}
		pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
			transaction_payment::constants::ConstantsApi
		}
		pub fn election_provider_multi_phase(
			&self,
		) -> election_provider_multi_phase::constants::ConstantsApi {
			election_provider_multi_phase::constants::ConstantsApi
		}
		pub fn staking(&self) -> staking::constants::ConstantsApi {
			staking::constants::ConstantsApi
		}
		pub fn technical_committee(&self) -> technical_committee::constants::ConstantsApi {
			technical_committee::constants::ConstantsApi
		}
		pub fn grandpa(&self) -> grandpa::constants::ConstantsApi {
			grandpa::constants::ConstantsApi
		}
		pub fn treasury(&self) -> treasury::constants::ConstantsApi {
			treasury::constants::ConstantsApi
		}
		pub fn im_online(&self) -> im_online::constants::ConstantsApi {
			im_online::constants::ConstantsApi
		}
		pub fn scheduler(&self) -> scheduler::constants::ConstantsApi {
			scheduler::constants::ConstantsApi
		}
		pub fn bounties(&self) -> bounties::constants::ConstantsApi {
			bounties::constants::ConstantsApi
		}
		pub fn tips(&self) -> tips::constants::ConstantsApi {
			tips::constants::ConstantsApi
		}
		pub fn data_availability(&self) -> data_availability::constants::ConstantsApi {
			data_availability::constants::ConstantsApi
		}
		pub fn nomad_home(&self) -> nomad_home::constants::ConstantsApi {
			nomad_home::constants::ConstantsApi
		}
		pub fn nomad_da_bridge(&self) -> nomad_da_bridge::constants::ConstantsApi {
			nomad_da_bridge::constants::ConstantsApi
		}
		pub fn multisig(&self) -> multisig::constants::ConstantsApi {
			multisig::constants::ConstantsApi
		}
		pub fn voter_list(&self) -> voter_list::constants::ConstantsApi {
			voter_list::constants::ConstantsApi
		}
		pub fn nomination_pools(&self) -> nomination_pools::constants::ConstantsApi {
			nomination_pools::constants::ConstantsApi
		}
		pub fn identity(&self) -> identity::constants::ConstantsApi {
			identity::constants::ConstantsApi
		}
	}
	pub struct StorageApi;
	impl StorageApi {
		pub fn system(&self) -> system::storage::StorageApi {
			system::storage::StorageApi
		}
		pub fn babe(&self) -> babe::storage::StorageApi {
			babe::storage::StorageApi
		}
		pub fn timestamp(&self) -> timestamp::storage::StorageApi {
			timestamp::storage::StorageApi
		}
		pub fn authorship(&self) -> authorship::storage::StorageApi {
			authorship::storage::StorageApi
		}
		pub fn indices(&self) -> indices::storage::StorageApi {
			indices::storage::StorageApi
		}
		pub fn balances(&self) -> balances::storage::StorageApi {
			balances::storage::StorageApi
		}
		pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
			transaction_payment::storage::StorageApi
		}
		pub fn election_provider_multi_phase(
			&self,
		) -> election_provider_multi_phase::storage::StorageApi {
			election_provider_multi_phase::storage::StorageApi
		}
		pub fn staking(&self) -> staking::storage::StorageApi {
			staking::storage::StorageApi
		}
		pub fn session(&self) -> session::storage::StorageApi {
			session::storage::StorageApi
		}
		pub fn technical_committee(&self) -> technical_committee::storage::StorageApi {
			technical_committee::storage::StorageApi
		}
		pub fn technical_membership(&self) -> technical_membership::storage::StorageApi {
			technical_membership::storage::StorageApi
		}
		pub fn grandpa(&self) -> grandpa::storage::StorageApi {
			grandpa::storage::StorageApi
		}
		pub fn treasury(&self) -> treasury::storage::StorageApi {
			treasury::storage::StorageApi
		}
		pub fn sudo(&self) -> sudo::storage::StorageApi {
			sudo::storage::StorageApi
		}
		pub fn im_online(&self) -> im_online::storage::StorageApi {
			im_online::storage::StorageApi
		}
		pub fn authority_discovery(&self) -> authority_discovery::storage::StorageApi {
			authority_discovery::storage::StorageApi
		}
		pub fn offences(&self) -> offences::storage::StorageApi {
			offences::storage::StorageApi
		}
		pub fn historical(&self) -> historical::storage::StorageApi {
			historical::storage::StorageApi
		}
		pub fn scheduler(&self) -> scheduler::storage::StorageApi {
			scheduler::storage::StorageApi
		}
		pub fn bounties(&self) -> bounties::storage::StorageApi {
			bounties::storage::StorageApi
		}
		pub fn tips(&self) -> tips::storage::StorageApi {
			tips::storage::StorageApi
		}
		pub fn mmr(&self) -> mmr::storage::StorageApi {
			mmr::storage::StorageApi
		}
		pub fn data_availability(&self) -> data_availability::storage::StorageApi {
			data_availability::storage::StorageApi
		}
		pub fn nomad_updater_manager(&self) -> nomad_updater_manager::storage::StorageApi {
			nomad_updater_manager::storage::StorageApi
		}
		pub fn nomad_home(&self) -> nomad_home::storage::StorageApi {
			nomad_home::storage::StorageApi
		}
		pub fn preimage(&self) -> preimage::storage::StorageApi {
			preimage::storage::StorageApi
		}
		pub fn multisig(&self) -> multisig::storage::StorageApi {
			multisig::storage::StorageApi
		}
		pub fn voter_list(&self) -> voter_list::storage::StorageApi {
			voter_list::storage::StorageApi
		}
		pub fn nomination_pools(&self) -> nomination_pools::storage::StorageApi {
			nomination_pools::storage::StorageApi
		}
		pub fn identity(&self) -> identity::storage::StorageApi {
			identity::storage::StorageApi
		}
	}
	pub struct TransactionApi;
	impl TransactionApi {
		pub fn system(&self) -> system::calls::TransactionApi {
			system::calls::TransactionApi
		}
		pub fn utility(&self) -> utility::calls::TransactionApi {
			utility::calls::TransactionApi
		}
		pub fn babe(&self) -> babe::calls::TransactionApi {
			babe::calls::TransactionApi
		}
		pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
			timestamp::calls::TransactionApi
		}
		pub fn indices(&self) -> indices::calls::TransactionApi {
			indices::calls::TransactionApi
		}
		pub fn balances(&self) -> balances::calls::TransactionApi {
			balances::calls::TransactionApi
		}
		pub fn election_provider_multi_phase(
			&self,
		) -> election_provider_multi_phase::calls::TransactionApi {
			election_provider_multi_phase::calls::TransactionApi
		}
		pub fn staking(&self) -> staking::calls::TransactionApi {
			staking::calls::TransactionApi
		}
		pub fn session(&self) -> session::calls::TransactionApi {
			session::calls::TransactionApi
		}
		pub fn technical_committee(&self) -> technical_committee::calls::TransactionApi {
			technical_committee::calls::TransactionApi
		}
		pub fn technical_membership(&self) -> technical_membership::calls::TransactionApi {
			technical_membership::calls::TransactionApi
		}
		pub fn grandpa(&self) -> grandpa::calls::TransactionApi {
			grandpa::calls::TransactionApi
		}
		pub fn treasury(&self) -> treasury::calls::TransactionApi {
			treasury::calls::TransactionApi
		}
		pub fn sudo(&self) -> sudo::calls::TransactionApi {
			sudo::calls::TransactionApi
		}
		pub fn im_online(&self) -> im_online::calls::TransactionApi {
			im_online::calls::TransactionApi
		}
		pub fn scheduler(&self) -> scheduler::calls::TransactionApi {
			scheduler::calls::TransactionApi
		}
		pub fn bounties(&self) -> bounties::calls::TransactionApi {
			bounties::calls::TransactionApi
		}
		pub fn tips(&self) -> tips::calls::TransactionApi {
			tips::calls::TransactionApi
		}
		pub fn data_availability(&self) -> data_availability::calls::TransactionApi {
			data_availability::calls::TransactionApi
		}
		pub fn nomad_updater_manager(&self) -> nomad_updater_manager::calls::TransactionApi {
			nomad_updater_manager::calls::TransactionApi
		}
		pub fn nomad_home(&self) -> nomad_home::calls::TransactionApi {
			nomad_home::calls::TransactionApi
		}
		pub fn nomad_da_bridge(&self) -> nomad_da_bridge::calls::TransactionApi {
			nomad_da_bridge::calls::TransactionApi
		}
		pub fn preimage(&self) -> preimage::calls::TransactionApi {
			preimage::calls::TransactionApi
		}
		pub fn multisig(&self) -> multisig::calls::TransactionApi {
			multisig::calls::TransactionApi
		}
		pub fn voter_list(&self) -> voter_list::calls::TransactionApi {
			voter_list::calls::TransactionApi
		}
		pub fn nomination_pools(&self) -> nomination_pools::calls::TransactionApi {
			nomination_pools::calls::TransactionApi
		}
		pub fn identity(&self) -> identity::calls::TransactionApi {
			identity::calls::TransactionApi
		}
		pub fn mandate(&self) -> mandate::calls::TransactionApi {
			mandate::calls::TransactionApi
		}
	}
	#[doc = r" check whether the Client you are using is aligned with the statically generated codegen."]
	pub fn validate_codegen<T: ::subxt::Config, C: ::subxt::client::OfflineClientT<T>>(
		client: &C,
	) -> Result<(), ::subxt::error::MetadataError> {
		let runtime_metadata_hash = client
			.metadata()
			.hasher()
			.only_these_pallets(&PALLETS)
			.hash();
		if runtime_metadata_hash
			!= [
				31u8, 33u8, 205u8, 190u8, 10u8, 202u8, 108u8, 42u8, 98u8, 6u8, 187u8, 205u8, 146u8,
				66u8, 136u8, 163u8, 140u8, 204u8, 239u8, 167u8, 47u8, 163u8, 47u8, 84u8, 172u8,
				244u8, 96u8, 51u8, 83u8, 57u8, 234u8, 213u8,
			] {
			Err(::subxt::error::MetadataError::IncompatibleCodegen)
		} else {
			Ok(())
		}
	}
	pub mod system {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Error for the System pallet"]
		pub type Error = runtime_types::frame_system::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::frame_system::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Remark {
					pub remark: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Remark {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "remark";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetHeapPages {
					pub pages: ::core::primitive::u64,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetHeapPages {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_heap_pages";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetCode {
					pub code: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetCode {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_code";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetCodeWithoutChecks {
					pub code: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetCodeWithoutChecks {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_code_without_checks";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetStorage {
					pub items: ::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetStorage {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_storage";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct KillStorage {
					pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				}
				impl ::subxt::blocks::StaticExtrinsic for KillStorage {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "kill_storage";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct KillPrefix {
					pub prefix: ::std::vec::Vec<::core::primitive::u8>,
					pub subkeys: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for KillPrefix {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "kill_prefix";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RemarkWithEvent {
					pub remark: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemarkWithEvent {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "remark_with_event";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::remark`]."]
				pub fn remark(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::Remark> {
					::subxt::tx::Payload::new_static(
						"System",
						"remark",
						types::Remark { remark },
						[
							43u8, 126u8, 180u8, 174u8, 141u8, 48u8, 52u8, 125u8, 166u8, 212u8,
							216u8, 98u8, 100u8, 24u8, 132u8, 71u8, 101u8, 64u8, 246u8, 169u8, 33u8,
							250u8, 147u8, 208u8, 2u8, 40u8, 129u8, 209u8, 232u8, 207u8, 207u8,
							13u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_heap_pages`]."]
				pub fn set_heap_pages(
					&self,
					pages: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<types::SetHeapPages> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_heap_pages",
						types::SetHeapPages { pages },
						[
							188u8, 191u8, 99u8, 216u8, 219u8, 109u8, 141u8, 50u8, 78u8, 235u8,
							215u8, 242u8, 195u8, 24u8, 111u8, 76u8, 229u8, 64u8, 99u8, 225u8,
							134u8, 121u8, 81u8, 209u8, 127u8, 223u8, 98u8, 215u8, 150u8, 70u8,
							57u8, 147u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_code`]."]
				pub fn set_code(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::SetCode> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_code",
						types::SetCode { code },
						[
							233u8, 248u8, 88u8, 245u8, 28u8, 65u8, 25u8, 169u8, 35u8, 237u8, 19u8,
							203u8, 136u8, 160u8, 18u8, 3u8, 20u8, 197u8, 81u8, 169u8, 244u8, 188u8,
							27u8, 147u8, 147u8, 236u8, 65u8, 25u8, 3u8, 143u8, 182u8, 22u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_code_without_checks`]."]
				pub fn set_code_without_checks(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::SetCodeWithoutChecks> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_code_without_checks",
						types::SetCodeWithoutChecks { code },
						[
							82u8, 212u8, 157u8, 44u8, 70u8, 0u8, 143u8, 15u8, 109u8, 109u8, 107u8,
							157u8, 141u8, 42u8, 169u8, 11u8, 15u8, 186u8, 252u8, 138u8, 10u8,
							147u8, 15u8, 178u8, 247u8, 229u8, 213u8, 98u8, 207u8, 231u8, 119u8,
							115u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_storage`]."]
				pub fn set_storage(
					&self,
					items: ::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
				) -> ::subxt::tx::Payload<types::SetStorage> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_storage",
						types::SetStorage { items },
						[
							184u8, 169u8, 248u8, 68u8, 40u8, 193u8, 190u8, 151u8, 96u8, 159u8,
							19u8, 237u8, 241u8, 156u8, 5u8, 158u8, 191u8, 237u8, 9u8, 13u8, 86u8,
							213u8, 77u8, 58u8, 48u8, 139u8, 1u8, 85u8, 220u8, 233u8, 139u8, 164u8,
						],
					)
				}
				#[doc = "See [`Pallet::kill_storage`]."]
				pub fn kill_storage(
					&self,
					keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				) -> ::subxt::tx::Payload<types::KillStorage> {
					::subxt::tx::Payload::new_static(
						"System",
						"kill_storage",
						types::KillStorage { keys },
						[
							73u8, 63u8, 196u8, 36u8, 144u8, 114u8, 34u8, 213u8, 108u8, 93u8, 209u8,
							234u8, 153u8, 185u8, 33u8, 91u8, 187u8, 195u8, 223u8, 130u8, 58u8,
							156u8, 63u8, 47u8, 228u8, 249u8, 216u8, 139u8, 143u8, 177u8, 41u8,
							35u8,
						],
					)
				}
				#[doc = "See [`Pallet::kill_prefix`]."]
				pub fn kill_prefix(
					&self,
					prefix: ::std::vec::Vec<::core::primitive::u8>,
					subkeys: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::KillPrefix> {
					::subxt::tx::Payload::new_static(
						"System",
						"kill_prefix",
						types::KillPrefix { prefix, subkeys },
						[
							184u8, 57u8, 139u8, 24u8, 208u8, 87u8, 108u8, 215u8, 198u8, 189u8,
							175u8, 242u8, 167u8, 215u8, 97u8, 63u8, 110u8, 166u8, 238u8, 98u8,
							67u8, 236u8, 111u8, 110u8, 234u8, 81u8, 102u8, 5u8, 182u8, 5u8, 214u8,
							85u8,
						],
					)
				}
				#[doc = "See [`Pallet::remark_with_event`]."]
				pub fn remark_with_event(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::RemarkWithEvent> {
					::subxt::tx::Payload::new_static(
						"System",
						"remark_with_event",
						types::RemarkWithEvent { remark },
						[
							120u8, 120u8, 153u8, 92u8, 184u8, 85u8, 34u8, 2u8, 174u8, 206u8, 105u8,
							228u8, 233u8, 130u8, 80u8, 246u8, 228u8, 59u8, 234u8, 240u8, 4u8, 49u8,
							147u8, 170u8, 115u8, 91u8, 149u8, 200u8, 228u8, 181u8, 8u8, 154u8,
						],
					)
				}
			}
		}
		#[doc = "Event for the System pallet."]
		pub type Event = runtime_types::frame_system::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An extrinsic completed successfully."]
			pub struct ExtrinsicSuccess {
				pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
			}
			impl ::subxt::events::StaticEvent for ExtrinsicSuccess {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicSuccess";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An extrinsic failed."]
			pub struct ExtrinsicFailed {
				pub dispatch_error: runtime_types::sp_runtime::DispatchError,
				pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
			}
			impl ::subxt::events::StaticEvent for ExtrinsicFailed {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "`:code` was updated."]
			pub struct CodeUpdated;
			impl ::subxt::events::StaticEvent for CodeUpdated {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "CodeUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A new account was created."]
			pub struct NewAccount {
				pub account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for NewAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "NewAccount";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account was reaped."]
			pub struct KilledAccount {
				pub account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for KilledAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "KilledAccount";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "On on-chain remark happened."]
			pub struct Remarked {
				pub sender: ::subxt::utils::AccountId32,
				pub hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Remarked {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "Remarked";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "On on-chain remark happend called by Root."]
			pub struct RemarkedByRoot {
				pub hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for RemarkedByRoot {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "RemarkedByRoot";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The full account information for a particular account ID."]
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_system::AccountInfo<
						::core::primitive::u32,
						runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Account",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							234u8, 12u8, 167u8, 96u8, 2u8, 244u8, 235u8, 62u8, 153u8, 200u8, 96u8,
							74u8, 135u8, 8u8, 35u8, 188u8, 146u8, 249u8, 246u8, 40u8, 224u8, 22u8,
							15u8, 99u8, 150u8, 222u8, 82u8, 85u8, 123u8, 123u8, 19u8, 110u8,
						],
					)
				}
				#[doc = " The full account information for a particular account ID."]
				pub fn account_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_system::AccountInfo<
						::core::primitive::u32,
						runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Account",
						Vec::new(),
						[
							234u8, 12u8, 167u8, 96u8, 2u8, 244u8, 235u8, 62u8, 153u8, 200u8, 96u8,
							74u8, 135u8, 8u8, 35u8, 188u8, 146u8, 249u8, 246u8, 40u8, 224u8, 22u8,
							15u8, 99u8, 150u8, 222u8, 82u8, 85u8, 123u8, 123u8, 19u8, 110u8,
						],
					)
				}
				#[doc = " Total extrinsics count for the current block."]
				pub fn extrinsic_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ExtrinsicCount",
						vec![],
						[
							102u8, 76u8, 236u8, 42u8, 40u8, 231u8, 33u8, 222u8, 123u8, 147u8,
							153u8, 148u8, 234u8, 203u8, 181u8, 119u8, 6u8, 187u8, 177u8, 199u8,
							120u8, 47u8, 137u8, 254u8, 96u8, 100u8, 165u8, 182u8, 249u8, 230u8,
							159u8, 79u8,
						],
					)
				}
				#[doc = " The current weight for the block."]
				pub fn block_weight(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_support::dispatch::PerDispatchClass<
						runtime_types::sp_weights::weight_v2::Weight,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"BlockWeight",
						vec![],
						[
							52u8, 191u8, 212u8, 137u8, 26u8, 39u8, 239u8, 35u8, 182u8, 32u8, 39u8,
							103u8, 56u8, 184u8, 60u8, 159u8, 167u8, 232u8, 193u8, 116u8, 105u8,
							56u8, 98u8, 127u8, 124u8, 188u8, 214u8, 154u8, 160u8, 41u8, 20u8,
							162u8,
						],
					)
				}
				#[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
				pub fn all_extrinsics_len(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_system::ExtrinsicLen,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"AllExtrinsicsLen",
						vec![],
						[
							210u8, 178u8, 187u8, 254u8, 187u8, 172u8, 33u8, 225u8, 56u8, 183u8,
							1u8, 152u8, 21u8, 25u8, 90u8, 117u8, 173u8, 85u8, 113u8, 166u8, 211u8,
							152u8, 186u8, 48u8, 171u8, 246u8, 67u8, 194u8, 107u8, 14u8, 104u8,
							179u8,
						],
					)
				}
				#[doc = " Map of block numbers to block hashes."]
				pub fn block_hash(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::H256,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"BlockHash",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
							103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
							164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
							202u8, 118u8,
						],
					)
				}
				#[doc = " Map of block numbers to block hashes."]
				pub fn block_hash_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::H256,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"BlockHash",
						Vec::new(),
						[
							217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
							103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
							164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
							202u8, 118u8,
						],
					)
				}
				#[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
				pub fn extrinsic_data(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ExtrinsicData",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
							220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
							128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
						],
					)
				}
				#[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
				pub fn extrinsic_data_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ExtrinsicData",
						Vec::new(),
						[
							160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
							220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
							128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
						],
					)
				}
				#[doc = " The current block number being processed. Set by `execute_block`."]
				pub fn number(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Number",
						vec![],
						[
							30u8, 194u8, 177u8, 90u8, 194u8, 232u8, 46u8, 180u8, 85u8, 129u8, 14u8,
							9u8, 8u8, 8u8, 23u8, 95u8, 230u8, 5u8, 13u8, 105u8, 125u8, 2u8, 22u8,
							200u8, 78u8, 93u8, 115u8, 28u8, 150u8, 113u8, 48u8, 53u8,
						],
					)
				}
				#[doc = " Hash of the previous block."]
				pub fn parent_hash(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::H256,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ParentHash",
						vec![],
						[
							26u8, 130u8, 11u8, 216u8, 155u8, 71u8, 128u8, 170u8, 30u8, 153u8, 21u8,
							192u8, 62u8, 93u8, 137u8, 80u8, 120u8, 81u8, 202u8, 94u8, 248u8, 125u8,
							71u8, 82u8, 141u8, 229u8, 32u8, 56u8, 73u8, 50u8, 101u8, 78u8,
						],
					)
				}
				#[doc = " Digest of the current block, also part of the block header."]
				pub fn digest(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_runtime::generic::digest::Digest,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Digest",
						vec![],
						[
							70u8, 156u8, 127u8, 89u8, 115u8, 250u8, 103u8, 62u8, 185u8, 153u8,
							26u8, 72u8, 39u8, 226u8, 181u8, 97u8, 137u8, 225u8, 45u8, 158u8, 212u8,
							254u8, 142u8, 136u8, 90u8, 22u8, 243u8, 125u8, 226u8, 49u8, 235u8,
							215u8,
						],
					)
				}
				#[doc = " Events deposited for the current block."]
				#[doc = ""]
				#[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
				#[doc = " It could otherwise inflate the PoV size of a block."]
				#[doc = ""]
				#[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
				#[doc = " just in case someone still reads them from within the runtime."]
				pub fn events(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<
						runtime_types::frame_system::EventRecord<
							runtime_types::da_runtime::RuntimeEvent,
							::subxt::utils::H256,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Events",
						vec![],
						[
							60u8, 162u8, 175u8, 153u8, 33u8, 61u8, 149u8, 182u8, 182u8, 30u8,
							192u8, 213u8, 200u8, 85u8, 103u8, 81u8, 43u8, 74u8, 242u8, 174u8, 56u8,
							179u8, 254u8, 135u8, 4u8, 240u8, 101u8, 169u8, 140u8, 11u8, 146u8,
							197u8,
						],
					)
				}
				#[doc = " The number of events in the `Events<T>` list."]
				pub fn event_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"EventCount",
						vec![],
						[
							175u8, 24u8, 252u8, 184u8, 210u8, 167u8, 146u8, 143u8, 164u8, 80u8,
							151u8, 205u8, 189u8, 189u8, 55u8, 220u8, 47u8, 101u8, 181u8, 33u8,
							254u8, 131u8, 13u8, 143u8, 3u8, 244u8, 245u8, 45u8, 2u8, 210u8, 79u8,
							133u8,
						],
					)
				}
				#[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
				#[doc = " of events in the `<Events<T>>` list."]
				#[doc = ""]
				#[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
				#[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
				#[doc = " in case of changes fetch the list of events of interest."]
				#[doc = ""]
				#[doc = " The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just"]
				#[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
				#[doc = " no notification will be triggered thus the event might be lost."]
				pub fn event_topics(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"EventTopics",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							154u8, 29u8, 31u8, 148u8, 254u8, 7u8, 124u8, 251u8, 241u8, 77u8, 24u8,
							37u8, 28u8, 75u8, 205u8, 17u8, 159u8, 79u8, 239u8, 62u8, 67u8, 60u8,
							252u8, 112u8, 215u8, 145u8, 103u8, 170u8, 110u8, 186u8, 221u8, 76u8,
						],
					)
				}
				#[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
				#[doc = " of events in the `<Events<T>>` list."]
				#[doc = ""]
				#[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
				#[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
				#[doc = " in case of changes fetch the list of events of interest."]
				#[doc = ""]
				#[doc = " The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just"]
				#[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
				#[doc = " no notification will be triggered thus the event might be lost."]
				pub fn event_topics_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"EventTopics",
						Vec::new(),
						[
							154u8, 29u8, 31u8, 148u8, 254u8, 7u8, 124u8, 251u8, 241u8, 77u8, 24u8,
							37u8, 28u8, 75u8, 205u8, 17u8, 159u8, 79u8, 239u8, 62u8, 67u8, 60u8,
							252u8, 112u8, 215u8, 145u8, 103u8, 170u8, 110u8, 186u8, 221u8, 76u8,
						],
					)
				}
				#[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
				pub fn last_runtime_upgrade(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_system::LastRuntimeUpgradeInfo,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"LastRuntimeUpgrade",
						vec![],
						[
							137u8, 29u8, 175u8, 75u8, 197u8, 208u8, 91u8, 207u8, 156u8, 87u8,
							148u8, 68u8, 91u8, 140u8, 22u8, 233u8, 1u8, 229u8, 56u8, 34u8, 40u8,
							194u8, 253u8, 30u8, 163u8, 39u8, 54u8, 209u8, 13u8, 27u8, 139u8, 184u8,
						],
					)
				}
				#[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
				pub fn upgraded_to_u32_ref_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"UpgradedToU32RefCount",
						vec![],
						[
							229u8, 73u8, 9u8, 132u8, 186u8, 116u8, 151u8, 171u8, 145u8, 29u8, 34u8,
							130u8, 52u8, 146u8, 124u8, 175u8, 79u8, 189u8, 147u8, 230u8, 234u8,
							107u8, 124u8, 31u8, 2u8, 22u8, 86u8, 190u8, 4u8, 147u8, 50u8, 245u8,
						],
					)
				}
				#[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
				#[doc = " (default) if not."]
				pub fn upgraded_to_triple_ref_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"UpgradedToTripleRefCount",
						vec![],
						[
							97u8, 66u8, 124u8, 243u8, 27u8, 167u8, 147u8, 81u8, 254u8, 201u8,
							101u8, 24u8, 40u8, 231u8, 14u8, 179u8, 154u8, 163u8, 71u8, 81u8, 185u8,
							167u8, 82u8, 254u8, 189u8, 3u8, 101u8, 207u8, 206u8, 194u8, 155u8,
							151u8,
						],
					)
				}
				#[doc = " The execution phase of the block."]
				pub fn execution_phase(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_system::Phase,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ExecutionPhase",
						vec![],
						[
							191u8, 129u8, 100u8, 134u8, 126u8, 116u8, 154u8, 203u8, 220u8, 200u8,
							0u8, 26u8, 161u8, 250u8, 133u8, 205u8, 146u8, 24u8, 5u8, 156u8, 158u8,
							35u8, 36u8, 253u8, 52u8, 235u8, 86u8, 167u8, 35u8, 100u8, 119u8, 27u8,
						],
					)
				}
				#[doc = " The dynamic block length"]
				pub fn dynamic_block_length(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_system::limits::BlockLength,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"DynamicBlockLength",
						vec![],
						[
							217u8, 155u8, 12u8, 122u8, 198u8, 141u8, 119u8, 205u8, 254u8, 77u8,
							138u8, 29u8, 107u8, 11u8, 81u8, 46u8, 163u8, 91u8, 25u8, 176u8, 179u8,
							232u8, 34u8, 239u8, 167u8, 83u8, 208u8, 92u8, 97u8, 166u8, 66u8, 253u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Block & extrinsics weights: base values and limits."]
				pub fn block_weights(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockWeights>
				{
					::subxt::constants::Address::new_static(
						"System",
						"BlockWeights",
						[
							238u8, 20u8, 221u8, 11u8, 146u8, 236u8, 47u8, 103u8, 8u8, 239u8, 13u8,
							176u8, 202u8, 10u8, 151u8, 68u8, 110u8, 162u8, 99u8, 40u8, 211u8,
							136u8, 71u8, 82u8, 50u8, 80u8, 244u8, 211u8, 231u8, 198u8, 36u8, 152u8,
						],
					)
				}
				#[doc = " The maximum length of a block (in bytes)."]
				pub fn block_length(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockLength> {
					::subxt::constants::Address::new_static(
						"System",
						"BlockLength",
						[
							166u8, 7u8, 126u8, 154u8, 133u8, 31u8, 100u8, 149u8, 118u8, 168u8, 1u8,
							21u8, 202u8, 114u8, 104u8, 193u8, 44u8, 97u8, 240u8, 37u8, 177u8, 43u8,
							83u8, 195u8, 176u8, 252u8, 201u8, 229u8, 170u8, 45u8, 136u8, 81u8,
						],
					)
				}
				#[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
				pub fn block_hash_count(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"System",
						"BlockHashCount",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The weight of runtime database operations the runtime can invoke."]
				pub fn db_weight(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_weights::RuntimeDbWeight> {
					::subxt::constants::Address::new_static(
						"System",
						"DbWeight",
						[
							206u8, 53u8, 134u8, 247u8, 42u8, 38u8, 197u8, 59u8, 191u8, 83u8, 160u8,
							9u8, 207u8, 133u8, 108u8, 152u8, 150u8, 103u8, 109u8, 228u8, 218u8,
							24u8, 27u8, 210u8, 106u8, 252u8, 74u8, 93u8, 27u8, 63u8, 109u8, 252u8,
						],
					)
				}
				#[doc = " Get the chain's current version."]
				pub fn version(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_version::RuntimeVersion> {
					::subxt::constants::Address::new_static(
						"System",
						"Version",
						[
							134u8, 0u8, 23u8, 0u8, 199u8, 213u8, 89u8, 240u8, 194u8, 186u8, 239u8,
							157u8, 168u8, 211u8, 223u8, 156u8, 138u8, 140u8, 194u8, 23u8, 167u8,
							158u8, 195u8, 233u8, 25u8, 165u8, 27u8, 237u8, 198u8, 206u8, 233u8,
							28u8,
						],
					)
				}
				#[doc = " The designated SS58 prefix of this chain."]
				#[doc = ""]
				#[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
				#[doc = " that the runtime should know about the prefix in order to make use of it as"]
				#[doc = " an identifier of the chain."]
				pub fn ss58_prefix(&self) -> ::subxt::constants::Address<::core::primitive::u16> {
					::subxt::constants::Address::new_static(
						"System",
						"SS58Prefix",
						[
							116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
							41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
							90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
						],
					)
				}
			}
		}
	}
	pub mod utility {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_utility::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_utility::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Batch {
					pub calls: ::std::vec::Vec<runtime_types::da_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Batch {
					const PALLET: &'static str = "Utility";
					const CALL: &'static str = "batch";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AsDerivative {
					pub index: ::core::primitive::u16,
					pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for AsDerivative {
					const PALLET: &'static str = "Utility";
					const CALL: &'static str = "as_derivative";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BatchAll {
					pub calls: ::std::vec::Vec<runtime_types::da_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for BatchAll {
					const PALLET: &'static str = "Utility";
					const CALL: &'static str = "batch_all";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct DispatchAs {
					pub as_origin: ::std::boxed::Box<runtime_types::da_runtime::OriginCaller>,
					pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for DispatchAs {
					const PALLET: &'static str = "Utility";
					const CALL: &'static str = "dispatch_as";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceBatch {
					pub calls: ::std::vec::Vec<runtime_types::da_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceBatch {
					const PALLET: &'static str = "Utility";
					const CALL: &'static str = "force_batch";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct WithWeight {
					pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					pub weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for WithWeight {
					const PALLET: &'static str = "Utility";
					const CALL: &'static str = "with_weight";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::batch`]."]
				pub fn batch(
					&self,
					calls: ::std::vec::Vec<runtime_types::da_runtime::RuntimeCall>,
				) -> ::subxt::tx::Payload<types::Batch> {
					::subxt::tx::Payload::new_static(
						"Utility",
						"batch",
						types::Batch { calls },
						[
							108u8, 14u8, 12u8, 71u8, 145u8, 118u8, 196u8, 13u8, 238u8, 185u8, 16u8,
							196u8, 207u8, 223u8, 98u8, 3u8, 242u8, 232u8, 190u8, 199u8, 28u8,
							170u8, 251u8, 8u8, 7u8, 27u8, 167u8, 27u8, 233u8, 189u8, 105u8, 69u8,
						],
					)
				}
				#[doc = "See [`Pallet::as_derivative`]."]
				pub fn as_derivative(
					&self,
					index: ::core::primitive::u16,
					call: runtime_types::da_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::AsDerivative> {
					::subxt::tx::Payload::new_static(
						"Utility",
						"as_derivative",
						types::AsDerivative {
							index,
							call: ::std::boxed::Box::new(call),
						},
						[
							251u8, 38u8, 215u8, 126u8, 215u8, 181u8, 174u8, 254u8, 8u8, 73u8,
							164u8, 100u8, 131u8, 180u8, 42u8, 152u8, 55u8, 78u8, 92u8, 165u8,
							170u8, 154u8, 191u8, 35u8, 116u8, 159u8, 103u8, 168u8, 109u8, 156u8,
							49u8, 143u8,
						],
					)
				}
				#[doc = "See [`Pallet::batch_all`]."]
				pub fn batch_all(
					&self,
					calls: ::std::vec::Vec<runtime_types::da_runtime::RuntimeCall>,
				) -> ::subxt::tx::Payload<types::BatchAll> {
					::subxt::tx::Payload::new_static(
						"Utility",
						"batch_all",
						types::BatchAll { calls },
						[
							59u8, 2u8, 120u8, 192u8, 146u8, 68u8, 230u8, 224u8, 73u8, 142u8, 61u8,
							39u8, 211u8, 172u8, 143u8, 76u8, 9u8, 126u8, 161u8, 211u8, 254u8, 95u8,
							185u8, 222u8, 84u8, 47u8, 32u8, 140u8, 184u8, 202u8, 195u8, 209u8,
						],
					)
				}
				#[doc = "See [`Pallet::dispatch_as`]."]
				pub fn dispatch_as(
					&self,
					as_origin: runtime_types::da_runtime::OriginCaller,
					call: runtime_types::da_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::DispatchAs> {
					::subxt::tx::Payload::new_static(
						"Utility",
						"dispatch_as",
						types::DispatchAs {
							as_origin: ::std::boxed::Box::new(as_origin),
							call: ::std::boxed::Box::new(call),
						},
						[
							71u8, 56u8, 157u8, 250u8, 235u8, 106u8, 157u8, 164u8, 190u8, 118u8,
							124u8, 157u8, 170u8, 0u8, 231u8, 230u8, 226u8, 174u8, 10u8, 100u8,
							152u8, 36u8, 239u8, 247u8, 7u8, 112u8, 212u8, 102u8, 221u8, 94u8,
							139u8, 112u8,
						],
					)
				}
				#[doc = "See [`Pallet::force_batch`]."]
				pub fn force_batch(
					&self,
					calls: ::std::vec::Vec<runtime_types::da_runtime::RuntimeCall>,
				) -> ::subxt::tx::Payload<types::ForceBatch> {
					::subxt::tx::Payload::new_static(
						"Utility",
						"force_batch",
						types::ForceBatch { calls },
						[
							108u8, 45u8, 123u8, 223u8, 121u8, 89u8, 102u8, 62u8, 58u8, 23u8, 18u8,
							41u8, 79u8, 42u8, 150u8, 141u8, 199u8, 169u8, 104u8, 193u8, 50u8,
							184u8, 134u8, 193u8, 97u8, 113u8, 66u8, 21u8, 157u8, 77u8, 50u8, 196u8,
						],
					)
				}
				#[doc = "See [`Pallet::with_weight`]."]
				pub fn with_weight(
					&self,
					call: runtime_types::da_runtime::RuntimeCall,
					weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::WithWeight> {
					::subxt::tx::Payload::new_static(
						"Utility",
						"with_weight",
						types::WithWeight {
							call: ::std::boxed::Box::new(call),
							weight,
						},
						[
							97u8, 121u8, 88u8, 83u8, 41u8, 91u8, 180u8, 148u8, 177u8, 78u8, 33u8,
							193u8, 6u8, 170u8, 126u8, 117u8, 37u8, 180u8, 133u8, 77u8, 138u8, 33u8,
							91u8, 0u8, 5u8, 185u8, 195u8, 253u8, 40u8, 81u8, 218u8, 29u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_utility::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Batch of dispatches did not complete fully. Index of first failing dispatch given, as"]
			#[doc = "well as the error."]
			pub struct BatchInterrupted {
				pub index: ::core::primitive::u32,
				pub error: runtime_types::sp_runtime::DispatchError,
			}
			impl ::subxt::events::StaticEvent for BatchInterrupted {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "BatchInterrupted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Batch of dispatches completed fully with no error."]
			pub struct BatchCompleted;
			impl ::subxt::events::StaticEvent for BatchCompleted {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "BatchCompleted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Batch of dispatches completed but has errors."]
			pub struct BatchCompletedWithErrors;
			impl ::subxt::events::StaticEvent for BatchCompletedWithErrors {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "BatchCompletedWithErrors";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A single item within a Batch of dispatches has completed with no error."]
			pub struct ItemCompleted;
			impl ::subxt::events::StaticEvent for ItemCompleted {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "ItemCompleted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A single item within a Batch of dispatches has completed with error."]
			pub struct ItemFailed {
				pub error: runtime_types::sp_runtime::DispatchError,
			}
			impl ::subxt::events::StaticEvent for ItemFailed {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "ItemFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A call was dispatched."]
			pub struct DispatchedAs {
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for DispatchedAs {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "DispatchedAs";
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The limit on the number of batched calls."]
				pub fn batched_calls_limit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Utility",
						"batched_calls_limit",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod babe {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_babe::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_babe::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ReportEquivocation {
					pub equivocation_proof: ::std::boxed::Box<
						runtime_types::sp_consensus_slots::EquivocationProof<
							runtime_types::avail_core::header::Header<
								::core::primitive::u32,
								runtime_types::sp_runtime::traits::BlakeTwo256,
							>,
							runtime_types::sp_consensus_babe::app::Public,
						>,
					>,
					pub key_owner_proof: runtime_types::sp_session::MembershipProof,
				}
				impl ::subxt::blocks::StaticExtrinsic for ReportEquivocation {
					const PALLET: &'static str = "Babe";
					const CALL: &'static str = "report_equivocation";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ReportEquivocationUnsigned {
					pub equivocation_proof: ::std::boxed::Box<
						runtime_types::sp_consensus_slots::EquivocationProof<
							runtime_types::avail_core::header::Header<
								::core::primitive::u32,
								runtime_types::sp_runtime::traits::BlakeTwo256,
							>,
							runtime_types::sp_consensus_babe::app::Public,
						>,
					>,
					pub key_owner_proof: runtime_types::sp_session::MembershipProof,
				}
				impl ::subxt::blocks::StaticExtrinsic for ReportEquivocationUnsigned {
					const PALLET: &'static str = "Babe";
					const CALL: &'static str = "report_equivocation_unsigned";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PlanConfigChange {
					pub config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
				}
				impl ::subxt::blocks::StaticExtrinsic for PlanConfigChange {
					const PALLET: &'static str = "Babe";
					const CALL: &'static str = "plan_config_change";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::report_equivocation`]."]
				pub fn report_equivocation(
					&self,
					equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
						runtime_types::avail_core::header::Header<
							::core::primitive::u32,
							runtime_types::sp_runtime::traits::BlakeTwo256,
						>,
						runtime_types::sp_consensus_babe::app::Public,
					>,
					key_owner_proof: runtime_types::sp_session::MembershipProof,
				) -> ::subxt::tx::Payload<types::ReportEquivocation> {
					::subxt::tx::Payload::new_static(
						"Babe",
						"report_equivocation",
						types::ReportEquivocation {
							equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
							key_owner_proof,
						},
						[
							119u8, 52u8, 131u8, 190u8, 86u8, 255u8, 95u8, 84u8, 22u8, 237u8, 182u8,
							255u8, 232u8, 242u8, 153u8, 230u8, 13u8, 139u8, 206u8, 81u8, 71u8,
							227u8, 105u8, 72u8, 88u8, 159u8, 226u8, 106u8, 96u8, 220u8, 96u8,
							239u8,
						],
					)
				}
				#[doc = "See [`Pallet::report_equivocation_unsigned`]."]
				pub fn report_equivocation_unsigned(
					&self,
					equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
						runtime_types::avail_core::header::Header<
							::core::primitive::u32,
							runtime_types::sp_runtime::traits::BlakeTwo256,
						>,
						runtime_types::sp_consensus_babe::app::Public,
					>,
					key_owner_proof: runtime_types::sp_session::MembershipProof,
				) -> ::subxt::tx::Payload<types::ReportEquivocationUnsigned> {
					::subxt::tx::Payload::new_static(
						"Babe",
						"report_equivocation_unsigned",
						types::ReportEquivocationUnsigned {
							equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
							key_owner_proof,
						},
						[
							111u8, 105u8, 167u8, 169u8, 87u8, 120u8, 54u8, 132u8, 144u8, 132u8,
							77u8, 33u8, 80u8, 162u8, 114u8, 220u8, 108u8, 49u8, 83u8, 100u8, 156u8,
							73u8, 32u8, 165u8, 37u8, 109u8, 113u8, 99u8, 83u8, 71u8, 75u8, 173u8,
						],
					)
				}
				#[doc = "See [`Pallet::plan_config_change`]."]
				pub fn plan_config_change(
					&self,
					config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
				) -> ::subxt::tx::Payload<types::PlanConfigChange> {
					::subxt::tx::Payload::new_static(
						"Babe",
						"plan_config_change",
						types::PlanConfigChange { config },
						[
							165u8, 26u8, 134u8, 130u8, 137u8, 42u8, 127u8, 161u8, 117u8, 251u8,
							215u8, 241u8, 69u8, 224u8, 134u8, 1u8, 187u8, 203u8, 168u8, 139u8,
							121u8, 243u8, 235u8, 223u8, 135u8, 128u8, 227u8, 129u8, 183u8, 51u8,
							135u8, 79u8,
						],
					)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Current epoch index."]
				pub fn epoch_index(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"EpochIndex",
						vec![],
						[
							32u8, 82u8, 130u8, 31u8, 190u8, 162u8, 237u8, 189u8, 104u8, 244u8,
							30u8, 199u8, 179u8, 0u8, 161u8, 107u8, 72u8, 240u8, 201u8, 222u8,
							177u8, 222u8, 35u8, 156u8, 81u8, 132u8, 162u8, 118u8, 238u8, 84u8,
							112u8, 89u8,
						],
					)
				}
				#[doc = " Current epoch authorities."]
				pub fn authorities(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
						runtime_types::sp_consensus_babe::app::Public,
						::core::primitive::u64,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"Authorities",
						vec![],
						[
							67u8, 196u8, 244u8, 13u8, 246u8, 245u8, 198u8, 98u8, 81u8, 55u8, 182u8,
							187u8, 214u8, 5u8, 181u8, 76u8, 251u8, 213u8, 144u8, 166u8, 36u8,
							153u8, 234u8, 181u8, 252u8, 55u8, 198u8, 175u8, 55u8, 211u8, 105u8,
							85u8,
						],
					)
				}
				#[doc = " The slot at which the first epoch actually started. This is 0"]
				#[doc = " until the first block of the chain."]
				pub fn genesis_slot(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_consensus_slots::Slot,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"GenesisSlot",
						vec![],
						[
							218u8, 174u8, 152u8, 76u8, 188u8, 214u8, 7u8, 88u8, 253u8, 187u8,
							139u8, 234u8, 51u8, 28u8, 220u8, 57u8, 73u8, 1u8, 18u8, 205u8, 80u8,
							160u8, 120u8, 216u8, 139u8, 191u8, 100u8, 108u8, 162u8, 106u8, 175u8,
							107u8,
						],
					)
				}
				#[doc = " Current slot number."]
				pub fn current_slot(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_consensus_slots::Slot,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"CurrentSlot",
						vec![],
						[
							112u8, 199u8, 115u8, 248u8, 217u8, 242u8, 45u8, 231u8, 178u8, 53u8,
							236u8, 167u8, 219u8, 238u8, 81u8, 243u8, 39u8, 140u8, 68u8, 19u8,
							201u8, 169u8, 211u8, 133u8, 135u8, 213u8, 150u8, 105u8, 60u8, 252u8,
							43u8, 57u8,
						],
					)
				}
				#[doc = " The epoch randomness for the *current* epoch."]
				#[doc = ""]
				#[doc = " # Security"]
				#[doc = ""]
				#[doc = " This MUST NOT be used for gambling, as it can be influenced by a"]
				#[doc = " malicious validator in the short term. It MAY be used in many"]
				#[doc = " cryptographic protocols, however, so long as one remembers that this"]
				#[doc = " (like everything else on-chain) it is public. For example, it can be"]
				#[doc = " used where a number is needed that cannot have been chosen by an"]
				#[doc = " adversary, for purposes such as public-coin zero-knowledge proofs."]
				pub fn randomness(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					[::core::primitive::u8; 32usize],
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"Randomness",
						vec![],
						[
							36u8, 15u8, 52u8, 73u8, 195u8, 177u8, 186u8, 125u8, 134u8, 11u8, 103u8,
							248u8, 170u8, 237u8, 105u8, 239u8, 168u8, 204u8, 147u8, 52u8, 15u8,
							226u8, 126u8, 176u8, 133u8, 186u8, 169u8, 241u8, 156u8, 118u8, 67u8,
							58u8,
						],
					)
				}
				#[doc = " Pending epoch configuration change that will be applied when the next epoch is enacted."]
				pub fn pending_epoch_config_change(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"PendingEpochConfigChange",
						vec![],
						[
							71u8, 143u8, 197u8, 44u8, 242u8, 120u8, 71u8, 244u8, 41u8, 201u8,
							132u8, 103u8, 96u8, 23u8, 111u8, 232u8, 30u8, 35u8, 154u8, 251u8,
							183u8, 23u8, 144u8, 80u8, 101u8, 117u8, 43u8, 228u8, 174u8, 221u8,
							183u8, 165u8,
						],
					)
				}
				#[doc = " Next epoch randomness."]
				pub fn next_randomness(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					[::core::primitive::u8; 32usize],
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"NextRandomness",
						vec![],
						[
							96u8, 191u8, 139u8, 171u8, 144u8, 92u8, 33u8, 58u8, 23u8, 219u8, 164u8,
							121u8, 59u8, 209u8, 112u8, 244u8, 50u8, 8u8, 14u8, 244u8, 103u8, 125u8,
							120u8, 210u8, 16u8, 250u8, 54u8, 192u8, 72u8, 8u8, 219u8, 152u8,
						],
					)
				}
				#[doc = " Next epoch authorities."]
				pub fn next_authorities(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
						runtime_types::sp_consensus_babe::app::Public,
						::core::primitive::u64,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"NextAuthorities",
						vec![],
						[
							116u8, 95u8, 126u8, 199u8, 237u8, 90u8, 202u8, 227u8, 247u8, 56u8,
							201u8, 113u8, 239u8, 191u8, 151u8, 56u8, 156u8, 133u8, 61u8, 64u8,
							141u8, 26u8, 8u8, 95u8, 177u8, 255u8, 54u8, 223u8, 132u8, 74u8, 210u8,
							128u8,
						],
					)
				}
				#[doc = " Randomness under construction."]
				#[doc = ""]
				#[doc = " We make a trade-off between storage accesses and list length."]
				#[doc = " We store the under-construction randomness in segments of up to"]
				#[doc = " `UNDER_CONSTRUCTION_SEGMENT_LENGTH`."]
				#[doc = ""]
				#[doc = " Once a segment reaches this length, we begin the next one."]
				#[doc = " We reset all segments and return to `0` at the beginning of every"]
				#[doc = " epoch."]
				pub fn segment_index(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"SegmentIndex",
						vec![],
						[
							145u8, 91u8, 142u8, 240u8, 184u8, 94u8, 68u8, 52u8, 130u8, 3u8, 75u8,
							175u8, 155u8, 130u8, 66u8, 9u8, 150u8, 242u8, 123u8, 111u8, 124u8,
							241u8, 100u8, 128u8, 220u8, 133u8, 96u8, 227u8, 164u8, 241u8, 170u8,
							34u8,
						],
					)
				}
				#[doc = " TWOX-NOTE: `SegmentIndex` is an increasing integer, so this is okay."]
				pub fn under_construction(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						[::core::primitive::u8; 32usize],
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"UnderConstruction",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							120u8, 120u8, 59u8, 247u8, 50u8, 6u8, 220u8, 14u8, 2u8, 76u8, 203u8,
							244u8, 232u8, 144u8, 253u8, 191u8, 101u8, 35u8, 99u8, 85u8, 111u8,
							168u8, 31u8, 110u8, 187u8, 124u8, 72u8, 32u8, 43u8, 66u8, 8u8, 215u8,
						],
					)
				}
				#[doc = " TWOX-NOTE: `SegmentIndex` is an increasing integer, so this is okay."]
				pub fn under_construction_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						[::core::primitive::u8; 32usize],
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"UnderConstruction",
						Vec::new(),
						[
							120u8, 120u8, 59u8, 247u8, 50u8, 6u8, 220u8, 14u8, 2u8, 76u8, 203u8,
							244u8, 232u8, 144u8, 253u8, 191u8, 101u8, 35u8, 99u8, 85u8, 111u8,
							168u8, 31u8, 110u8, 187u8, 124u8, 72u8, 32u8, 43u8, 66u8, 8u8, 215u8,
						],
					)
				}
				#[doc = " Temporary value (cleared at block finalization) which is `Some`"]
				#[doc = " if per-block initialization has already been called for current block."]
				pub fn initialized(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::option::Option<runtime_types::sp_consensus_babe::digests::PreDigest>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"Initialized",
						vec![],
						[
							61u8, 100u8, 12u8, 43u8, 50u8, 166u8, 173u8, 130u8, 86u8, 36u8, 92u8,
							221u8, 44u8, 235u8, 241u8, 150u8, 231u8, 108u8, 15u8, 134u8, 12u8, 6u8,
							198u8, 102u8, 63u8, 69u8, 201u8, 171u8, 14u8, 135u8, 254u8, 239u8,
						],
					)
				}
				#[doc = " This field should always be populated during block processing unless"]
				#[doc = " secondary plain slots are enabled (which don't contain a VRF output)."]
				#[doc = ""]
				#[doc = " It is set in `on_finalize`, before it will contain the value from the last block."]
				pub fn author_vrf_randomness(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::option::Option<[::core::primitive::u8; 32usize]>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"AuthorVrfRandomness",
						vec![],
						[
							160u8, 157u8, 62u8, 48u8, 196u8, 136u8, 63u8, 132u8, 155u8, 183u8,
							91u8, 201u8, 146u8, 29u8, 192u8, 142u8, 168u8, 152u8, 197u8, 233u8,
							5u8, 25u8, 0u8, 154u8, 234u8, 180u8, 146u8, 132u8, 106u8, 164u8, 149u8,
							63u8,
						],
					)
				}
				#[doc = " The block numbers when the last and current epoch have started, respectively `N-1` and"]
				#[doc = " `N`."]
				#[doc = " NOTE: We track this is in order to annotate the block number when a given pool of"]
				#[doc = " entropy was fixed (i.e. it was known to chain observers). Since epochs are defined in"]
				#[doc = " slots, which may be skipped, the block numbers may not line up with the slot numbers."]
				pub fn epoch_start(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::core::primitive::u32, ::core::primitive::u32),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"EpochStart",
						vec![],
						[
							246u8, 69u8, 165u8, 217u8, 181u8, 138u8, 201u8, 64u8, 251u8, 121u8,
							50u8, 231u8, 221u8, 144u8, 225u8, 249u8, 42u8, 135u8, 31u8, 136u8,
							21u8, 160u8, 186u8, 148u8, 139u8, 232u8, 182u8, 121u8, 82u8, 110u8,
							14u8, 160u8,
						],
					)
				}
				#[doc = " How late the current block is compared to its parent."]
				#[doc = ""]
				#[doc = " This entry is populated as part of block execution and is cleaned up"]
				#[doc = " on block finalization. Querying this storage entry outside of block"]
				#[doc = " execution context should always yield zero."]
				pub fn lateness(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"Lateness",
						vec![],
						[
							229u8, 214u8, 133u8, 149u8, 32u8, 159u8, 26u8, 22u8, 252u8, 131u8,
							200u8, 191u8, 231u8, 176u8, 178u8, 127u8, 33u8, 212u8, 139u8, 220u8,
							157u8, 38u8, 4u8, 226u8, 204u8, 32u8, 55u8, 20u8, 205u8, 141u8, 29u8,
							87u8,
						],
					)
				}
				#[doc = " The configuration for the current epoch. Should never be `None` as it is initialized in"]
				#[doc = " genesis."]
				pub fn epoch_config(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_consensus_babe::BabeEpochConfiguration,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"EpochConfig",
						vec![],
						[
							23u8, 188u8, 70u8, 119u8, 36u8, 199u8, 230u8, 191u8, 131u8, 219u8,
							85u8, 201u8, 237u8, 70u8, 214u8, 149u8, 212u8, 94u8, 87u8, 87u8, 62u8,
							16u8, 46u8, 143u8, 73u8, 169u8, 42u8, 139u8, 157u8, 139u8, 190u8,
							166u8,
						],
					)
				}
				#[doc = " The configuration for the next epoch, `None` if the config will not change"]
				#[doc = " (you can fallback to `EpochConfig` instead in that case)."]
				pub fn next_epoch_config(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_consensus_babe::BabeEpochConfiguration,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"NextEpochConfig",
						vec![],
						[
							35u8, 132u8, 198u8, 33u8, 167u8, 69u8, 180u8, 215u8, 207u8, 40u8, 35u8,
							78u8, 167u8, 22u8, 32u8, 246u8, 111u8, 207u8, 88u8, 13u8, 28u8, 86u8,
							220u8, 102u8, 35u8, 105u8, 160u8, 163u8, 13u8, 99u8, 142u8, 69u8,
						],
					)
				}
				#[doc = " A list of the last 100 skipped epochs and the corresponding session index"]
				#[doc = " when the epoch was skipped."]
				#[doc = ""]
				#[doc = " This is only used for validating equivocation proofs. An equivocation proof"]
				#[doc = " must contains a key-ownership proof for a given session, therefore we need a"]
				#[doc = " way to tie together sessions and epoch indices, i.e. we need to validate that"]
				#[doc = " a validator was the owner of a given key on a given session, and what the"]
				#[doc = " active epoch index was during that session."]
				pub fn skipped_epochs(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						::core::primitive::u64,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Babe",
						"SkippedEpochs",
						vec![],
						[
							120u8, 167u8, 144u8, 97u8, 41u8, 216u8, 103u8, 90u8, 3u8, 86u8, 196u8,
							35u8, 160u8, 150u8, 144u8, 233u8, 128u8, 35u8, 119u8, 66u8, 6u8, 63u8,
							114u8, 140u8, 182u8, 228u8, 192u8, 30u8, 50u8, 145u8, 217u8, 108u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The amount of time, in slots, that each epoch should last."]
				#[doc = " NOTE: Currently it is not possible to change the epoch duration after"]
				#[doc = " the chain has started. Attempting to do so will brick block production."]
				pub fn epoch_duration(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"Babe",
						"EpochDuration",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
				#[doc = " The expected average block time at which BABE should be creating"]
				#[doc = " blocks. Since BABE is probabilistic it is not trivial to figure out"]
				#[doc = " what the expected average block time should be based on the slot"]
				#[doc = " duration and the security parameter `c` (where `1 - c` represents"]
				#[doc = " the probability of a slot being empty)."]
				pub fn expected_block_time(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"Babe",
						"ExpectedBlockTime",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
				#[doc = " Max number of authorities allowed"]
				pub fn max_authorities(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Babe",
						"MaxAuthorities",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod timestamp {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_timestamp::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Set {
					#[codec(compact)]
					pub now: ::core::primitive::u64,
				}
				impl ::subxt::blocks::StaticExtrinsic for Set {
					const PALLET: &'static str = "Timestamp";
					const CALL: &'static str = "set";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::set`]."]
				pub fn set(&self, now: ::core::primitive::u64) -> ::subxt::tx::Payload<types::Set> {
					::subxt::tx::Payload::new_static(
						"Timestamp",
						"set",
						types::Set { now },
						[
							37u8, 95u8, 49u8, 218u8, 24u8, 22u8, 0u8, 95u8, 72u8, 35u8, 155u8,
							199u8, 213u8, 54u8, 207u8, 22u8, 185u8, 193u8, 221u8, 70u8, 18u8,
							200u8, 4u8, 231u8, 195u8, 173u8, 6u8, 122u8, 11u8, 203u8, 231u8, 227u8,
						],
					)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Current time for the current block."]
				pub fn now(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Timestamp",
						"Now",
						vec![],
						[
							44u8, 50u8, 80u8, 30u8, 195u8, 146u8, 123u8, 238u8, 8u8, 163u8, 187u8,
							92u8, 61u8, 39u8, 51u8, 29u8, 173u8, 169u8, 217u8, 158u8, 85u8, 187u8,
							141u8, 26u8, 12u8, 115u8, 51u8, 11u8, 200u8, 244u8, 138u8, 152u8,
						],
					)
				}
				#[doc = " Did the timestamp get updated in this block?"]
				pub fn did_update(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Timestamp",
						"DidUpdate",
						vec![],
						[
							229u8, 175u8, 246u8, 102u8, 237u8, 158u8, 212u8, 229u8, 238u8, 214u8,
							205u8, 160u8, 164u8, 252u8, 195u8, 75u8, 139u8, 110u8, 22u8, 34u8,
							248u8, 204u8, 107u8, 46u8, 20u8, 200u8, 238u8, 167u8, 71u8, 41u8,
							214u8, 140u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The minimum period between blocks. Beware that this is different to the *expected*"]
				#[doc = " period that the block production apparatus provides. Your chosen consensus system will"]
				#[doc = " generally work with this to determine a sensible block time. e.g. For Aura, it will be"]
				#[doc = " double this period on default settings."]
				pub fn minimum_period(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"Timestamp",
						"MinimumPeriod",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
			}
		}
	}
	pub mod authorship {
		use super::root_mod;
		use super::runtime_types;
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Author of current block."]
				pub fn author(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Authorship",
						"Author",
						vec![],
						[
							247u8, 192u8, 118u8, 227u8, 47u8, 20u8, 203u8, 199u8, 216u8, 87u8,
							220u8, 50u8, 166u8, 61u8, 168u8, 213u8, 253u8, 62u8, 202u8, 199u8,
							61u8, 192u8, 237u8, 53u8, 22u8, 148u8, 164u8, 245u8, 99u8, 24u8, 146u8,
							18u8,
						],
					)
				}
			}
		}
	}
	pub mod indices {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_indices::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_indices::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Claim {
					pub index: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Claim {
					const PALLET: &'static str = "Indices";
					const CALL: &'static str = "claim";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Transfer {
					pub new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					pub index: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Transfer {
					const PALLET: &'static str = "Indices";
					const CALL: &'static str = "transfer";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Free {
					pub index: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Free {
					const PALLET: &'static str = "Indices";
					const CALL: &'static str = "free";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceTransfer {
					pub new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					pub index: ::core::primitive::u32,
					pub freeze: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceTransfer {
					const PALLET: &'static str = "Indices";
					const CALL: &'static str = "force_transfer";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Freeze {
					pub index: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Freeze {
					const PALLET: &'static str = "Indices";
					const CALL: &'static str = "freeze";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::claim`]."]
				pub fn claim(
					&self,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Claim> {
					::subxt::tx::Payload::new_static(
						"Indices",
						"claim",
						types::Claim { index },
						[
							146u8, 58u8, 246u8, 135u8, 59u8, 90u8, 3u8, 5u8, 140u8, 169u8, 232u8,
							195u8, 11u8, 107u8, 36u8, 141u8, 118u8, 174u8, 160u8, 160u8, 19u8,
							205u8, 177u8, 193u8, 18u8, 102u8, 115u8, 31u8, 72u8, 29u8, 91u8, 235u8,
						],
					)
				}
				#[doc = "See [`Pallet::transfer`]."]
				pub fn transfer(
					&self,
					new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Transfer> {
					::subxt::tx::Payload::new_static(
						"Indices",
						"transfer",
						types::Transfer { new, index },
						[
							139u8, 89u8, 163u8, 53u8, 141u8, 131u8, 135u8, 107u8, 0u8, 131u8, 53u8,
							202u8, 117u8, 99u8, 239u8, 205u8, 101u8, 97u8, 251u8, 247u8, 116u8,
							128u8, 220u8, 186u8, 151u8, 203u8, 82u8, 222u8, 199u8, 137u8, 41u8,
							199u8,
						],
					)
				}
				#[doc = "See [`Pallet::free`]."]
				pub fn free(
					&self,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Free> {
					::subxt::tx::Payload::new_static(
						"Indices",
						"free",
						types::Free { index },
						[
							241u8, 211u8, 234u8, 102u8, 189u8, 22u8, 209u8, 27u8, 8u8, 229u8, 80u8,
							227u8, 138u8, 252u8, 222u8, 111u8, 77u8, 201u8, 235u8, 51u8, 163u8,
							247u8, 13u8, 126u8, 216u8, 136u8, 57u8, 222u8, 56u8, 66u8, 215u8,
							244u8,
						],
					)
				}
				#[doc = "See [`Pallet::force_transfer`]."]
				pub fn force_transfer(
					&self,
					new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					index: ::core::primitive::u32,
					freeze: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::ForceTransfer> {
					::subxt::tx::Payload::new_static(
						"Indices",
						"force_transfer",
						types::ForceTransfer { new, index, freeze },
						[
							160u8, 55u8, 190u8, 50u8, 35u8, 79u8, 28u8, 117u8, 9u8, 2u8, 34u8,
							99u8, 247u8, 50u8, 77u8, 158u8, 156u8, 101u8, 90u8, 246u8, 129u8,
							106u8, 23u8, 142u8, 213u8, 88u8, 191u8, 232u8, 99u8, 139u8, 61u8,
							125u8,
						],
					)
				}
				#[doc = "See [`Pallet::freeze`]."]
				pub fn freeze(
					&self,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Freeze> {
					::subxt::tx::Payload::new_static(
						"Indices",
						"freeze",
						types::Freeze { index },
						[
							238u8, 215u8, 108u8, 156u8, 84u8, 240u8, 130u8, 229u8, 27u8, 132u8,
							93u8, 78u8, 2u8, 251u8, 43u8, 203u8, 2u8, 142u8, 147u8, 48u8, 92u8,
							101u8, 207u8, 24u8, 51u8, 16u8, 36u8, 229u8, 188u8, 129u8, 160u8,
							117u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_indices::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A account index was assigned."]
			pub struct IndexAssigned {
				pub who: ::subxt::utils::AccountId32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for IndexAssigned {
				const PALLET: &'static str = "Indices";
				const EVENT: &'static str = "IndexAssigned";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A account index has been freed up (unassigned)."]
			pub struct IndexFreed {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for IndexFreed {
				const PALLET: &'static str = "Indices";
				const EVENT: &'static str = "IndexFreed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A account index has been frozen to its current account ID."]
			pub struct IndexFrozen {
				pub index: ::core::primitive::u32,
				pub who: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for IndexFrozen {
				const PALLET: &'static str = "Indices";
				const EVENT: &'static str = "IndexFrozen";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The lookup from index to account."]
				pub fn accounts(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						::subxt::utils::AccountId32,
						::core::primitive::u128,
						::core::primitive::bool,
					),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Indices",
						"Accounts",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							48u8, 189u8, 43u8, 119u8, 32u8, 168u8, 28u8, 12u8, 245u8, 81u8, 119u8,
							182u8, 23u8, 201u8, 33u8, 147u8, 128u8, 171u8, 155u8, 134u8, 71u8,
							87u8, 100u8, 248u8, 107u8, 129u8, 36u8, 197u8, 220u8, 90u8, 11u8,
							238u8,
						],
					)
				}
				#[doc = " The lookup from index to account."]
				pub fn accounts_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						::subxt::utils::AccountId32,
						::core::primitive::u128,
						::core::primitive::bool,
					),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Indices",
						"Accounts",
						Vec::new(),
						[
							48u8, 189u8, 43u8, 119u8, 32u8, 168u8, 28u8, 12u8, 245u8, 81u8, 119u8,
							182u8, 23u8, 201u8, 33u8, 147u8, 128u8, 171u8, 155u8, 134u8, 71u8,
							87u8, 100u8, 248u8, 107u8, 129u8, 36u8, 197u8, 220u8, 90u8, 11u8,
							238u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The deposit needed for reserving an index."]
				pub fn deposit(&self) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Indices",
						"Deposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
			}
		}
	}
	pub mod balances {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_balances::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_balances::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct TransferAllowDeath {
					pub dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					#[codec(compact)]
					pub value: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferAllowDeath {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "transfer_allow_death";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetBalanceDeprecated {
					pub who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					#[codec(compact)]
					pub new_free: ::core::primitive::u128,
					#[codec(compact)]
					pub old_reserved: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetBalanceDeprecated {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "set_balance_deprecated";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceTransfer {
					pub source: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					pub dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					#[codec(compact)]
					pub value: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceTransfer {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "force_transfer";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct TransferKeepAlive {
					pub dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					#[codec(compact)]
					pub value: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferKeepAlive {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "transfer_keep_alive";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct TransferAll {
					pub dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					pub keep_alive: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferAll {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "transfer_all";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceUnreserve {
					pub who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					pub amount: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceUnreserve {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "force_unreserve";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct UpgradeAccounts {
					pub who: ::std::vec::Vec<::subxt::utils::AccountId32>,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpgradeAccounts {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "upgrade_accounts";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Transfer {
					pub dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					#[codec(compact)]
					pub value: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for Transfer {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "transfer";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceSetBalance {
					pub who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					#[codec(compact)]
					pub new_free: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceSetBalance {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "force_set_balance";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::transfer_allow_death`]."]
				pub fn transfer_allow_death(
					&self,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::TransferAllowDeath> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"transfer_allow_death",
						types::TransferAllowDeath { dest, value },
						[
							100u8, 112u8, 63u8, 118u8, 67u8, 47u8, 63u8, 58u8, 249u8, 180u8, 242u8,
							65u8, 237u8, 246u8, 118u8, 80u8, 181u8, 220u8, 5u8, 1u8, 144u8, 184u8,
							214u8, 24u8, 170u8, 165u8, 227u8, 142u8, 9u8, 43u8, 152u8, 101u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_balance_deprecated`]."]
				pub fn set_balance_deprecated(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					new_free: ::core::primitive::u128,
					old_reserved: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::SetBalanceDeprecated> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"set_balance_deprecated",
						types::SetBalanceDeprecated {
							who,
							new_free,
							old_reserved,
						},
						[
							135u8, 204u8, 148u8, 253u8, 76u8, 20u8, 79u8, 5u8, 133u8, 232u8, 10u8,
							79u8, 199u8, 52u8, 199u8, 42u8, 124u8, 55u8, 82u8, 71u8, 102u8, 81u8,
							46u8, 101u8, 38u8, 183u8, 219u8, 188u8, 227u8, 255u8, 136u8, 124u8,
						],
					)
				}
				#[doc = "See [`Pallet::force_transfer`]."]
				pub fn force_transfer(
					&self,
					source: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::ForceTransfer> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"force_transfer",
						types::ForceTransfer {
							source,
							dest,
							value,
						},
						[
							192u8, 208u8, 171u8, 116u8, 91u8, 20u8, 177u8, 202u8, 113u8, 195u8,
							44u8, 49u8, 140u8, 190u8, 168u8, 41u8, 165u8, 202u8, 68u8, 127u8,
							247u8, 183u8, 126u8, 38u8, 124u8, 41u8, 13u8, 11u8, 69u8, 32u8, 35u8,
							12u8,
						],
					)
				}
				#[doc = "See [`Pallet::transfer_keep_alive`]."]
				pub fn transfer_keep_alive(
					&self,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::TransferKeepAlive> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"transfer_keep_alive",
						types::TransferKeepAlive { dest, value },
						[
							186u8, 56u8, 39u8, 71u8, 205u8, 98u8, 251u8, 252u8, 106u8, 6u8, 92u8,
							217u8, 254u8, 186u8, 113u8, 196u8, 114u8, 248u8, 54u8, 226u8, 53u8,
							73u8, 6u8, 66u8, 96u8, 54u8, 117u8, 23u8, 34u8, 200u8, 84u8, 227u8,
						],
					)
				}
				#[doc = "See [`Pallet::transfer_all`]."]
				pub fn transfer_all(
					&self,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::TransferAll> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"transfer_all",
						types::TransferAll { dest, keep_alive },
						[
							39u8, 244u8, 99u8, 182u8, 181u8, 159u8, 95u8, 217u8, 8u8, 210u8, 113u8,
							254u8, 171u8, 199u8, 93u8, 146u8, 209u8, 229u8, 55u8, 97u8, 131u8,
							226u8, 155u8, 41u8, 199u8, 145u8, 177u8, 137u8, 198u8, 241u8, 90u8,
							20u8,
						],
					)
				}
				#[doc = "See [`Pallet::force_unreserve`]."]
				pub fn force_unreserve(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::ForceUnreserve> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"force_unreserve",
						types::ForceUnreserve { who, amount },
						[
							153u8, 166u8, 64u8, 196u8, 251u8, 238u8, 79u8, 171u8, 169u8, 183u8,
							68u8, 220u8, 117u8, 237u8, 231u8, 156u8, 6u8, 186u8, 196u8, 238u8,
							195u8, 16u8, 84u8, 236u8, 40u8, 6u8, 228u8, 211u8, 19u8, 248u8, 171u8,
							137u8,
						],
					)
				}
				#[doc = "See [`Pallet::upgrade_accounts`]."]
				pub fn upgrade_accounts(
					&self,
					who: ::std::vec::Vec<::subxt::utils::AccountId32>,
				) -> ::subxt::tx::Payload<types::UpgradeAccounts> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"upgrade_accounts",
						types::UpgradeAccounts { who },
						[
							66u8, 200u8, 179u8, 104u8, 65u8, 2u8, 101u8, 56u8, 130u8, 161u8, 224u8,
							233u8, 255u8, 124u8, 70u8, 122u8, 8u8, 49u8, 103u8, 178u8, 68u8, 47u8,
							214u8, 166u8, 217u8, 116u8, 178u8, 50u8, 212u8, 164u8, 98u8, 226u8,
						],
					)
				}
				#[doc = "See [`Pallet::transfer`]."]
				pub fn transfer(
					&self,
					dest: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::Transfer> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"transfer",
						types::Transfer { dest, value },
						[
							78u8, 137u8, 180u8, 250u8, 225u8, 152u8, 150u8, 139u8, 30u8, 4u8, 57u8,
							241u8, 135u8, 108u8, 67u8, 239u8, 65u8, 13u8, 10u8, 205u8, 125u8,
							208u8, 237u8, 60u8, 18u8, 13u8, 232u8, 47u8, 47u8, 151u8, 79u8, 23u8,
						],
					)
				}
				#[doc = "See [`Pallet::force_set_balance`]."]
				pub fn force_set_balance(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					new_free: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::ForceSetBalance> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"force_set_balance",
						types::ForceSetBalance { who, new_free },
						[
							171u8, 138u8, 53u8, 85u8, 202u8, 35u8, 151u8, 115u8, 143u8, 67u8, 81u8,
							25u8, 250u8, 108u8, 179u8, 46u8, 194u8, 23u8, 169u8, 223u8, 6u8, 179u8,
							240u8, 4u8, 26u8, 9u8, 229u8, 29u8, 248u8, 6u8, 23u8, 232u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_balances::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account was created with some free balance."]
			pub struct Endowed {
				pub account: ::subxt::utils::AccountId32,
				pub free_balance: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Endowed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Endowed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
			#[doc = "resulting in an outright loss."]
			pub struct DustLost {
				pub account: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for DustLost {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "DustLost";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Transfer succeeded."]
			pub struct Transfer {
				pub from: ::subxt::utils::AccountId32,
				pub to: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Transfer {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Transfer";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A balance was set by root."]
			pub struct BalanceSet {
				pub who: ::subxt::utils::AccountId32,
				pub free: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for BalanceSet {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "BalanceSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was reserved (moved from free to reserved)."]
			pub struct Reserved {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Reserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Reserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was unreserved (moved from reserved to free)."]
			pub struct Unreserved {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Unreserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Unreserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was moved from the reserve of the first account to the second account."]
			#[doc = "Final argument indicates the destination balance type."]
			pub struct ReserveRepatriated {
				pub from: ::subxt::utils::AccountId32,
				pub to: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
				pub destination_status:
					runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
			}
			impl ::subxt::events::StaticEvent for ReserveRepatriated {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "ReserveRepatriated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was deposited (e.g. for transaction fees)."]
			pub struct Deposit {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Deposit {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Deposit";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
			pub struct Withdraw {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Withdraw {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Withdraw";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
			pub struct Slashed {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Slashed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Slashed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was minted into an account."]
			pub struct Minted {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Minted {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Minted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was burned from an account."]
			pub struct Burned {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Burned {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Burned";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was suspended from an account (it can be restored later)."]
			pub struct Suspended {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Suspended {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Suspended";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was restored into an account."]
			pub struct Restored {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Restored {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Restored";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account was upgraded."]
			pub struct Upgraded {
				pub who: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Upgraded {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Upgraded";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
			pub struct Issued {
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Issued {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Issued";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
			pub struct Rescinded {
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Rescinded {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Rescinded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was locked."]
			pub struct Locked {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Locked {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Locked";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was unlocked."]
			pub struct Unlocked {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Unlocked {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Unlocked";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was frozen."]
			pub struct Frozen {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Frozen {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Frozen";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was thawed."]
			pub struct Thawed {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Thawed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Thawed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The total units issued in the system."]
				pub fn total_issuance(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"TotalIssuance",
						vec![],
						[
							116u8, 70u8, 119u8, 194u8, 69u8, 37u8, 116u8, 206u8, 171u8, 70u8,
							171u8, 210u8, 226u8, 111u8, 184u8, 204u8, 206u8, 11u8, 68u8, 72u8,
							255u8, 19u8, 194u8, 11u8, 27u8, 194u8, 81u8, 204u8, 59u8, 224u8, 202u8,
							185u8,
						],
					)
				}
				#[doc = " The total units of outstanding deactivated balance in the system."]
				pub fn inactive_issuance(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"InactiveIssuance",
						vec![],
						[
							212u8, 185u8, 19u8, 50u8, 250u8, 72u8, 173u8, 50u8, 4u8, 104u8, 161u8,
							249u8, 77u8, 247u8, 204u8, 248u8, 11u8, 18u8, 57u8, 4u8, 82u8, 110u8,
							30u8, 216u8, 16u8, 37u8, 87u8, 67u8, 189u8, 235u8, 214u8, 155u8,
						],
					)
				}
				#[doc = " The Balances pallet example of storing the balance of an account."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " You can also store the balance of an account in the `System` pallet."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "   type AccountStore = System"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
				#[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
				#[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
				#[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Account",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							47u8, 253u8, 83u8, 165u8, 18u8, 176u8, 62u8, 239u8, 78u8, 85u8, 231u8,
							235u8, 157u8, 145u8, 251u8, 35u8, 225u8, 171u8, 82u8, 167u8, 68u8,
							206u8, 28u8, 169u8, 8u8, 93u8, 169u8, 101u8, 180u8, 206u8, 231u8,
							143u8,
						],
					)
				}
				#[doc = " The Balances pallet example of storing the balance of an account."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " You can also store the balance of an account in the `System` pallet."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "   type AccountStore = System"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
				#[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
				#[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
				#[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
				pub fn account_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Account",
						Vec::new(),
						[
							47u8, 253u8, 83u8, 165u8, 18u8, 176u8, 62u8, 239u8, 78u8, 85u8, 231u8,
							235u8, 157u8, 145u8, 251u8, 35u8, 225u8, 171u8, 82u8, 167u8, 68u8,
							206u8, 28u8, 169u8, 8u8, 93u8, 169u8, 101u8, 180u8, 206u8, 231u8,
							143u8,
						],
					)
				}
				#[doc = " Any liquidity locks on some account balances."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				pub fn locks(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
						runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Locks",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							44u8, 44u8, 48u8, 20u8, 121u8, 168u8, 200u8, 87u8, 205u8, 172u8, 111u8,
							208u8, 62u8, 243u8, 225u8, 223u8, 181u8, 36u8, 197u8, 9u8, 52u8, 182u8,
							113u8, 55u8, 126u8, 164u8, 82u8, 209u8, 151u8, 126u8, 186u8, 85u8,
						],
					)
				}
				#[doc = " Any liquidity locks on some account balances."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				pub fn locks_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
						runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Locks",
						Vec::new(),
						[
							44u8, 44u8, 48u8, 20u8, 121u8, 168u8, 200u8, 87u8, 205u8, 172u8, 111u8,
							208u8, 62u8, 243u8, 225u8, 223u8, 181u8, 36u8, 197u8, 9u8, 52u8, 182u8,
							113u8, 55u8, 126u8, 164u8, 82u8, 209u8, 151u8, 126u8, 186u8, 85u8,
						],
					)
				}
				#[doc = " Named reserves on some account balances."]
				pub fn reserves(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::ReserveData<
							[::core::primitive::u8; 8usize],
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Reserves",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							192u8, 99u8, 91u8, 129u8, 195u8, 73u8, 153u8, 126u8, 82u8, 52u8, 56u8,
							85u8, 105u8, 178u8, 113u8, 101u8, 229u8, 37u8, 242u8, 174u8, 166u8,
							244u8, 68u8, 173u8, 14u8, 225u8, 172u8, 70u8, 181u8, 211u8, 165u8,
							134u8,
						],
					)
				}
				#[doc = " Named reserves on some account balances."]
				pub fn reserves_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::ReserveData<
							[::core::primitive::u8; 8usize],
							::core::primitive::u128,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Reserves",
						Vec::new(),
						[
							192u8, 99u8, 91u8, 129u8, 195u8, 73u8, 153u8, 126u8, 82u8, 52u8, 56u8,
							85u8, 105u8, 178u8, 113u8, 101u8, 229u8, 37u8, 242u8, 174u8, 166u8,
							244u8, 68u8, 173u8, 14u8, 225u8, 172u8, 70u8, 181u8, 211u8, 165u8,
							134u8,
						],
					)
				}
				#[doc = " Holds on account balances."]
				pub fn holds(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::IdAmount<
							runtime_types::da_runtime::RuntimeHoldReason,
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Holds",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							37u8, 176u8, 2u8, 18u8, 109u8, 26u8, 66u8, 81u8, 28u8, 104u8, 149u8,
							117u8, 119u8, 114u8, 196u8, 35u8, 172u8, 155u8, 66u8, 195u8, 98u8,
							37u8, 134u8, 22u8, 106u8, 221u8, 215u8, 97u8, 25u8, 28u8, 21u8, 206u8,
						],
					)
				}
				#[doc = " Holds on account balances."]
				pub fn holds_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::IdAmount<
							runtime_types::da_runtime::RuntimeHoldReason,
							::core::primitive::u128,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Holds",
						Vec::new(),
						[
							37u8, 176u8, 2u8, 18u8, 109u8, 26u8, 66u8, 81u8, 28u8, 104u8, 149u8,
							117u8, 119u8, 114u8, 196u8, 35u8, 172u8, 155u8, 66u8, 195u8, 98u8,
							37u8, 134u8, 22u8, 106u8, 221u8, 215u8, 97u8, 25u8, 28u8, 21u8, 206u8,
						],
					)
				}
				#[doc = " Freeze locks on account balances."]
				pub fn freezes(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::IdAmount<
							(),
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Freezes",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
							112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
							163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
						],
					)
				}
				#[doc = " Freeze locks on account balances."]
				pub fn freezes_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::IdAmount<
							(),
							::core::primitive::u128,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Freezes",
						Vec::new(),
						[
							69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
							112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
							163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The minimum amount required to keep an account open. MUST BE GREATER THAN ZERO!"]
				#[doc = ""]
				#[doc = " If you *really* need it to be zero, you can enable the feature `insecure_zero_ed` for"]
				#[doc = " this pallet. However, you do so at your own risk: this will open up a major DoS vector."]
				#[doc = " In case you have multiple sources of provider references, you may also get unexpected"]
				#[doc = " behaviour if you set this to zero."]
				#[doc = ""]
				#[doc = " Bottom line: Do yourself a favour and make it at least one!"]
				pub fn existential_deposit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Balances",
						"ExistentialDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The maximum number of locks that should exist on an account."]
				#[doc = " Not strictly enforced, but used for weight estimation."]
				pub fn max_locks(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Balances",
						"MaxLocks",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of named reserves that can exist on an account."]
				pub fn max_reserves(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Balances",
						"MaxReserves",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of holds that can exist on an account at any time."]
				pub fn max_holds(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Balances",
						"MaxHolds",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of individual freeze locks that can exist on an account at any time."]
				pub fn max_freezes(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Balances",
						"MaxFreezes",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod transaction_payment {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
			#[doc = "has been paid by `who`."]
			pub struct TransactionFeePaid {
				pub who: ::subxt::utils::AccountId32,
				pub actual_fee: ::core::primitive::u128,
				pub tip: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for TransactionFeePaid {
				const PALLET: &'static str = "TransactionPayment";
				const EVENT: &'static str = "TransactionFeePaid";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn next_fee_multiplier(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::fixed_point::FixedU128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TransactionPayment",
						"NextFeeMultiplier",
						vec![],
						[
							247u8, 39u8, 81u8, 170u8, 225u8, 226u8, 82u8, 147u8, 34u8, 113u8,
							147u8, 213u8, 59u8, 80u8, 139u8, 35u8, 36u8, 196u8, 152u8, 19u8, 9u8,
							159u8, 176u8, 79u8, 249u8, 201u8, 170u8, 1u8, 129u8, 79u8, 146u8,
							197u8,
						],
					)
				}
				pub fn storage_version(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_transaction_payment::Releases,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TransactionPayment",
						"StorageVersion",
						vec![],
						[
							105u8, 243u8, 158u8, 241u8, 159u8, 231u8, 253u8, 6u8, 4u8, 32u8, 85u8,
							178u8, 126u8, 31u8, 203u8, 134u8, 154u8, 38u8, 122u8, 155u8, 150u8,
							251u8, 174u8, 15u8, 74u8, 134u8, 216u8, 244u8, 168u8, 175u8, 158u8,
							144u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " A fee mulitplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
				#[doc = " `priority`"]
				#[doc = ""]
				#[doc = " This value is multipled by the `final_fee` to obtain a \"virtual tip\" that is later"]
				#[doc = " added to a tip component in regular `priority` calculations."]
				#[doc = " It means that a `Normal` transaction can front-run a similarly-sized `Operational`"]
				#[doc = " extrinsic (with no tip), by including a tip value greater than the virtual tip."]
				#[doc = ""]
				#[doc = " ```rust,ignore"]
				#[doc = " // For `Normal`"]
				#[doc = " let priority = priority_calc(tip);"]
				#[doc = ""]
				#[doc = " // For `Operational`"]
				#[doc = " let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;"]
				#[doc = " let priority = priority_calc(tip + virtual_tip);"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " Note that since we use `final_fee` the multiplier applies also to the regular `tip`"]
				#[doc = " sent with the transaction. So, not only does the transaction get a priority bump based"]
				#[doc = " on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`"]
				#[doc = " transactions."]
				pub fn operational_fee_multiplier(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u8> {
					::subxt::constants::Address::new_static(
						"TransactionPayment",
						"OperationalFeeMultiplier",
						[
							141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
							28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
							114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
							165u8,
						],
					)
				}
			}
		}
	}
	pub mod election_provider_multi_phase {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Error of the pallet that can be returned in response to dispatches."]
		pub type Error = runtime_types::pallet_election_provider_multi_phase::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_election_provider_multi_phase::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SubmitUnsigned {
					pub raw_solution: ::std::boxed::Box<
						runtime_types::pallet_election_provider_multi_phase::RawSolution<
							runtime_types::da_runtime::constants::staking::NposSolution16,
						>,
					>,
					pub witness:
						runtime_types::pallet_election_provider_multi_phase::SolutionOrSnapshotSize,
				}
				impl ::subxt::blocks::StaticExtrinsic for SubmitUnsigned {
					const PALLET: &'static str = "ElectionProviderMultiPhase";
					const CALL: &'static str = "submit_unsigned";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetMinimumUntrustedScore {
					pub maybe_next_score:
						::core::option::Option<runtime_types::sp_npos_elections::ElectionScore>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetMinimumUntrustedScore {
					const PALLET: &'static str = "ElectionProviderMultiPhase";
					const CALL: &'static str = "set_minimum_untrusted_score";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetEmergencyElectionResult {
					pub supports: ::std::vec::Vec<(
						::subxt::utils::AccountId32,
						runtime_types::sp_npos_elections::Support<::subxt::utils::AccountId32>,
					)>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetEmergencyElectionResult {
					const PALLET: &'static str = "ElectionProviderMultiPhase";
					const CALL: &'static str = "set_emergency_election_result";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Submit {
					pub raw_solution: ::std::boxed::Box<
						runtime_types::pallet_election_provider_multi_phase::RawSolution<
							runtime_types::da_runtime::constants::staking::NposSolution16,
						>,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Submit {
					const PALLET: &'static str = "ElectionProviderMultiPhase";
					const CALL: &'static str = "submit";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct GovernanceFallback {
					pub maybe_max_voters: ::core::option::Option<::core::primitive::u32>,
					pub maybe_max_targets: ::core::option::Option<::core::primitive::u32>,
				}
				impl ::subxt::blocks::StaticExtrinsic for GovernanceFallback {
					const PALLET: &'static str = "ElectionProviderMultiPhase";
					const CALL: &'static str = "governance_fallback";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::submit_unsigned`]."]
				pub fn submit_unsigned(
					&self,
					raw_solution: runtime_types::pallet_election_provider_multi_phase::RawSolution<
						runtime_types::da_runtime::constants::staking::NposSolution16,
					>,
					witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize,
				) -> ::subxt::tx::Payload<types::SubmitUnsigned> {
					::subxt::tx::Payload::new_static(
						"ElectionProviderMultiPhase",
						"submit_unsigned",
						types::SubmitUnsigned {
							raw_solution: ::std::boxed::Box::new(raw_solution),
							witness,
						},
						[
							34u8, 115u8, 43u8, 180u8, 202u8, 212u8, 42u8, 17u8, 187u8, 233u8, 54u8,
							206u8, 238u8, 239u8, 35u8, 240u8, 136u8, 197u8, 117u8, 113u8, 213u8,
							46u8, 94u8, 47u8, 84u8, 186u8, 177u8, 61u8, 3u8, 202u8, 2u8, 186u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_minimum_untrusted_score`]."]
				pub fn set_minimum_untrusted_score(
					&self,
					maybe_next_score: ::core::option::Option<
						runtime_types::sp_npos_elections::ElectionScore,
					>,
				) -> ::subxt::tx::Payload<types::SetMinimumUntrustedScore> {
					::subxt::tx::Payload::new_static(
						"ElectionProviderMultiPhase",
						"set_minimum_untrusted_score",
						types::SetMinimumUntrustedScore { maybe_next_score },
						[
							36u8, 32u8, 197u8, 96u8, 189u8, 98u8, 96u8, 138u8, 84u8, 99u8, 235u8,
							44u8, 103u8, 25u8, 118u8, 194u8, 166u8, 158u8, 212u8, 36u8, 243u8,
							86u8, 202u8, 231u8, 189u8, 226u8, 21u8, 112u8, 20u8, 163u8, 229u8,
							240u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_emergency_election_result`]."]
				pub fn set_emergency_election_result(
					&self,
					supports: ::std::vec::Vec<(
						::subxt::utils::AccountId32,
						runtime_types::sp_npos_elections::Support<::subxt::utils::AccountId32>,
					)>,
				) -> ::subxt::tx::Payload<types::SetEmergencyElectionResult> {
					::subxt::tx::Payload::new_static(
						"ElectionProviderMultiPhase",
						"set_emergency_election_result",
						types::SetEmergencyElectionResult { supports },
						[
							158u8, 35u8, 6u8, 145u8, 37u8, 239u8, 101u8, 90u8, 121u8, 123u8, 240u8,
							131u8, 154u8, 13u8, 111u8, 120u8, 146u8, 151u8, 203u8, 125u8, 115u8,
							255u8, 58u8, 154u8, 177u8, 204u8, 140u8, 87u8, 9u8, 63u8, 146u8, 209u8,
						],
					)
				}
				#[doc = "See [`Pallet::submit`]."]
				pub fn submit(
					&self,
					raw_solution: runtime_types::pallet_election_provider_multi_phase::RawSolution<
						runtime_types::da_runtime::constants::staking::NposSolution16,
					>,
				) -> ::subxt::tx::Payload<types::Submit> {
					::subxt::tx::Payload::new_static(
						"ElectionProviderMultiPhase",
						"submit",
						types::Submit {
							raw_solution: ::std::boxed::Box::new(raw_solution),
						},
						[
							55u8, 153u8, 215u8, 21u8, 19u8, 192u8, 199u8, 19u8, 145u8, 27u8, 54u8,
							128u8, 23u8, 3u8, 255u8, 87u8, 27u8, 75u8, 248u8, 145u8, 238u8, 75u8,
							204u8, 173u8, 71u8, 252u8, 29u8, 71u8, 45u8, 143u8, 179u8, 154u8,
						],
					)
				}
				#[doc = "See [`Pallet::governance_fallback`]."]
				pub fn governance_fallback(
					&self,
					maybe_max_voters: ::core::option::Option<::core::primitive::u32>,
					maybe_max_targets: ::core::option::Option<::core::primitive::u32>,
				) -> ::subxt::tx::Payload<types::GovernanceFallback> {
					::subxt::tx::Payload::new_static(
						"ElectionProviderMultiPhase",
						"governance_fallback",
						types::GovernanceFallback {
							maybe_max_voters,
							maybe_max_targets,
						},
						[
							168u8, 109u8, 243u8, 125u8, 188u8, 177u8, 251u8, 179u8, 158u8, 246u8,
							179u8, 247u8, 87u8, 217u8, 190u8, 107u8, 207u8, 249u8, 204u8, 27u8,
							166u8, 49u8, 135u8, 71u8, 88u8, 142u8, 58u8, 206u8, 137u8, 142u8, 75u8,
							127u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_election_provider_multi_phase::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A solution was stored with the given compute."]
			#[doc = ""]
			#[doc = "The `origin` indicates the origin of the solution. If `origin` is `Some(AccountId)`,"]
			#[doc = "the stored solution was submited in the signed phase by a miner with the `AccountId`."]
			#[doc = "Otherwise, the solution was stored either during the unsigned phase or by"]
			#[doc = "`T::ForceOrigin`. The `bool` is `true` when a previous solution was ejected to make"]
			#[doc = "room for this one."]
			pub struct SolutionStored {
				pub compute: runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
				pub origin: ::core::option::Option<::subxt::utils::AccountId32>,
				pub prev_ejected: ::core::primitive::bool,
			}
			impl ::subxt::events::StaticEvent for SolutionStored {
				const PALLET: &'static str = "ElectionProviderMultiPhase";
				const EVENT: &'static str = "SolutionStored";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The election has been finalized, with the given computation and score."]
			pub struct ElectionFinalized {
				pub compute: runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
				pub score: runtime_types::sp_npos_elections::ElectionScore,
			}
			impl ::subxt::events::StaticEvent for ElectionFinalized {
				const PALLET: &'static str = "ElectionProviderMultiPhase";
				const EVENT: &'static str = "ElectionFinalized";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An election failed."]
			#[doc = ""]
			#[doc = "Not much can be said about which computes failed in the process."]
			pub struct ElectionFailed;
			impl ::subxt::events::StaticEvent for ElectionFailed {
				const PALLET: &'static str = "ElectionProviderMultiPhase";
				const EVENT: &'static str = "ElectionFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account has been rewarded for their signed submission being finalized."]
			pub struct Rewarded {
				pub account: ::subxt::utils::AccountId32,
				pub value: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Rewarded {
				const PALLET: &'static str = "ElectionProviderMultiPhase";
				const EVENT: &'static str = "Rewarded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account has been slashed for submitting an invalid signed submission."]
			pub struct Slashed {
				pub account: ::subxt::utils::AccountId32,
				pub value: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Slashed {
				const PALLET: &'static str = "ElectionProviderMultiPhase";
				const EVENT: &'static str = "Slashed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "There was a phase transition in a given round."]
			pub struct PhaseTransitioned {
				pub from: runtime_types::pallet_election_provider_multi_phase::Phase<
					::core::primitive::u32,
				>,
				pub to: runtime_types::pallet_election_provider_multi_phase::Phase<
					::core::primitive::u32,
				>,
				pub round: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for PhaseTransitioned {
				const PALLET: &'static str = "ElectionProviderMultiPhase";
				const EVENT: &'static str = "PhaseTransitioned";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Internal counter for the number of rounds."]
				#[doc = ""]
				#[doc = " This is useful for de-duplication of transactions submitted to the pool, and general"]
				#[doc = " diagnostics of the pallet."]
				#[doc = ""]
				#[doc = " This is merely incremented once per every time that an upstream `elect` is called."]
				pub fn round(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ElectionProviderMultiPhase",
						"Round",
						vec![],
						[
							37u8, 2u8, 47u8, 240u8, 18u8, 213u8, 214u8, 74u8, 57u8, 4u8, 103u8,
							253u8, 45u8, 17u8, 123u8, 203u8, 173u8, 170u8, 234u8, 109u8, 139u8,
							143u8, 216u8, 3u8, 161u8, 5u8, 0u8, 106u8, 181u8, 214u8, 170u8, 105u8,
						],
					)
				}
				#[doc = " Current phase."]
				pub fn current_phase(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_election_provider_multi_phase::Phase<
						::core::primitive::u32,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ElectionProviderMultiPhase",
						"CurrentPhase",
						vec![],
						[
							230u8, 7u8, 51u8, 158u8, 77u8, 36u8, 148u8, 175u8, 138u8, 205u8, 195u8,
							236u8, 66u8, 148u8, 0u8, 77u8, 160u8, 249u8, 128u8, 58u8, 189u8, 48u8,
							195u8, 198u8, 115u8, 251u8, 13u8, 206u8, 163u8, 180u8, 108u8, 10u8,
						],
					)
				}
				#[doc = " Current best solution, signed or unsigned, queued to be returned upon `elect`."]
				#[doc = ""]
				#[doc = " Always sorted by score."]
				pub fn queued_solution(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_election_provider_multi_phase::ReadySolution,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ElectionProviderMultiPhase",
						"QueuedSolution",
						vec![],
						[
							64u8, 237u8, 221u8, 29u8, 144u8, 141u8, 147u8, 4u8, 46u8, 239u8, 34u8,
							242u8, 164u8, 69u8, 108u8, 145u8, 95u8, 167u8, 34u8, 211u8, 103u8,
							165u8, 183u8, 193u8, 245u8, 226u8, 140u8, 50u8, 176u8, 127u8, 108u8,
							171u8,
						],
					)
				}
				#[doc = " Snapshot data of the round."]
				#[doc = ""]
				#[doc = " This is created at the beginning of the signed phase and cleared upon calling `elect`."]
				pub fn snapshot(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_election_provider_multi_phase::RoundSnapshot<
						::subxt::utils::AccountId32,
						(
							::subxt::utils::AccountId32,
							::core::primitive::u64,
							runtime_types::bounded_collections::bounded_vec::BoundedVec<
								::subxt::utils::AccountId32,
							>,
						),
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ElectionProviderMultiPhase",
						"Snapshot",
						vec![],
						[
							180u8, 77u8, 217u8, 249u8, 212u8, 99u8, 36u8, 26u8, 237u8, 4u8, 94u8,
							80u8, 160u8, 6u8, 194u8, 98u8, 174u8, 153u8, 127u8, 124u8, 109u8,
							188u8, 143u8, 151u8, 51u8, 200u8, 133u8, 66u8, 68u8, 226u8, 124u8,
							158u8,
						],
					)
				}
				#[doc = " Desired number of targets to elect for this round."]
				#[doc = ""]
				#[doc = " Only exists when [`Snapshot`] is present."]
				pub fn desired_targets(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ElectionProviderMultiPhase",
						"DesiredTargets",
						vec![],
						[
							67u8, 241u8, 33u8, 113u8, 62u8, 173u8, 233u8, 76u8, 99u8, 12u8, 61u8,
							237u8, 21u8, 252u8, 39u8, 37u8, 86u8, 167u8, 173u8, 53u8, 238u8, 172u8,
							97u8, 59u8, 27u8, 164u8, 163u8, 76u8, 140u8, 37u8, 159u8, 250u8,
						],
					)
				}
				#[doc = " The metadata of the [`RoundSnapshot`]"]
				#[doc = ""]
				#[doc = " Only exists when [`Snapshot`] is present."]
				pub fn snapshot_metadata(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_election_provider_multi_phase::SolutionOrSnapshotSize,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ElectionProviderMultiPhase",
						"SnapshotMetadata",
						vec![],
						[
							14u8, 189u8, 135u8, 84u8, 238u8, 133u8, 76u8, 176u8, 181u8, 185u8,
							111u8, 102u8, 181u8, 14u8, 172u8, 86u8, 188u8, 139u8, 73u8, 192u8,
							203u8, 117u8, 39u8, 119u8, 108u8, 225u8, 163u8, 36u8, 91u8, 30u8, 0u8,
							196u8,
						],
					)
				}
				#[doc = " The next index to be assigned to an incoming signed submission."]
				#[doc = ""]
				#[doc = " Every accepted submission is assigned a unique index; that index is bound to that particular"]
				#[doc = " submission for the duration of the election. On election finalization, the next index is"]
				#[doc = " reset to 0."]
				#[doc = ""]
				#[doc = " We can't just use `SignedSubmissionIndices.len()`, because that's a bounded set; past its"]
				#[doc = " capacity, it will simply saturate. We can't just iterate over `SignedSubmissionsMap`,"]
				#[doc = " because iteration is slow. Instead, we store the value here."]
				pub fn signed_submission_next_index(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ElectionProviderMultiPhase",
						"SignedSubmissionNextIndex",
						vec![],
						[
							188u8, 126u8, 77u8, 166u8, 42u8, 81u8, 12u8, 239u8, 195u8, 16u8, 132u8,
							178u8, 217u8, 158u8, 28u8, 19u8, 201u8, 148u8, 47u8, 105u8, 178u8,
							115u8, 17u8, 78u8, 71u8, 178u8, 205u8, 171u8, 71u8, 52u8, 194u8, 82u8,
						],
					)
				}
				#[doc = " A sorted, bounded vector of `(score, block_number, index)`, where each `index` points to a"]
				#[doc = " value in `SignedSubmissions`."]
				#[doc = ""]
				#[doc = " We never need to process more than a single signed submission at a time. Signed submissions"]
				#[doc = " can be quite large, so we're willing to pay the cost of multiple database accesses to access"]
				#[doc = " them one at a time instead of reading and decoding all of them at once."]
				pub fn signed_submission_indices(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						runtime_types::sp_npos_elections::ElectionScore,
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ElectionProviderMultiPhase",
						"SignedSubmissionIndices",
						vec![],
						[
							203u8, 96u8, 121u8, 1u8, 24u8, 150u8, 185u8, 93u8, 129u8, 63u8, 52u8,
							163u8, 67u8, 45u8, 100u8, 11u8, 254u8, 224u8, 18u8, 1u8, 133u8, 246u8,
							125u8, 211u8, 93u8, 99u8, 194u8, 105u8, 176u8, 162u8, 238u8, 181u8,
						],
					)
				}
				#[doc = " Unchecked, signed solutions."]
				#[doc = ""]
				#[doc = " Together with `SubmissionIndices`, this stores a bounded set of `SignedSubmissions` while"]
				#[doc = " allowing us to keep only a single one in memory at a time."]
				#[doc = ""]
				#[doc = " Twox note: the key of the map is an auto-incrementing index which users cannot inspect or"]
				#[doc = " affect; we shouldn't need a cryptographically secure hasher."]
				pub fn signed_submissions_map(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_election_provider_multi_phase::signed::SignedSubmission<
						::subxt::utils::AccountId32,
						::core::primitive::u128,
						runtime_types::da_runtime::constants::staking::NposSolution16,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"ElectionProviderMultiPhase",
						"SignedSubmissionsMap",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							79u8, 183u8, 109u8, 221u8, 2u8, 64u8, 197u8, 162u8, 221u8, 170u8,
							140u8, 136u8, 205u8, 111u8, 8u8, 179u8, 166u8, 104u8, 74u8, 219u8,
							202u8, 123u8, 31u8, 129u8, 207u8, 58u8, 241u8, 91u8, 147u8, 112u8,
							162u8, 105u8,
						],
					)
				}
				#[doc = " Unchecked, signed solutions."]
				#[doc = ""]
				#[doc = " Together with `SubmissionIndices`, this stores a bounded set of `SignedSubmissions` while"]
				#[doc = " allowing us to keep only a single one in memory at a time."]
				#[doc = ""]
				#[doc = " Twox note: the key of the map is an auto-incrementing index which users cannot inspect or"]
				#[doc = " affect; we shouldn't need a cryptographically secure hasher."]
				pub fn signed_submissions_map_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_election_provider_multi_phase::signed::SignedSubmission<
						::subxt::utils::AccountId32,
						::core::primitive::u128,
						runtime_types::da_runtime::constants::staking::NposSolution16,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"ElectionProviderMultiPhase",
						"SignedSubmissionsMap",
						Vec::new(),
						[
							79u8, 183u8, 109u8, 221u8, 2u8, 64u8, 197u8, 162u8, 221u8, 170u8,
							140u8, 136u8, 205u8, 111u8, 8u8, 179u8, 166u8, 104u8, 74u8, 219u8,
							202u8, 123u8, 31u8, 129u8, 207u8, 58u8, 241u8, 91u8, 147u8, 112u8,
							162u8, 105u8,
						],
					)
				}
				#[doc = " The minimum score that each 'untrusted' solution must attain in order to be considered"]
				#[doc = " feasible."]
				#[doc = ""]
				#[doc = " Can be set via `set_minimum_untrusted_score`."]
				pub fn minimum_untrusted_score(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_npos_elections::ElectionScore,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ElectionProviderMultiPhase",
						"MinimumUntrustedScore",
						vec![],
						[
							105u8, 218u8, 96u8, 38u8, 82u8, 115u8, 30u8, 178u8, 21u8, 89u8, 59u8,
							7u8, 203u8, 240u8, 224u8, 209u8, 78u8, 28u8, 198u8, 236u8, 252u8,
							122u8, 72u8, 59u8, 156u8, 242u8, 26u8, 160u8, 145u8, 40u8, 6u8, 101u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Duration of the unsigned phase."]
				pub fn unsigned_phase(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"ElectionProviderMultiPhase",
						"UnsignedPhase",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Duration of the signed phase."]
				pub fn signed_phase(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"ElectionProviderMultiPhase",
						"SignedPhase",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The minimum amount of improvement to the solution score that defines a solution as"]
				#[doc = " \"better\" in the Signed phase."]
				pub fn better_signed_threshold(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_arithmetic::per_things::Perbill>
				{
					::subxt::constants::Address::new_static(
						"ElectionProviderMultiPhase",
						"BetterSignedThreshold",
						[
							65u8, 93u8, 120u8, 165u8, 204u8, 81u8, 159u8, 163u8, 93u8, 135u8,
							114u8, 121u8, 147u8, 35u8, 215u8, 213u8, 4u8, 223u8, 83u8, 37u8, 225u8,
							200u8, 189u8, 156u8, 140u8, 36u8, 58u8, 46u8, 42u8, 232u8, 155u8, 0u8,
						],
					)
				}
				#[doc = " The minimum amount of improvement to the solution score that defines a solution as"]
				#[doc = " \"better\" in the Unsigned phase."]
				pub fn better_unsigned_threshold(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_arithmetic::per_things::Perbill>
				{
					::subxt::constants::Address::new_static(
						"ElectionProviderMultiPhase",
						"BetterUnsignedThreshold",
						[
							65u8, 93u8, 120u8, 165u8, 204u8, 81u8, 159u8, 163u8, 93u8, 135u8,
							114u8, 121u8, 147u8, 35u8, 215u8, 213u8, 4u8, 223u8, 83u8, 37u8, 225u8,
							200u8, 189u8, 156u8, 140u8, 36u8, 58u8, 46u8, 42u8, 232u8, 155u8, 0u8,
						],
					)
				}
				#[doc = " The repeat threshold of the offchain worker."]
				#[doc = ""]
				#[doc = " For example, if it is 5, that means that at least 5 blocks will elapse between attempts"]
				#[doc = " to submit the worker's solution."]
				pub fn offchain_repeat(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"ElectionProviderMultiPhase",
						"OffchainRepeat",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The priority of the unsigned transaction submitted in the unsigned-phase"]
				pub fn miner_tx_priority(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"ElectionProviderMultiPhase",
						"MinerTxPriority",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
				#[doc = " Maximum number of signed submissions that can be queued."]
				#[doc = ""]
				#[doc = " It is best to avoid adjusting this during an election, as it impacts downstream data"]
				#[doc = " structures. In particular, `SignedSubmissionIndices<T>` is bounded on this value. If you"]
				#[doc = " update this value during an election, you _must_ ensure that"]
				#[doc = " `SignedSubmissionIndices.len()` is less than or equal to the new value. Otherwise,"]
				#[doc = " attempts to submit new solutions may cause a runtime panic."]
				pub fn signed_max_submissions(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"ElectionProviderMultiPhase",
						"SignedMaxSubmissions",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Maximum weight of a signed solution."]
				#[doc = ""]
				#[doc = " If [`Config::MinerConfig`] is being implemented to submit signed solutions (outside of"]
				#[doc = " this pallet), then [`MinerConfig::solution_weight`] is used to compare against"]
				#[doc = " this value."]
				pub fn signed_max_weight(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_weights::weight_v2::Weight> {
					::subxt::constants::Address::new_static(
						"ElectionProviderMultiPhase",
						"SignedMaxWeight",
						[
							222u8, 183u8, 203u8, 169u8, 31u8, 134u8, 28u8, 12u8, 47u8, 140u8, 71u8,
							74u8, 61u8, 55u8, 71u8, 236u8, 215u8, 83u8, 28u8, 70u8, 45u8, 128u8,
							184u8, 57u8, 101u8, 83u8, 42u8, 165u8, 34u8, 155u8, 64u8, 145u8,
						],
					)
				}
				#[doc = " The maximum amount of unchecked solutions to refund the call fee for."]
				pub fn signed_max_refunds(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"ElectionProviderMultiPhase",
						"SignedMaxRefunds",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Base reward for a signed solution"]
				pub fn signed_reward_base(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"ElectionProviderMultiPhase",
						"SignedRewardBase",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " Base deposit for a signed solution."]
				pub fn signed_deposit_base(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"ElectionProviderMultiPhase",
						"SignedDepositBase",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " Per-byte deposit for a signed solution."]
				pub fn signed_deposit_byte(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"ElectionProviderMultiPhase",
						"SignedDepositByte",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " Per-weight deposit for a signed solution."]
				pub fn signed_deposit_weight(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"ElectionProviderMultiPhase",
						"SignedDepositWeight",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The maximum number of electing voters to put in the snapshot. At the moment, snapshots"]
				#[doc = " are only over a single block, but once multi-block elections are introduced they will"]
				#[doc = " take place over multiple blocks."]
				pub fn max_electing_voters(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"ElectionProviderMultiPhase",
						"MaxElectingVoters",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of electable targets to put in the snapshot."]
				pub fn max_electable_targets(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u16> {
					::subxt::constants::Address::new_static(
						"ElectionProviderMultiPhase",
						"MaxElectableTargets",
						[
							116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
							41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
							90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
						],
					)
				}
				#[doc = " The maximum number of winners that can be elected by this `ElectionProvider`"]
				#[doc = " implementation."]
				#[doc = ""]
				#[doc = " Note: This must always be greater or equal to `T::DataProvider::desired_targets()`."]
				pub fn max_winners(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"ElectionProviderMultiPhase",
						"MaxWinners",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn miner_max_length(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"ElectionProviderMultiPhase",
						"MinerMaxLength",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn miner_max_weight(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_weights::weight_v2::Weight> {
					::subxt::constants::Address::new_static(
						"ElectionProviderMultiPhase",
						"MinerMaxWeight",
						[
							222u8, 183u8, 203u8, 169u8, 31u8, 134u8, 28u8, 12u8, 47u8, 140u8, 71u8,
							74u8, 61u8, 55u8, 71u8, 236u8, 215u8, 83u8, 28u8, 70u8, 45u8, 128u8,
							184u8, 57u8, 101u8, 83u8, 42u8, 165u8, 34u8, 155u8, 64u8, 145u8,
						],
					)
				}
				pub fn miner_max_votes_per_voter(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"ElectionProviderMultiPhase",
						"MinerMaxVotesPerVoter",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn miner_max_winners(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"ElectionProviderMultiPhase",
						"MinerMaxWinners",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod staking {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_staking::pallet::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_staking::pallet::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Bond {
					#[codec(compact)]
					pub value: ::core::primitive::u128,
					pub payee: runtime_types::pallet_staking::RewardDestination<
						::subxt::utils::AccountId32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Bond {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "bond";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BondExtra {
					#[codec(compact)]
					pub max_additional: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for BondExtra {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "bond_extra";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Unbond {
					#[codec(compact)]
					pub value: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for Unbond {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "unbond";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct WithdrawUnbonded {
					pub num_slashing_spans: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for WithdrawUnbonded {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "withdraw_unbonded";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Validate {
					pub prefs: runtime_types::pallet_staking::ValidatorPrefs,
				}
				impl ::subxt::blocks::StaticExtrinsic for Validate {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "validate";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Nominate {
					pub targets: ::std::vec::Vec<
						::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Nominate {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "nominate";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Chill;
				impl ::subxt::blocks::StaticExtrinsic for Chill {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "chill";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetPayee {
					pub payee: runtime_types::pallet_staking::RewardDestination<
						::subxt::utils::AccountId32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetPayee {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "set_payee";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetController;
				impl ::subxt::blocks::StaticExtrinsic for SetController {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "set_controller";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetValidatorCount {
					#[codec(compact)]
					pub new: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetValidatorCount {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "set_validator_count";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct IncreaseValidatorCount {
					#[codec(compact)]
					pub additional: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for IncreaseValidatorCount {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "increase_validator_count";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ScaleValidatorCount {
					pub factor: runtime_types::sp_arithmetic::per_things::Percent,
				}
				impl ::subxt::blocks::StaticExtrinsic for ScaleValidatorCount {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "scale_validator_count";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceNoEras;
				impl ::subxt::blocks::StaticExtrinsic for ForceNoEras {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "force_no_eras";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceNewEra;
				impl ::subxt::blocks::StaticExtrinsic for ForceNewEra {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "force_new_era";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetInvulnerables {
					pub invulnerables: ::std::vec::Vec<::subxt::utils::AccountId32>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetInvulnerables {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "set_invulnerables";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceUnstake {
					pub stash: ::subxt::utils::AccountId32,
					pub num_slashing_spans: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceUnstake {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "force_unstake";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceNewEraAlways;
				impl ::subxt::blocks::StaticExtrinsic for ForceNewEraAlways {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "force_new_era_always";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CancelDeferredSlash {
					pub era: ::core::primitive::u32,
					pub slash_indices: ::std::vec::Vec<::core::primitive::u32>,
				}
				impl ::subxt::blocks::StaticExtrinsic for CancelDeferredSlash {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "cancel_deferred_slash";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PayoutStakers {
					pub validator_stash: ::subxt::utils::AccountId32,
					pub era: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for PayoutStakers {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "payout_stakers";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Rebond {
					#[codec(compact)]
					pub value: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for Rebond {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "rebond";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ReapStash {
					pub stash: ::subxt::utils::AccountId32,
					pub num_slashing_spans: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for ReapStash {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "reap_stash";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Kick {
					pub who: ::std::vec::Vec<
						::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Kick {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "kick";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetStakingConfigs {
					pub min_nominator_bond: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
						::core::primitive::u128,
					>,
					pub min_validator_bond: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
						::core::primitive::u128,
					>,
					pub max_nominator_count:
						runtime_types::pallet_staking::pallet::pallet::ConfigOp<
							::core::primitive::u32,
						>,
					pub max_validator_count:
						runtime_types::pallet_staking::pallet::pallet::ConfigOp<
							::core::primitive::u32,
						>,
					pub chill_threshold: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
						runtime_types::sp_arithmetic::per_things::Percent,
					>,
					pub min_commission: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
						runtime_types::sp_arithmetic::per_things::Perbill,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetStakingConfigs {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "set_staking_configs";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ChillOther {
					pub controller: ::subxt::utils::AccountId32,
				}
				impl ::subxt::blocks::StaticExtrinsic for ChillOther {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "chill_other";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceApplyMinCommission {
					pub validator_stash: ::subxt::utils::AccountId32,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceApplyMinCommission {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "force_apply_min_commission";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetMinCommission {
					pub new: runtime_types::sp_arithmetic::per_things::Perbill,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetMinCommission {
					const PALLET: &'static str = "Staking";
					const CALL: &'static str = "set_min_commission";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::bond`]."]
				pub fn bond(
					&self,
					value: ::core::primitive::u128,
					payee: runtime_types::pallet_staking::RewardDestination<
						::subxt::utils::AccountId32,
					>,
				) -> ::subxt::tx::Payload<types::Bond> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"bond",
						types::Bond { value, payee },
						[
							45u8, 207u8, 34u8, 221u8, 252u8, 224u8, 162u8, 185u8, 67u8, 224u8,
							88u8, 91u8, 232u8, 114u8, 183u8, 44u8, 39u8, 5u8, 12u8, 163u8, 57u8,
							31u8, 251u8, 58u8, 37u8, 232u8, 206u8, 75u8, 164u8, 26u8, 170u8, 101u8,
						],
					)
				}
				#[doc = "See [`Pallet::bond_extra`]."]
				pub fn bond_extra(
					&self,
					max_additional: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::BondExtra> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"bond_extra",
						types::BondExtra { max_additional },
						[
							9u8, 143u8, 179u8, 99u8, 91u8, 254u8, 114u8, 189u8, 202u8, 245u8, 48u8,
							130u8, 103u8, 17u8, 183u8, 177u8, 172u8, 156u8, 227u8, 145u8, 191u8,
							134u8, 81u8, 3u8, 170u8, 85u8, 40u8, 56u8, 216u8, 95u8, 232u8, 52u8,
						],
					)
				}
				#[doc = "See [`Pallet::unbond`]."]
				pub fn unbond(
					&self,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::Unbond> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"unbond",
						types::Unbond { value },
						[
							70u8, 201u8, 146u8, 56u8, 51u8, 237u8, 90u8, 193u8, 69u8, 42u8, 168u8,
							96u8, 215u8, 128u8, 253u8, 22u8, 239u8, 14u8, 214u8, 103u8, 170u8,
							140u8, 2u8, 182u8, 3u8, 190u8, 184u8, 191u8, 231u8, 137u8, 50u8, 16u8,
						],
					)
				}
				#[doc = "See [`Pallet::withdraw_unbonded`]."]
				pub fn withdraw_unbonded(
					&self,
					num_slashing_spans: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::WithdrawUnbonded> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"withdraw_unbonded",
						types::WithdrawUnbonded { num_slashing_spans },
						[
							229u8, 128u8, 177u8, 224u8, 197u8, 118u8, 239u8, 142u8, 179u8, 164u8,
							10u8, 205u8, 124u8, 254u8, 209u8, 157u8, 172u8, 87u8, 58u8, 120u8,
							74u8, 12u8, 150u8, 117u8, 234u8, 32u8, 191u8, 182u8, 92u8, 97u8, 77u8,
							59u8,
						],
					)
				}
				#[doc = "See [`Pallet::validate`]."]
				pub fn validate(
					&self,
					prefs: runtime_types::pallet_staking::ValidatorPrefs,
				) -> ::subxt::tx::Payload<types::Validate> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"validate",
						types::Validate { prefs },
						[
							63u8, 83u8, 12u8, 16u8, 56u8, 84u8, 41u8, 141u8, 202u8, 0u8, 37u8,
							30u8, 115u8, 2u8, 145u8, 101u8, 168u8, 89u8, 94u8, 98u8, 8u8, 45u8,
							140u8, 237u8, 101u8, 136u8, 179u8, 162u8, 205u8, 41u8, 88u8, 248u8,
						],
					)
				}
				#[doc = "See [`Pallet::nominate`]."]
				pub fn nominate(
					&self,
					targets: ::std::vec::Vec<
						::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					>,
				) -> ::subxt::tx::Payload<types::Nominate> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"nominate",
						types::Nominate { targets },
						[
							15u8, 90u8, 71u8, 240u8, 160u8, 193u8, 136u8, 125u8, 36u8, 150u8,
							105u8, 92u8, 188u8, 102u8, 160u8, 56u8, 164u8, 250u8, 75u8, 106u8,
							10u8, 123u8, 18u8, 50u8, 59u8, 128u8, 35u8, 199u8, 103u8, 109u8, 45u8,
							223u8,
						],
					)
				}
				#[doc = "See [`Pallet::chill`]."]
				pub fn chill(&self) -> ::subxt::tx::Payload<types::Chill> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"chill",
						types::Chill {},
						[
							157u8, 75u8, 243u8, 69u8, 110u8, 192u8, 22u8, 27u8, 107u8, 68u8, 236u8,
							58u8, 179u8, 34u8, 118u8, 98u8, 131u8, 62u8, 242u8, 84u8, 149u8, 24u8,
							83u8, 223u8, 78u8, 12u8, 192u8, 22u8, 111u8, 11u8, 171u8, 149u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_payee`]."]
				pub fn set_payee(
					&self,
					payee: runtime_types::pallet_staking::RewardDestination<
						::subxt::utils::AccountId32,
					>,
				) -> ::subxt::tx::Payload<types::SetPayee> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"set_payee",
						types::SetPayee { payee },
						[
							86u8, 172u8, 187u8, 98u8, 106u8, 240u8, 184u8, 60u8, 163u8, 244u8, 7u8,
							64u8, 147u8, 168u8, 192u8, 177u8, 211u8, 138u8, 73u8, 188u8, 159u8,
							154u8, 175u8, 219u8, 231u8, 235u8, 93u8, 195u8, 204u8, 100u8, 196u8,
							241u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_controller`]."]
				pub fn set_controller(&self) -> ::subxt::tx::Payload<types::SetController> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"set_controller",
						types::SetController {},
						[
							172u8, 27u8, 195u8, 188u8, 145u8, 203u8, 190u8, 174u8, 145u8, 43u8,
							253u8, 87u8, 11u8, 229u8, 112u8, 18u8, 57u8, 101u8, 84u8, 235u8, 109u8,
							228u8, 58u8, 129u8, 179u8, 174u8, 245u8, 169u8, 89u8, 240u8, 39u8,
							67u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_validator_count`]."]
				pub fn set_validator_count(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::SetValidatorCount> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"set_validator_count",
						types::SetValidatorCount { new },
						[
							172u8, 225u8, 157u8, 48u8, 242u8, 217u8, 126u8, 206u8, 26u8, 156u8,
							203u8, 100u8, 116u8, 189u8, 98u8, 89u8, 151u8, 101u8, 77u8, 236u8,
							101u8, 8u8, 148u8, 236u8, 180u8, 175u8, 232u8, 146u8, 141u8, 141u8,
							78u8, 165u8,
						],
					)
				}
				#[doc = "See [`Pallet::increase_validator_count`]."]
				pub fn increase_validator_count(
					&self,
					additional: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::IncreaseValidatorCount> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"increase_validator_count",
						types::IncreaseValidatorCount { additional },
						[
							108u8, 67u8, 131u8, 248u8, 139u8, 227u8, 224u8, 221u8, 248u8, 94u8,
							141u8, 104u8, 131u8, 250u8, 127u8, 164u8, 137u8, 211u8, 5u8, 27u8,
							185u8, 251u8, 120u8, 243u8, 165u8, 50u8, 197u8, 161u8, 125u8, 195u8,
							16u8, 29u8,
						],
					)
				}
				#[doc = "See [`Pallet::scale_validator_count`]."]
				pub fn scale_validator_count(
					&self,
					factor: runtime_types::sp_arithmetic::per_things::Percent,
				) -> ::subxt::tx::Payload<types::ScaleValidatorCount> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"scale_validator_count",
						types::ScaleValidatorCount { factor },
						[
							93u8, 200u8, 119u8, 240u8, 148u8, 144u8, 175u8, 135u8, 102u8, 130u8,
							183u8, 216u8, 28u8, 215u8, 155u8, 233u8, 152u8, 65u8, 49u8, 125u8,
							196u8, 79u8, 31u8, 195u8, 233u8, 79u8, 150u8, 138u8, 103u8, 161u8,
							78u8, 154u8,
						],
					)
				}
				#[doc = "See [`Pallet::force_no_eras`]."]
				pub fn force_no_eras(&self) -> ::subxt::tx::Payload<types::ForceNoEras> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"force_no_eras",
						types::ForceNoEras {},
						[
							77u8, 5u8, 105u8, 167u8, 251u8, 78u8, 52u8, 80u8, 177u8, 226u8, 28u8,
							130u8, 106u8, 62u8, 40u8, 210u8, 110u8, 62u8, 21u8, 113u8, 234u8,
							227u8, 171u8, 205u8, 240u8, 46u8, 32u8, 84u8, 184u8, 208u8, 61u8,
							207u8,
						],
					)
				}
				#[doc = "See [`Pallet::force_new_era`]."]
				pub fn force_new_era(&self) -> ::subxt::tx::Payload<types::ForceNewEra> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"force_new_era",
						types::ForceNewEra {},
						[
							119u8, 45u8, 11u8, 87u8, 236u8, 189u8, 41u8, 142u8, 130u8, 10u8, 132u8,
							140u8, 210u8, 134u8, 66u8, 152u8, 149u8, 55u8, 60u8, 31u8, 190u8, 41u8,
							177u8, 103u8, 245u8, 193u8, 95u8, 255u8, 29u8, 79u8, 112u8, 188u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_invulnerables`]."]
				pub fn set_invulnerables(
					&self,
					invulnerables: ::std::vec::Vec<::subxt::utils::AccountId32>,
				) -> ::subxt::tx::Payload<types::SetInvulnerables> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"set_invulnerables",
						types::SetInvulnerables { invulnerables },
						[
							31u8, 115u8, 221u8, 229u8, 187u8, 61u8, 33u8, 22u8, 126u8, 142u8,
							248u8, 190u8, 213u8, 35u8, 49u8, 208u8, 193u8, 0u8, 58u8, 18u8, 136u8,
							220u8, 32u8, 8u8, 121u8, 36u8, 184u8, 57u8, 6u8, 125u8, 199u8, 245u8,
						],
					)
				}
				#[doc = "See [`Pallet::force_unstake`]."]
				pub fn force_unstake(
					&self,
					stash: ::subxt::utils::AccountId32,
					num_slashing_spans: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::ForceUnstake> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"force_unstake",
						types::ForceUnstake {
							stash,
							num_slashing_spans,
						},
						[
							205u8, 115u8, 222u8, 58u8, 168u8, 3u8, 59u8, 58u8, 220u8, 98u8, 204u8,
							90u8, 36u8, 250u8, 178u8, 45u8, 213u8, 158u8, 92u8, 107u8, 3u8, 94u8,
							118u8, 194u8, 187u8, 196u8, 101u8, 250u8, 36u8, 119u8, 21u8, 19u8,
						],
					)
				}
				#[doc = "See [`Pallet::force_new_era_always`]."]
				pub fn force_new_era_always(
					&self,
				) -> ::subxt::tx::Payload<types::ForceNewEraAlways> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"force_new_era_always",
						types::ForceNewEraAlways {},
						[
							102u8, 153u8, 116u8, 85u8, 80u8, 52u8, 89u8, 215u8, 173u8, 159u8, 96u8,
							99u8, 180u8, 5u8, 62u8, 142u8, 181u8, 101u8, 160u8, 57u8, 177u8, 182u8,
							6u8, 252u8, 107u8, 252u8, 225u8, 104u8, 147u8, 123u8, 244u8, 134u8,
						],
					)
				}
				#[doc = "See [`Pallet::cancel_deferred_slash`]."]
				pub fn cancel_deferred_slash(
					&self,
					era: ::core::primitive::u32,
					slash_indices: ::std::vec::Vec<::core::primitive::u32>,
				) -> ::subxt::tx::Payload<types::CancelDeferredSlash> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"cancel_deferred_slash",
						types::CancelDeferredSlash { era, slash_indices },
						[
							65u8, 90u8, 54u8, 7u8, 89u8, 238u8, 254u8, 76u8, 219u8, 26u8, 137u8,
							181u8, 154u8, 49u8, 35u8, 99u8, 181u8, 193u8, 209u8, 181u8, 212u8,
							153u8, 49u8, 83u8, 77u8, 170u8, 175u8, 142u8, 63u8, 187u8, 183u8,
							199u8,
						],
					)
				}
				#[doc = "See [`Pallet::payout_stakers`]."]
				pub fn payout_stakers(
					&self,
					validator_stash: ::subxt::utils::AccountId32,
					era: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::PayoutStakers> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"payout_stakers",
						types::PayoutStakers {
							validator_stash,
							era,
						},
						[
							69u8, 67u8, 140u8, 197u8, 89u8, 20u8, 59u8, 55u8, 142u8, 197u8, 62u8,
							107u8, 239u8, 50u8, 237u8, 52u8, 4u8, 65u8, 119u8, 73u8, 138u8, 57u8,
							46u8, 78u8, 252u8, 157u8, 187u8, 14u8, 232u8, 244u8, 217u8, 171u8,
						],
					)
				}
				#[doc = "See [`Pallet::rebond`]."]
				pub fn rebond(
					&self,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::Rebond> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"rebond",
						types::Rebond { value },
						[
							204u8, 209u8, 27u8, 219u8, 45u8, 129u8, 15u8, 39u8, 105u8, 165u8,
							255u8, 55u8, 0u8, 59u8, 115u8, 79u8, 139u8, 82u8, 163u8, 197u8, 44u8,
							89u8, 41u8, 234u8, 116u8, 214u8, 248u8, 123u8, 250u8, 49u8, 15u8, 77u8,
						],
					)
				}
				#[doc = "See [`Pallet::reap_stash`]."]
				pub fn reap_stash(
					&self,
					stash: ::subxt::utils::AccountId32,
					num_slashing_spans: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::ReapStash> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"reap_stash",
						types::ReapStash {
							stash,
							num_slashing_spans,
						},
						[
							231u8, 240u8, 152u8, 33u8, 10u8, 60u8, 18u8, 233u8, 0u8, 229u8, 90u8,
							45u8, 118u8, 29u8, 98u8, 109u8, 89u8, 7u8, 228u8, 254u8, 119u8, 125u8,
							172u8, 209u8, 217u8, 107u8, 50u8, 226u8, 31u8, 5u8, 153u8, 93u8,
						],
					)
				}
				#[doc = "See [`Pallet::kick`]."]
				pub fn kick(
					&self,
					who: ::std::vec::Vec<
						::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					>,
				) -> ::subxt::tx::Payload<types::Kick> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"kick",
						types::Kick { who },
						[
							28u8, 254u8, 219u8, 14u8, 44u8, 187u8, 186u8, 72u8, 55u8, 21u8, 129u8,
							205u8, 225u8, 204u8, 162u8, 53u8, 255u8, 56u8, 49u8, 194u8, 63u8, 43u8,
							96u8, 177u8, 125u8, 0u8, 163u8, 97u8, 231u8, 159u8, 43u8, 249u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_staking_configs`]."]
				pub fn set_staking_configs(
					&self,
					min_nominator_bond: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
						::core::primitive::u128,
					>,
					min_validator_bond: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
						::core::primitive::u128,
					>,
					max_nominator_count: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
						::core::primitive::u32,
					>,
					max_validator_count: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
						::core::primitive::u32,
					>,
					chill_threshold: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
						runtime_types::sp_arithmetic::per_things::Percent,
					>,
					min_commission: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
						runtime_types::sp_arithmetic::per_things::Perbill,
					>,
				) -> ::subxt::tx::Payload<types::SetStakingConfigs> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"set_staking_configs",
						types::SetStakingConfigs {
							min_nominator_bond,
							min_validator_bond,
							max_nominator_count,
							max_validator_count,
							chill_threshold,
							min_commission,
						},
						[
							198u8, 212u8, 176u8, 138u8, 79u8, 177u8, 241u8, 104u8, 72u8, 170u8,
							35u8, 178u8, 205u8, 167u8, 218u8, 118u8, 42u8, 226u8, 180u8, 17u8,
							112u8, 175u8, 55u8, 248u8, 64u8, 127u8, 51u8, 65u8, 132u8, 210u8, 88u8,
							213u8,
						],
					)
				}
				#[doc = "See [`Pallet::chill_other`]."]
				pub fn chill_other(
					&self,
					controller: ::subxt::utils::AccountId32,
				) -> ::subxt::tx::Payload<types::ChillOther> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"chill_other",
						types::ChillOther { controller },
						[
							143u8, 82u8, 167u8, 43u8, 102u8, 136u8, 78u8, 139u8, 110u8, 159u8,
							235u8, 226u8, 237u8, 140u8, 142u8, 47u8, 77u8, 57u8, 209u8, 208u8, 9u8,
							193u8, 3u8, 77u8, 147u8, 41u8, 182u8, 122u8, 178u8, 185u8, 32u8, 182u8,
						],
					)
				}
				#[doc = "See [`Pallet::force_apply_min_commission`]."]
				pub fn force_apply_min_commission(
					&self,
					validator_stash: ::subxt::utils::AccountId32,
				) -> ::subxt::tx::Payload<types::ForceApplyMinCommission> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"force_apply_min_commission",
						types::ForceApplyMinCommission { validator_stash },
						[
							158u8, 27u8, 152u8, 23u8, 97u8, 53u8, 54u8, 49u8, 179u8, 236u8, 69u8,
							65u8, 253u8, 136u8, 232u8, 44u8, 207u8, 66u8, 5u8, 186u8, 49u8, 91u8,
							173u8, 5u8, 84u8, 45u8, 154u8, 91u8, 239u8, 97u8, 62u8, 42u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_min_commission`]."]
				pub fn set_min_commission(
					&self,
					new: runtime_types::sp_arithmetic::per_things::Perbill,
				) -> ::subxt::tx::Payload<types::SetMinCommission> {
					::subxt::tx::Payload::new_static(
						"Staking",
						"set_min_commission",
						types::SetMinCommission { new },
						[
							96u8, 168u8, 55u8, 79u8, 79u8, 49u8, 8u8, 127u8, 98u8, 158u8, 106u8,
							187u8, 177u8, 201u8, 68u8, 181u8, 219u8, 172u8, 63u8, 120u8, 172u8,
							173u8, 251u8, 167u8, 84u8, 165u8, 238u8, 115u8, 110u8, 97u8, 144u8,
							50u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_staking::pallet::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The era payout has been set; the first balance is the validator-payout; the second is"]
			#[doc = "the remainder from the maximum amount of reward."]
			pub struct EraPaid {
				pub era_index: ::core::primitive::u32,
				pub validator_payout: ::core::primitive::u128,
				pub remainder: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for EraPaid {
				const PALLET: &'static str = "Staking";
				const EVENT: &'static str = "EraPaid";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The nominator has been rewarded by this amount."]
			pub struct Rewarded {
				pub stash: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Rewarded {
				const PALLET: &'static str = "Staking";
				const EVENT: &'static str = "Rewarded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A staker (validator or nominator) has been slashed by the given amount."]
			pub struct Slashed {
				pub staker: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Slashed {
				const PALLET: &'static str = "Staking";
				const EVENT: &'static str = "Slashed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A slash for the given validator, for the given percentage of their stake, at the given"]
			#[doc = "era as been reported."]
			pub struct SlashReported {
				pub validator: ::subxt::utils::AccountId32,
				pub fraction: runtime_types::sp_arithmetic::per_things::Perbill,
				pub slash_era: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for SlashReported {
				const PALLET: &'static str = "Staking";
				const EVENT: &'static str = "SlashReported";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An old slashing report from a prior era was discarded because it could"]
			#[doc = "not be processed."]
			pub struct OldSlashingReportDiscarded {
				pub session_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for OldSlashingReportDiscarded {
				const PALLET: &'static str = "Staking";
				const EVENT: &'static str = "OldSlashingReportDiscarded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A new set of stakers was elected."]
			pub struct StakersElected;
			impl ::subxt::events::StaticEvent for StakersElected {
				const PALLET: &'static str = "Staking";
				const EVENT: &'static str = "StakersElected";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account has bonded this amount. \\[stash, amount\\]"]
			#[doc = ""]
			#[doc = "NOTE: This event is only emitted when funds are bonded via a dispatchable. Notably,"]
			#[doc = "it will not be emitted for staking rewards when they are added to stake."]
			pub struct Bonded {
				pub stash: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Bonded {
				const PALLET: &'static str = "Staking";
				const EVENT: &'static str = "Bonded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account has unbonded this amount."]
			pub struct Unbonded {
				pub stash: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Unbonded {
				const PALLET: &'static str = "Staking";
				const EVENT: &'static str = "Unbonded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account has called `withdraw_unbonded` and removed unbonding chunks worth `Balance`"]
			#[doc = "from the unlocking queue."]
			pub struct Withdrawn {
				pub stash: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Withdrawn {
				const PALLET: &'static str = "Staking";
				const EVENT: &'static str = "Withdrawn";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A nominator has been kicked from a validator."]
			pub struct Kicked {
				pub nominator: ::subxt::utils::AccountId32,
				pub stash: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Kicked {
				const PALLET: &'static str = "Staking";
				const EVENT: &'static str = "Kicked";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The election failed. No new era is planned."]
			pub struct StakingElectionFailed;
			impl ::subxt::events::StaticEvent for StakingElectionFailed {
				const PALLET: &'static str = "Staking";
				const EVENT: &'static str = "StakingElectionFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account has stopped participating as either a validator or nominator."]
			pub struct Chilled {
				pub stash: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Chilled {
				const PALLET: &'static str = "Staking";
				const EVENT: &'static str = "Chilled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The stakers' rewards are getting paid."]
			pub struct PayoutStarted {
				pub era_index: ::core::primitive::u32,
				pub validator_stash: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for PayoutStarted {
				const PALLET: &'static str = "Staking";
				const EVENT: &'static str = "PayoutStarted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A validator has set their preferences."]
			pub struct ValidatorPrefsSet {
				pub stash: ::subxt::utils::AccountId32,
				pub prefs: runtime_types::pallet_staking::ValidatorPrefs,
			}
			impl ::subxt::events::StaticEvent for ValidatorPrefsSet {
				const PALLET: &'static str = "Staking";
				const EVENT: &'static str = "ValidatorPrefsSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A new force era mode was set."]
			pub struct ForceEra {
				pub mode: runtime_types::pallet_staking::Forcing,
			}
			impl ::subxt::events::StaticEvent for ForceEra {
				const PALLET: &'static str = "Staking";
				const EVENT: &'static str = "ForceEra";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The ideal number of active validators."]
				pub fn validator_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"ValidatorCount",
						vec![],
						[
							105u8, 251u8, 193u8, 198u8, 232u8, 118u8, 73u8, 115u8, 205u8, 78u8,
							49u8, 253u8, 140u8, 193u8, 161u8, 205u8, 13u8, 147u8, 125u8, 102u8,
							142u8, 244u8, 210u8, 227u8, 225u8, 46u8, 144u8, 122u8, 254u8, 48u8,
							44u8, 169u8,
						],
					)
				}
				#[doc = " Minimum number of staking participants before emergency conditions are imposed."]
				pub fn minimum_validator_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"MinimumValidatorCount",
						vec![],
						[
							103u8, 178u8, 29u8, 91u8, 90u8, 31u8, 49u8, 9u8, 11u8, 58u8, 178u8,
							30u8, 219u8, 55u8, 58u8, 181u8, 80u8, 155u8, 9u8, 11u8, 38u8, 46u8,
							125u8, 179u8, 220u8, 20u8, 212u8, 181u8, 136u8, 103u8, 58u8, 48u8,
						],
					)
				}
				#[doc = " Any validators that may never be slashed or forcibly kicked. It's a Vec since they're"]
				#[doc = " easy to initialize and the performance hit is minimal (we expect no more than four"]
				#[doc = " invulnerables) and restricted to testnets."]
				pub fn invulnerables(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"Invulnerables",
						vec![],
						[
							199u8, 35u8, 0u8, 229u8, 160u8, 128u8, 139u8, 245u8, 27u8, 133u8, 47u8,
							240u8, 86u8, 195u8, 90u8, 169u8, 158u8, 231u8, 128u8, 58u8, 24u8,
							173u8, 138u8, 122u8, 226u8, 104u8, 239u8, 114u8, 91u8, 165u8, 207u8,
							150u8,
						],
					)
				}
				#[doc = " Map from all locked \"stash\" accounts to the controller account."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
				pub fn bonded(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"Bonded",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							146u8, 230u8, 48u8, 190u8, 166u8, 127u8, 237u8, 216u8, 71u8, 33u8,
							108u8, 121u8, 204u8, 211u8, 133u8, 123u8, 52u8, 164u8, 201u8, 209u8,
							236u8, 35u8, 190u8, 77u8, 126u8, 150u8, 79u8, 244u8, 15u8, 247u8,
							161u8, 107u8,
						],
					)
				}
				#[doc = " Map from all locked \"stash\" accounts to the controller account."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
				pub fn bonded_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"Bonded",
						Vec::new(),
						[
							146u8, 230u8, 48u8, 190u8, 166u8, 127u8, 237u8, 216u8, 71u8, 33u8,
							108u8, 121u8, 204u8, 211u8, 133u8, 123u8, 52u8, 164u8, 201u8, 209u8,
							236u8, 35u8, 190u8, 77u8, 126u8, 150u8, 79u8, 244u8, 15u8, 247u8,
							161u8, 107u8,
						],
					)
				}
				#[doc = " The minimum active bond to become and maintain the role of a nominator."]
				pub fn min_nominator_bond(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"MinNominatorBond",
						vec![],
						[
							102u8, 115u8, 254u8, 15u8, 191u8, 228u8, 85u8, 249u8, 112u8, 190u8,
							129u8, 243u8, 236u8, 39u8, 195u8, 232u8, 10u8, 230u8, 11u8, 144u8,
							115u8, 1u8, 45u8, 70u8, 181u8, 161u8, 17u8, 92u8, 19u8, 70u8, 100u8,
							94u8,
						],
					)
				}
				#[doc = " The minimum active bond to become and maintain the role of a validator."]
				pub fn min_validator_bond(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"MinValidatorBond",
						vec![],
						[
							146u8, 249u8, 26u8, 52u8, 224u8, 81u8, 85u8, 153u8, 118u8, 169u8,
							140u8, 37u8, 208u8, 242u8, 8u8, 29u8, 156u8, 73u8, 154u8, 162u8, 186u8,
							159u8, 119u8, 100u8, 109u8, 227u8, 6u8, 139u8, 155u8, 203u8, 167u8,
							244u8,
						],
					)
				}
				#[doc = " The minimum active nominator stake of the last successful election."]
				pub fn minimum_active_stake(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"MinimumActiveStake",
						vec![],
						[
							166u8, 211u8, 59u8, 23u8, 2u8, 160u8, 244u8, 52u8, 153u8, 12u8, 103u8,
							113u8, 51u8, 232u8, 145u8, 188u8, 54u8, 67u8, 227u8, 221u8, 186u8, 6u8,
							28u8, 63u8, 146u8, 212u8, 233u8, 173u8, 134u8, 41u8, 169u8, 153u8,
						],
					)
				}
				#[doc = " The minimum amount of commission that validators can set."]
				#[doc = ""]
				#[doc = " If set to `0`, no limit exists."]
				pub fn min_commission(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::per_things::Perbill,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"MinCommission",
						vec![],
						[
							220u8, 197u8, 232u8, 212u8, 205u8, 242u8, 121u8, 165u8, 255u8, 199u8,
							122u8, 20u8, 145u8, 245u8, 175u8, 26u8, 45u8, 70u8, 207u8, 26u8, 112u8,
							234u8, 181u8, 167u8, 140u8, 75u8, 15u8, 1u8, 221u8, 168u8, 17u8, 211u8,
						],
					)
				}
				#[doc = " Map from all (unlocked) \"controller\" accounts to the info regarding the staking."]
				pub fn ledger(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::StakingLedger,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"Ledger",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							77u8, 39u8, 230u8, 122u8, 108u8, 191u8, 251u8, 28u8, 233u8, 225u8,
							195u8, 224u8, 234u8, 90u8, 173u8, 170u8, 143u8, 246u8, 246u8, 21u8,
							38u8, 187u8, 112u8, 111u8, 206u8, 181u8, 183u8, 186u8, 96u8, 8u8,
							225u8, 224u8,
						],
					)
				}
				#[doc = " Map from all (unlocked) \"controller\" accounts to the info regarding the staking."]
				pub fn ledger_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::StakingLedger,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"Ledger",
						Vec::new(),
						[
							77u8, 39u8, 230u8, 122u8, 108u8, 191u8, 251u8, 28u8, 233u8, 225u8,
							195u8, 224u8, 234u8, 90u8, 173u8, 170u8, 143u8, 246u8, 246u8, 21u8,
							38u8, 187u8, 112u8, 111u8, 206u8, 181u8, 183u8, 186u8, 96u8, 8u8,
							225u8, 224u8,
						],
					)
				}
				#[doc = " Where the reward payment should be made. Keyed by stash."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
				pub fn payee(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::RewardDestination<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"Payee",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							198u8, 238u8, 10u8, 104u8, 204u8, 7u8, 193u8, 254u8, 169u8, 18u8,
							187u8, 212u8, 90u8, 243u8, 73u8, 29u8, 216u8, 144u8, 93u8, 140u8, 11u8,
							124u8, 4u8, 191u8, 107u8, 61u8, 15u8, 152u8, 70u8, 82u8, 60u8, 75u8,
						],
					)
				}
				#[doc = " Where the reward payment should be made. Keyed by stash."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
				pub fn payee_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::RewardDestination<::subxt::utils::AccountId32>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"Payee",
						Vec::new(),
						[
							198u8, 238u8, 10u8, 104u8, 204u8, 7u8, 193u8, 254u8, 169u8, 18u8,
							187u8, 212u8, 90u8, 243u8, 73u8, 29u8, 216u8, 144u8, 93u8, 140u8, 11u8,
							124u8, 4u8, 191u8, 107u8, 61u8, 15u8, 152u8, 70u8, 82u8, 60u8, 75u8,
						],
					)
				}
				#[doc = " The map from (wannabe) validator stash key to the preferences of that validator."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
				pub fn validators(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::ValidatorPrefs,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"Validators",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							149u8, 207u8, 68u8, 38u8, 24u8, 220u8, 207u8, 84u8, 236u8, 33u8, 210u8,
							124u8, 200u8, 99u8, 98u8, 29u8, 235u8, 46u8, 124u8, 4u8, 203u8, 6u8,
							209u8, 21u8, 124u8, 236u8, 112u8, 118u8, 180u8, 85u8, 78u8, 13u8,
						],
					)
				}
				#[doc = " The map from (wannabe) validator stash key to the preferences of that validator."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
				pub fn validators_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::ValidatorPrefs,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"Validators",
						Vec::new(),
						[
							149u8, 207u8, 68u8, 38u8, 24u8, 220u8, 207u8, 84u8, 236u8, 33u8, 210u8,
							124u8, 200u8, 99u8, 98u8, 29u8, 235u8, 46u8, 124u8, 4u8, 203u8, 6u8,
							209u8, 21u8, 124u8, 236u8, 112u8, 118u8, 180u8, 85u8, 78u8, 13u8,
						],
					)
				}
				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_validators(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"CounterForValidators",
						vec![],
						[
							169u8, 146u8, 194u8, 114u8, 57u8, 232u8, 137u8, 93u8, 214u8, 98u8,
							176u8, 151u8, 237u8, 165u8, 176u8, 252u8, 73u8, 124u8, 22u8, 166u8,
							225u8, 217u8, 65u8, 56u8, 174u8, 12u8, 32u8, 2u8, 7u8, 173u8, 125u8,
							235u8,
						],
					)
				}
				#[doc = " The maximum validator count before we stop allowing new validators to join."]
				#[doc = ""]
				#[doc = " When this value is not set, no limits are enforced."]
				pub fn max_validators_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"MaxValidatorsCount",
						vec![],
						[
							139u8, 116u8, 236u8, 217u8, 110u8, 47u8, 140u8, 197u8, 184u8, 246u8,
							180u8, 188u8, 233u8, 99u8, 102u8, 21u8, 114u8, 23u8, 143u8, 163u8,
							224u8, 250u8, 248u8, 185u8, 235u8, 94u8, 110u8, 83u8, 170u8, 123u8,
							113u8, 168u8,
						],
					)
				}
				#[doc = " The map from nominator stash key to their nomination preferences, namely the validators that"]
				#[doc = " they wish to support."]
				#[doc = ""]
				#[doc = " Note that the keys of this storage map might become non-decodable in case the"]
				#[doc = " [`Config::MaxNominations`] configuration is decreased. In this rare case, these nominators"]
				#[doc = " are still existent in storage, their key is correct and retrievable (i.e. `contains_key`"]
				#[doc = " indicates that they exist), but their value cannot be decoded. Therefore, the non-decodable"]
				#[doc = " nominators will effectively not-exist, until they re-submit their preferences such that it"]
				#[doc = " is within the bounds of the newly set `Config::MaxNominations`."]
				#[doc = ""]
				#[doc = " This implies that `::iter_keys().count()` and `::iter().count()` might return different"]
				#[doc = " values for this map. Moreover, the main `::count()` is aligned with the former, namely the"]
				#[doc = " number of keys that exist."]
				#[doc = ""]
				#[doc = " Lastly, if any of the nominators become non-decodable, they can be chilled immediately via"]
				#[doc = " [`Call::chill_other`] dispatchable by anyone."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
				pub fn nominators(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::Nominations,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"Nominators",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							114u8, 45u8, 86u8, 23u8, 12u8, 98u8, 114u8, 3u8, 170u8, 11u8, 100u8,
							17u8, 122u8, 158u8, 192u8, 21u8, 160u8, 87u8, 85u8, 142u8, 241u8,
							232u8, 25u8, 6u8, 36u8, 85u8, 155u8, 79u8, 124u8, 173u8, 0u8, 252u8,
						],
					)
				}
				#[doc = " The map from nominator stash key to their nomination preferences, namely the validators that"]
				#[doc = " they wish to support."]
				#[doc = ""]
				#[doc = " Note that the keys of this storage map might become non-decodable in case the"]
				#[doc = " [`Config::MaxNominations`] configuration is decreased. In this rare case, these nominators"]
				#[doc = " are still existent in storage, their key is correct and retrievable (i.e. `contains_key`"]
				#[doc = " indicates that they exist), but their value cannot be decoded. Therefore, the non-decodable"]
				#[doc = " nominators will effectively not-exist, until they re-submit their preferences such that it"]
				#[doc = " is within the bounds of the newly set `Config::MaxNominations`."]
				#[doc = ""]
				#[doc = " This implies that `::iter_keys().count()` and `::iter().count()` might return different"]
				#[doc = " values for this map. Moreover, the main `::count()` is aligned with the former, namely the"]
				#[doc = " number of keys that exist."]
				#[doc = ""]
				#[doc = " Lastly, if any of the nominators become non-decodable, they can be chilled immediately via"]
				#[doc = " [`Call::chill_other`] dispatchable by anyone."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
				pub fn nominators_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::Nominations,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"Nominators",
						Vec::new(),
						[
							114u8, 45u8, 86u8, 23u8, 12u8, 98u8, 114u8, 3u8, 170u8, 11u8, 100u8,
							17u8, 122u8, 158u8, 192u8, 21u8, 160u8, 87u8, 85u8, 142u8, 241u8,
							232u8, 25u8, 6u8, 36u8, 85u8, 155u8, 79u8, 124u8, 173u8, 0u8, 252u8,
						],
					)
				}
				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_nominators(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"CounterForNominators",
						vec![],
						[
							150u8, 236u8, 184u8, 12u8, 224u8, 26u8, 13u8, 204u8, 208u8, 178u8,
							68u8, 148u8, 232u8, 85u8, 74u8, 248u8, 167u8, 61u8, 88u8, 126u8, 40u8,
							20u8, 73u8, 47u8, 94u8, 57u8, 144u8, 77u8, 156u8, 179u8, 55u8, 49u8,
						],
					)
				}
				#[doc = " The maximum nominator count before we stop allowing new validators to join."]
				#[doc = ""]
				#[doc = " When this value is not set, no limits are enforced."]
				pub fn max_nominators_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"MaxNominatorsCount",
						vec![],
						[
							11u8, 234u8, 179u8, 254u8, 95u8, 119u8, 35u8, 255u8, 141u8, 95u8,
							148u8, 209u8, 43u8, 202u8, 19u8, 57u8, 185u8, 50u8, 152u8, 192u8, 95u8,
							13u8, 158u8, 245u8, 113u8, 199u8, 255u8, 187u8, 37u8, 44u8, 8u8, 119u8,
						],
					)
				}
				#[doc = " The current era index."]
				#[doc = ""]
				#[doc = " This is the latest planned era, depending on how the Session pallet queues the validator"]
				#[doc = " set, it might be active or not."]
				pub fn current_era(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"CurrentEra",
						vec![],
						[
							247u8, 239u8, 171u8, 18u8, 137u8, 240u8, 213u8, 3u8, 173u8, 173u8,
							236u8, 141u8, 202u8, 191u8, 228u8, 120u8, 196u8, 188u8, 13u8, 66u8,
							253u8, 117u8, 90u8, 8u8, 158u8, 11u8, 236u8, 141u8, 178u8, 44u8, 119u8,
							25u8,
						],
					)
				}
				#[doc = " The active era information, it holds index and start."]
				#[doc = ""]
				#[doc = " The active era is the era being currently rewarded. Validator set of this era must be"]
				#[doc = " equal to [`SessionInterface::validators`]."]
				pub fn active_era(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::ActiveEraInfo,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"ActiveEra",
						vec![],
						[
							24u8, 229u8, 66u8, 56u8, 111u8, 234u8, 139u8, 93u8, 245u8, 137u8,
							110u8, 110u8, 121u8, 15u8, 216u8, 207u8, 97u8, 120u8, 125u8, 45u8,
							61u8, 2u8, 50u8, 100u8, 3u8, 106u8, 12u8, 233u8, 123u8, 156u8, 145u8,
							38u8,
						],
					)
				}
				#[doc = " The session index at which the era start for the last `HISTORY_DEPTH` eras."]
				#[doc = ""]
				#[doc = " Note: This tracks the starting session (i.e. session index when era start being active)"]
				#[doc = " for the eras in `[CurrentEra - HISTORY_DEPTH, CurrentEra]`."]
				pub fn eras_start_session_index(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"ErasStartSessionIndex",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							72u8, 185u8, 246u8, 202u8, 79u8, 127u8, 173u8, 74u8, 216u8, 238u8,
							58u8, 82u8, 235u8, 222u8, 76u8, 144u8, 97u8, 84u8, 17u8, 164u8, 132u8,
							167u8, 24u8, 195u8, 175u8, 132u8, 156u8, 87u8, 234u8, 147u8, 103u8,
							58u8,
						],
					)
				}
				#[doc = " The session index at which the era start for the last `HISTORY_DEPTH` eras."]
				#[doc = ""]
				#[doc = " Note: This tracks the starting session (i.e. session index when era start being active)"]
				#[doc = " for the eras in `[CurrentEra - HISTORY_DEPTH, CurrentEra]`."]
				pub fn eras_start_session_index_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"ErasStartSessionIndex",
						Vec::new(),
						[
							72u8, 185u8, 246u8, 202u8, 79u8, 127u8, 173u8, 74u8, 216u8, 238u8,
							58u8, 82u8, 235u8, 222u8, 76u8, 144u8, 97u8, 84u8, 17u8, 164u8, 132u8,
							167u8, 24u8, 195u8, 175u8, 132u8, 156u8, 87u8, 234u8, 147u8, 103u8,
							58u8,
						],
					)
				}
				#[doc = " Exposure of validator at era."]
				#[doc = ""]
				#[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
				#[doc = ""]
				#[doc = " Is it removed after `HISTORY_DEPTH` eras."]
				#[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]
				pub fn eras_stakers(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::Exposure<
						::subxt::utils::AccountId32,
						::core::primitive::u128,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"ErasStakers",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							103u8, 38u8, 198u8, 91u8, 133u8, 9u8, 10u8, 201u8, 103u8, 169u8, 159u8,
							172u8, 59u8, 238u8, 21u8, 30u8, 140u8, 183u8, 160u8, 61u8, 36u8, 162u8,
							244u8, 61u8, 78u8, 33u8, 134u8, 176u8, 112u8, 153u8, 192u8, 252u8,
						],
					)
				}
				#[doc = " Exposure of validator at era."]
				#[doc = ""]
				#[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
				#[doc = ""]
				#[doc = " Is it removed after `HISTORY_DEPTH` eras."]
				#[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]
				pub fn eras_stakers_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::Exposure<
						::subxt::utils::AccountId32,
						::core::primitive::u128,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"ErasStakers",
						Vec::new(),
						[
							103u8, 38u8, 198u8, 91u8, 133u8, 9u8, 10u8, 201u8, 103u8, 169u8, 159u8,
							172u8, 59u8, 238u8, 21u8, 30u8, 140u8, 183u8, 160u8, 61u8, 36u8, 162u8,
							244u8, 61u8, 78u8, 33u8, 134u8, 176u8, 112u8, 153u8, 192u8, 252u8,
						],
					)
				}
				#[doc = " Clipped Exposure of validator at era."]
				#[doc = ""]
				#[doc = " This is similar to [`ErasStakers`] but number of nominators exposed is reduced to the"]
				#[doc = " `T::MaxNominatorRewardedPerValidator` biggest stakers."]
				#[doc = " (Note: the field `total` and `own` of the exposure remains unchanged)."]
				#[doc = " This is used to limit the i/o cost for the nominator payout."]
				#[doc = ""]
				#[doc = " This is keyed fist by the era index to allow bulk deletion and then the stash account."]
				#[doc = ""]
				#[doc = " Is it removed after `HISTORY_DEPTH` eras."]
				#[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]
				pub fn eras_stakers_clipped(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::Exposure<
						::subxt::utils::AccountId32,
						::core::primitive::u128,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"ErasStakersClipped",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							119u8, 253u8, 51u8, 32u8, 173u8, 173u8, 49u8, 121u8, 141u8, 128u8,
							219u8, 112u8, 173u8, 42u8, 145u8, 37u8, 8u8, 12u8, 27u8, 37u8, 232u8,
							187u8, 130u8, 227u8, 113u8, 111u8, 185u8, 197u8, 157u8, 136u8, 205u8,
							32u8,
						],
					)
				}
				#[doc = " Clipped Exposure of validator at era."]
				#[doc = ""]
				#[doc = " This is similar to [`ErasStakers`] but number of nominators exposed is reduced to the"]
				#[doc = " `T::MaxNominatorRewardedPerValidator` biggest stakers."]
				#[doc = " (Note: the field `total` and `own` of the exposure remains unchanged)."]
				#[doc = " This is used to limit the i/o cost for the nominator payout."]
				#[doc = ""]
				#[doc = " This is keyed fist by the era index to allow bulk deletion and then the stash account."]
				#[doc = ""]
				#[doc = " Is it removed after `HISTORY_DEPTH` eras."]
				#[doc = " If stakers hasn't been set or has been removed then empty exposure is returned."]
				pub fn eras_stakers_clipped_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::Exposure<
						::subxt::utils::AccountId32,
						::core::primitive::u128,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"ErasStakersClipped",
						Vec::new(),
						[
							119u8, 253u8, 51u8, 32u8, 173u8, 173u8, 49u8, 121u8, 141u8, 128u8,
							219u8, 112u8, 173u8, 42u8, 145u8, 37u8, 8u8, 12u8, 27u8, 37u8, 232u8,
							187u8, 130u8, 227u8, 113u8, 111u8, 185u8, 197u8, 157u8, 136u8, 205u8,
							32u8,
						],
					)
				}
				#[doc = " Similar to `ErasStakers`, this holds the preferences of validators."]
				#[doc = ""]
				#[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
				#[doc = ""]
				#[doc = " Is it removed after `HISTORY_DEPTH` eras."]
				pub fn eras_validator_prefs(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::ValidatorPrefs,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"ErasValidatorPrefs",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							201u8, 204u8, 230u8, 197u8, 37u8, 83u8, 124u8, 26u8, 10u8, 75u8, 164u8,
							102u8, 83u8, 24u8, 158u8, 127u8, 27u8, 173u8, 125u8, 63u8, 251u8,
							128u8, 239u8, 182u8, 115u8, 109u8, 13u8, 97u8, 211u8, 104u8, 189u8,
							127u8,
						],
					)
				}
				#[doc = " Similar to `ErasStakers`, this holds the preferences of validators."]
				#[doc = ""]
				#[doc = " This is keyed first by the era index to allow bulk deletion and then the stash account."]
				#[doc = ""]
				#[doc = " Is it removed after `HISTORY_DEPTH` eras."]
				pub fn eras_validator_prefs_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::ValidatorPrefs,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"ErasValidatorPrefs",
						Vec::new(),
						[
							201u8, 204u8, 230u8, 197u8, 37u8, 83u8, 124u8, 26u8, 10u8, 75u8, 164u8,
							102u8, 83u8, 24u8, 158u8, 127u8, 27u8, 173u8, 125u8, 63u8, 251u8,
							128u8, 239u8, 182u8, 115u8, 109u8, 13u8, 97u8, 211u8, 104u8, 189u8,
							127u8,
						],
					)
				}
				#[doc = " The total validator era payout for the last `HISTORY_DEPTH` eras."]
				#[doc = ""]
				#[doc = " Eras that haven't finished yet or has been removed doesn't have reward."]
				pub fn eras_validator_reward(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"ErasValidatorReward",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							185u8, 85u8, 179u8, 163u8, 178u8, 168u8, 141u8, 200u8, 59u8, 77u8, 2u8,
							197u8, 36u8, 188u8, 133u8, 117u8, 2u8, 25u8, 105u8, 132u8, 44u8, 75u8,
							15u8, 82u8, 57u8, 89u8, 242u8, 234u8, 70u8, 244u8, 198u8, 126u8,
						],
					)
				}
				#[doc = " The total validator era payout for the last `HISTORY_DEPTH` eras."]
				#[doc = ""]
				#[doc = " Eras that haven't finished yet or has been removed doesn't have reward."]
				pub fn eras_validator_reward_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"ErasValidatorReward",
						Vec::new(),
						[
							185u8, 85u8, 179u8, 163u8, 178u8, 168u8, 141u8, 200u8, 59u8, 77u8, 2u8,
							197u8, 36u8, 188u8, 133u8, 117u8, 2u8, 25u8, 105u8, 132u8, 44u8, 75u8,
							15u8, 82u8, 57u8, 89u8, 242u8, 234u8, 70u8, 244u8, 198u8, 126u8,
						],
					)
				}
				#[doc = " Rewards for the last `HISTORY_DEPTH` eras."]
				#[doc = " If reward hasn't been set or has been removed then 0 reward is returned."]
				pub fn eras_reward_points(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::EraRewardPoints<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"ErasRewardPoints",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							237u8, 135u8, 146u8, 156u8, 172u8, 48u8, 147u8, 207u8, 15u8, 86u8,
							55u8, 38u8, 29u8, 253u8, 198u8, 192u8, 99u8, 213u8, 80u8, 72u8, 212u8,
							60u8, 60u8, 180u8, 33u8, 17u8, 77u8, 0u8, 165u8, 225u8, 60u8, 213u8,
						],
					)
				}
				#[doc = " Rewards for the last `HISTORY_DEPTH` eras."]
				#[doc = " If reward hasn't been set or has been removed then 0 reward is returned."]
				pub fn eras_reward_points_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::EraRewardPoints<::subxt::utils::AccountId32>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"ErasRewardPoints",
						Vec::new(),
						[
							237u8, 135u8, 146u8, 156u8, 172u8, 48u8, 147u8, 207u8, 15u8, 86u8,
							55u8, 38u8, 29u8, 253u8, 198u8, 192u8, 99u8, 213u8, 80u8, 72u8, 212u8,
							60u8, 60u8, 180u8, 33u8, 17u8, 77u8, 0u8, 165u8, 225u8, 60u8, 213u8,
						],
					)
				}
				#[doc = " The total amount staked for the last `HISTORY_DEPTH` eras."]
				#[doc = " If total hasn't been set or has been removed then 0 stake is returned."]
				pub fn eras_total_stake(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"ErasTotalStake",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							8u8, 78u8, 101u8, 62u8, 124u8, 126u8, 66u8, 26u8, 47u8, 126u8, 239u8,
							204u8, 222u8, 104u8, 19u8, 108u8, 238u8, 160u8, 112u8, 242u8, 56u8,
							2u8, 250u8, 164u8, 250u8, 213u8, 201u8, 84u8, 193u8, 117u8, 108u8,
							146u8,
						],
					)
				}
				#[doc = " The total amount staked for the last `HISTORY_DEPTH` eras."]
				#[doc = " If total hasn't been set or has been removed then 0 stake is returned."]
				pub fn eras_total_stake_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"ErasTotalStake",
						Vec::new(),
						[
							8u8, 78u8, 101u8, 62u8, 124u8, 126u8, 66u8, 26u8, 47u8, 126u8, 239u8,
							204u8, 222u8, 104u8, 19u8, 108u8, 238u8, 160u8, 112u8, 242u8, 56u8,
							2u8, 250u8, 164u8, 250u8, 213u8, 201u8, 84u8, 193u8, 117u8, 108u8,
							146u8,
						],
					)
				}
				#[doc = " Mode of era forcing."]
				pub fn force_era(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::Forcing,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"ForceEra",
						vec![],
						[
							177u8, 148u8, 73u8, 108u8, 136u8, 126u8, 89u8, 18u8, 124u8, 66u8, 30u8,
							102u8, 133u8, 164u8, 78u8, 214u8, 184u8, 163u8, 75u8, 164u8, 117u8,
							233u8, 209u8, 158u8, 99u8, 208u8, 21u8, 194u8, 152u8, 82u8, 16u8,
							222u8,
						],
					)
				}
				#[doc = " The percentage of the slash that is distributed to reporters."]
				#[doc = ""]
				#[doc = " The rest of the slashed value is handled by the `Slash`."]
				pub fn slash_reward_fraction(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::per_things::Perbill,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"SlashRewardFraction",
						vec![],
						[
							53u8, 88u8, 253u8, 237u8, 84u8, 228u8, 187u8, 130u8, 108u8, 195u8,
							135u8, 25u8, 75u8, 52u8, 238u8, 62u8, 133u8, 38u8, 139u8, 129u8, 216u8,
							193u8, 197u8, 216u8, 245u8, 171u8, 128u8, 207u8, 125u8, 246u8, 248u8,
							7u8,
						],
					)
				}
				#[doc = " The amount of currency given to reporters of a slash event which was"]
				#[doc = " canceled by extraordinary circumstances (e.g. governance)."]
				pub fn canceled_slash_payout(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"CanceledSlashPayout",
						vec![],
						[
							221u8, 88u8, 134u8, 81u8, 22u8, 229u8, 100u8, 27u8, 86u8, 244u8, 229u8,
							107u8, 251u8, 119u8, 58u8, 153u8, 19u8, 20u8, 254u8, 169u8, 248u8,
							220u8, 98u8, 118u8, 48u8, 213u8, 22u8, 79u8, 242u8, 250u8, 147u8,
							173u8,
						],
					)
				}
				#[doc = " All unapplied slashes that are queued for later."]
				pub fn unapplied_slashes(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<
						runtime_types::pallet_staking::UnappliedSlash<
							::subxt::utils::AccountId32,
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"UnappliedSlashes",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							121u8, 1u8, 135u8, 243u8, 99u8, 254u8, 238u8, 207u8, 145u8, 172u8,
							186u8, 131u8, 181u8, 109u8, 199u8, 93u8, 129u8, 65u8, 106u8, 118u8,
							197u8, 83u8, 65u8, 45u8, 149u8, 1u8, 85u8, 99u8, 239u8, 148u8, 40u8,
							177u8,
						],
					)
				}
				#[doc = " All unapplied slashes that are queued for later."]
				pub fn unapplied_slashes_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<
						runtime_types::pallet_staking::UnappliedSlash<
							::subxt::utils::AccountId32,
							::core::primitive::u128,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"UnappliedSlashes",
						Vec::new(),
						[
							121u8, 1u8, 135u8, 243u8, 99u8, 254u8, 238u8, 207u8, 145u8, 172u8,
							186u8, 131u8, 181u8, 109u8, 199u8, 93u8, 129u8, 65u8, 106u8, 118u8,
							197u8, 83u8, 65u8, 45u8, 149u8, 1u8, 85u8, 99u8, 239u8, 148u8, 40u8,
							177u8,
						],
					)
				}
				#[doc = " A mapping from still-bonded eras to the first session index of that era."]
				#[doc = ""]
				#[doc = " Must contains information for eras for the range:"]
				#[doc = " `[active_era - bounding_duration; active_era]`"]
				pub fn bonded_eras(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"BondedEras",
						vec![],
						[
							187u8, 216u8, 245u8, 253u8, 194u8, 182u8, 60u8, 244u8, 203u8, 84u8,
							228u8, 163u8, 149u8, 205u8, 57u8, 176u8, 203u8, 156u8, 20u8, 29u8,
							52u8, 234u8, 200u8, 63u8, 88u8, 49u8, 89u8, 117u8, 252u8, 75u8, 172u8,
							53u8,
						],
					)
				}
				#[doc = " All slashing events on validators, mapped by era to the highest slash proportion"]
				#[doc = " and slash value of the era."]
				pub fn validator_slash_in_era(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::sp_arithmetic::per_things::Perbill,
						::core::primitive::u128,
					),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"ValidatorSlashInEra",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							224u8, 141u8, 93u8, 44u8, 47u8, 157u8, 205u8, 12u8, 68u8, 41u8, 221u8,
							210u8, 141u8, 225u8, 253u8, 22u8, 175u8, 11u8, 92u8, 76u8, 180u8, 4u8,
							106u8, 135u8, 166u8, 47u8, 201u8, 43u8, 165u8, 42u8, 232u8, 219u8,
						],
					)
				}
				#[doc = " All slashing events on validators, mapped by era to the highest slash proportion"]
				#[doc = " and slash value of the era."]
				pub fn validator_slash_in_era_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						runtime_types::sp_arithmetic::per_things::Perbill,
						::core::primitive::u128,
					),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"ValidatorSlashInEra",
						Vec::new(),
						[
							224u8, 141u8, 93u8, 44u8, 47u8, 157u8, 205u8, 12u8, 68u8, 41u8, 221u8,
							210u8, 141u8, 225u8, 253u8, 22u8, 175u8, 11u8, 92u8, 76u8, 180u8, 4u8,
							106u8, 135u8, 166u8, 47u8, 201u8, 43u8, 165u8, 42u8, 232u8, 219u8,
						],
					)
				}
				#[doc = " All slashing events on nominators, mapped by era to the highest slash value of the era."]
				pub fn nominator_slash_in_era(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"NominatorSlashInEra",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							8u8, 89u8, 171u8, 183u8, 64u8, 29u8, 44u8, 185u8, 11u8, 204u8, 67u8,
							60u8, 208u8, 132u8, 9u8, 214u8, 13u8, 148u8, 205u8, 26u8, 5u8, 7u8,
							250u8, 191u8, 83u8, 118u8, 95u8, 17u8, 40u8, 126u8, 16u8, 135u8,
						],
					)
				}
				#[doc = " All slashing events on nominators, mapped by era to the highest slash value of the era."]
				pub fn nominator_slash_in_era_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"NominatorSlashInEra",
						Vec::new(),
						[
							8u8, 89u8, 171u8, 183u8, 64u8, 29u8, 44u8, 185u8, 11u8, 204u8, 67u8,
							60u8, 208u8, 132u8, 9u8, 214u8, 13u8, 148u8, 205u8, 26u8, 5u8, 7u8,
							250u8, 191u8, 83u8, 118u8, 95u8, 17u8, 40u8, 126u8, 16u8, 135u8,
						],
					)
				}
				#[doc = " Slashing spans for stash accounts."]
				pub fn slashing_spans(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::slashing::SlashingSpans,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"SlashingSpans",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							160u8, 190u8, 57u8, 128u8, 105u8, 73u8, 194u8, 75u8, 12u8, 120u8,
							141u8, 190u8, 235u8, 250u8, 221u8, 200u8, 141u8, 162u8, 31u8, 85u8,
							239u8, 108u8, 200u8, 148u8, 155u8, 48u8, 44u8, 89u8, 5u8, 177u8, 236u8,
							182u8,
						],
					)
				}
				#[doc = " Slashing spans for stash accounts."]
				pub fn slashing_spans_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::slashing::SlashingSpans,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"SlashingSpans",
						Vec::new(),
						[
							160u8, 190u8, 57u8, 128u8, 105u8, 73u8, 194u8, 75u8, 12u8, 120u8,
							141u8, 190u8, 235u8, 250u8, 221u8, 200u8, 141u8, 162u8, 31u8, 85u8,
							239u8, 108u8, 200u8, 148u8, 155u8, 48u8, 44u8, 89u8, 5u8, 177u8, 236u8,
							182u8,
						],
					)
				}
				#[doc = " Records information about the maximum slash of a stash within a slashing span,"]
				#[doc = " as well as how much reward has been paid out."]
				pub fn span_slash(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::slashing::SpanRecord<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"SpanSlash",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							6u8, 241u8, 205u8, 89u8, 62u8, 181u8, 211u8, 216u8, 190u8, 41u8, 81u8,
							136u8, 136u8, 139u8, 57u8, 243u8, 174u8, 150u8, 132u8, 211u8, 79u8,
							138u8, 108u8, 218u8, 19u8, 225u8, 60u8, 26u8, 135u8, 6u8, 21u8, 116u8,
						],
					)
				}
				#[doc = " Records information about the maximum slash of a stash within a slashing span,"]
				#[doc = " as well as how much reward has been paid out."]
				pub fn span_slash_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_staking::slashing::SpanRecord<::core::primitive::u128>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"SpanSlash",
						Vec::new(),
						[
							6u8, 241u8, 205u8, 89u8, 62u8, 181u8, 211u8, 216u8, 190u8, 41u8, 81u8,
							136u8, 136u8, 139u8, 57u8, 243u8, 174u8, 150u8, 132u8, 211u8, 79u8,
							138u8, 108u8, 218u8, 19u8, 225u8, 60u8, 26u8, 135u8, 6u8, 21u8, 116u8,
						],
					)
				}
				#[doc = " The last planned session scheduled by the session pallet."]
				#[doc = ""]
				#[doc = " This is basically in sync with the call to [`pallet_session::SessionManager::new_session`]."]
				pub fn current_planned_session(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"CurrentPlannedSession",
						vec![],
						[
							12u8, 47u8, 20u8, 104u8, 155u8, 181u8, 35u8, 91u8, 172u8, 97u8, 206u8,
							135u8, 185u8, 142u8, 46u8, 72u8, 32u8, 118u8, 225u8, 191u8, 28u8,
							130u8, 7u8, 38u8, 181u8, 233u8, 201u8, 8u8, 160u8, 161u8, 86u8, 204u8,
						],
					)
				}
				#[doc = " Indices of validators that have offended in the active era and whether they are currently"]
				#[doc = " disabled."]
				#[doc = ""]
				#[doc = " This value should be a superset of disabled validators since not all offences lead to the"]
				#[doc = " validator being disabled (if there was no slash). This is needed to track the percentage of"]
				#[doc = " validators that have offended in the current era, ensuring a new era is forced if"]
				#[doc = " `OffendingValidatorsThreshold` is reached. The vec is always kept sorted so that we can find"]
				#[doc = " whether a given validator has previously offended using binary search. It gets cleared when"]
				#[doc = " the era ends."]
				pub fn offending_validators(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(::core::primitive::u32, ::core::primitive::bool)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"OffendingValidators",
						vec![],
						[
							201u8, 31u8, 141u8, 182u8, 160u8, 180u8, 37u8, 226u8, 50u8, 65u8,
							103u8, 11u8, 38u8, 120u8, 200u8, 219u8, 219u8, 98u8, 185u8, 137u8,
							154u8, 20u8, 130u8, 163u8, 126u8, 185u8, 33u8, 194u8, 76u8, 172u8,
							70u8, 220u8,
						],
					)
				}
				#[doc = " The threshold for when users can start calling `chill_other` for other validators /"]
				#[doc = " nominators. The threshold is compared to the actual number of validators / nominators"]
				#[doc = " (`CountFor*`) in the system compared to the configured max (`Max*Count`)."]
				pub fn chill_threshold(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::per_things::Percent,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Staking",
						"ChillThreshold",
						vec![],
						[
							133u8, 222u8, 1u8, 208u8, 212u8, 216u8, 247u8, 66u8, 178u8, 96u8, 35u8,
							112u8, 33u8, 245u8, 11u8, 249u8, 255u8, 212u8, 204u8, 161u8, 44u8,
							38u8, 126u8, 151u8, 140u8, 42u8, 253u8, 101u8, 1u8, 23u8, 239u8, 39u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Maximum number of nominations per nominator."]
				pub fn max_nominations(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Staking",
						"MaxNominations",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Number of eras to keep in history."]
				#[doc = ""]
				#[doc = " Following information is kept for eras in `[current_era -"]
				#[doc = " HistoryDepth, current_era]`: `ErasStakers`, `ErasStakersClipped`,"]
				#[doc = " `ErasValidatorPrefs`, `ErasValidatorReward`, `ErasRewardPoints`,"]
				#[doc = " `ErasTotalStake`, `ErasStartSessionIndex`,"]
				#[doc = " `StakingLedger.claimed_rewards`."]
				#[doc = ""]
				#[doc = " Must be more than the number of eras delayed by session."]
				#[doc = " I.e. active era must always be in history. I.e. `active_era >"]
				#[doc = " current_era - history_depth` must be guaranteed."]
				#[doc = ""]
				#[doc = " If migrating an existing pallet from storage value to config value,"]
				#[doc = " this should be set to same value or greater as in storage."]
				#[doc = ""]
				#[doc = " Note: `HistoryDepth` is used as the upper bound for the `BoundedVec`"]
				#[doc = " item `StakingLedger.claimed_rewards`. Setting this value lower than"]
				#[doc = " the existing value can lead to inconsistencies in the"]
				#[doc = " `StakingLedger` and will need to be handled properly in a migration."]
				#[doc = " The test `reducing_history_depth_abrupt` shows this effect."]
				pub fn history_depth(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Staking",
						"HistoryDepth",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Number of sessions per era."]
				pub fn sessions_per_era(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Staking",
						"SessionsPerEra",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Number of eras that staked funds must remain bonded for."]
				pub fn bonding_duration(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Staking",
						"BondingDuration",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Number of eras that slashes are deferred by, after computation."]
				#[doc = ""]
				#[doc = " This should be less than the bonding duration. Set to 0 if slashes"]
				#[doc = " should be applied immediately, without opportunity for intervention."]
				pub fn slash_defer_duration(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Staking",
						"SlashDeferDuration",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of nominators rewarded for each validator."]
				#[doc = ""]
				#[doc = " For each validator only the `$MaxNominatorRewardedPerValidator` biggest stakers can"]
				#[doc = " claim their reward. This used to limit the i/o cost for the nominator payout."]
				pub fn max_nominator_rewarded_per_validator(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Staking",
						"MaxNominatorRewardedPerValidator",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of `unlocking` chunks a [`StakingLedger`] can"]
				#[doc = " have. Effectively determines how many unique eras a staker may be"]
				#[doc = " unbonding in."]
				#[doc = ""]
				#[doc = " Note: `MaxUnlockingChunks` is used as the upper bound for the"]
				#[doc = " `BoundedVec` item `StakingLedger.unlocking`. Setting this value"]
				#[doc = " lower than the existing value can lead to inconsistencies in the"]
				#[doc = " `StakingLedger` and will need to be handled properly in a runtime"]
				#[doc = " migration. The test `reducing_max_unlocking_chunks_abrupt` shows"]
				#[doc = " this effect."]
				pub fn max_unlocking_chunks(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Staking",
						"MaxUnlockingChunks",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod session {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Error for the session pallet."]
		pub type Error = runtime_types::pallet_session::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_session::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetKeys {
					pub keys: runtime_types::da_runtime::primitives::SessionKeys,
					pub proof: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetKeys {
					const PALLET: &'static str = "Session";
					const CALL: &'static str = "set_keys";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PurgeKeys;
				impl ::subxt::blocks::StaticExtrinsic for PurgeKeys {
					const PALLET: &'static str = "Session";
					const CALL: &'static str = "purge_keys";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::set_keys`]."]
				pub fn set_keys(
					&self,
					keys: runtime_types::da_runtime::primitives::SessionKeys,
					proof: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::SetKeys> {
					::subxt::tx::Payload::new_static(
						"Session",
						"set_keys",
						types::SetKeys { keys, proof },
						[
							249u8, 76u8, 61u8, 94u8, 145u8, 109u8, 28u8, 115u8, 21u8, 162u8, 253u8,
							79u8, 254u8, 5u8, 223u8, 195u8, 75u8, 96u8, 197u8, 10u8, 176u8, 69u8,
							84u8, 65u8, 68u8, 255u8, 239u8, 79u8, 228u8, 247u8, 171u8, 73u8,
						],
					)
				}
				#[doc = "See [`Pallet::purge_keys`]."]
				pub fn purge_keys(&self) -> ::subxt::tx::Payload<types::PurgeKeys> {
					::subxt::tx::Payload::new_static(
						"Session",
						"purge_keys",
						types::PurgeKeys {},
						[
							215u8, 204u8, 146u8, 236u8, 32u8, 78u8, 198u8, 79u8, 85u8, 214u8, 15u8,
							151u8, 158u8, 31u8, 146u8, 119u8, 119u8, 204u8, 151u8, 169u8, 226u8,
							67u8, 217u8, 39u8, 241u8, 245u8, 203u8, 240u8, 203u8, 172u8, 16u8,
							209u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_session::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "New session has happened. Note that the argument is the session index, not the"]
			#[doc = "block number as the type might suggest."]
			pub struct NewSession {
				pub session_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for NewSession {
				const PALLET: &'static str = "Session";
				const EVENT: &'static str = "NewSession";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current set of validators."]
				pub fn validators(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"Validators",
						vec![],
						[
							50u8, 86u8, 154u8, 222u8, 249u8, 209u8, 156u8, 22u8, 155u8, 25u8,
							133u8, 194u8, 210u8, 50u8, 38u8, 28u8, 139u8, 201u8, 90u8, 139u8,
							115u8, 12u8, 12u8, 141u8, 4u8, 178u8, 201u8, 241u8, 223u8, 234u8, 6u8,
							86u8,
						],
					)
				}
				#[doc = " Current index of the session."]
				pub fn current_index(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"CurrentIndex",
						vec![],
						[
							167u8, 151u8, 125u8, 150u8, 159u8, 21u8, 78u8, 217u8, 237u8, 183u8,
							135u8, 65u8, 187u8, 114u8, 188u8, 206u8, 16u8, 32u8, 69u8, 208u8,
							134u8, 159u8, 232u8, 224u8, 243u8, 27u8, 31u8, 166u8, 145u8, 44u8,
							221u8, 230u8,
						],
					)
				}
				#[doc = " True if the underlying economic identities or weighting behind the validators"]
				#[doc = " has changed in the queued validator set."]
				pub fn queued_changed(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"QueuedChanged",
						vec![],
						[
							184u8, 137u8, 224u8, 137u8, 31u8, 236u8, 95u8, 164u8, 102u8, 225u8,
							198u8, 227u8, 140u8, 37u8, 113u8, 57u8, 59u8, 4u8, 202u8, 102u8, 117u8,
							36u8, 226u8, 64u8, 113u8, 141u8, 199u8, 111u8, 99u8, 144u8, 198u8,
							153u8,
						],
					)
				}
				#[doc = " The queued keys for the next session. When the next session begins, these keys"]
				#[doc = " will be used to determine the validator's session keys."]
				pub fn queued_keys(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(
						::subxt::utils::AccountId32,
						runtime_types::da_runtime::primitives::SessionKeys,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"QueuedKeys",
						vec![],
						[
							253u8, 189u8, 69u8, 24u8, 103u8, 84u8, 10u8, 154u8, 177u8, 219u8,
							105u8, 118u8, 254u8, 193u8, 177u8, 122u8, 59u8, 95u8, 81u8, 128u8,
							252u8, 140u8, 164u8, 86u8, 8u8, 84u8, 199u8, 123u8, 212u8, 27u8, 68u8,
							29u8,
						],
					)
				}
				#[doc = " Indices of disabled validators."]
				#[doc = ""]
				#[doc = " The vec is always kept sorted so that we can find whether a given validator is"]
				#[doc = " disabled using binary search. It gets cleared when `on_session_ending` returns"]
				#[doc = " a new set of identities."]
				pub fn disabled_validators(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"DisabledValidators",
						vec![],
						[
							213u8, 19u8, 168u8, 234u8, 187u8, 200u8, 180u8, 97u8, 234u8, 189u8,
							36u8, 233u8, 158u8, 184u8, 45u8, 35u8, 129u8, 213u8, 133u8, 8u8, 104u8,
							183u8, 46u8, 68u8, 154u8, 240u8, 132u8, 22u8, 247u8, 11u8, 54u8, 221u8,
						],
					)
				}
				#[doc = " The next session keys for a validator."]
				pub fn next_keys(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::da_runtime::primitives::SessionKeys,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"NextKeys",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							27u8, 24u8, 147u8, 49u8, 85u8, 69u8, 22u8, 49u8, 162u8, 194u8, 189u8,
							109u8, 68u8, 151u8, 200u8, 43u8, 23u8, 251u8, 43u8, 102u8, 85u8, 236u8,
							95u8, 195u8, 49u8, 237u8, 236u8, 208u8, 37u8, 5u8, 84u8, 45u8,
						],
					)
				}
				#[doc = " The next session keys for a validator."]
				pub fn next_keys_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::da_runtime::primitives::SessionKeys,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"NextKeys",
						Vec::new(),
						[
							27u8, 24u8, 147u8, 49u8, 85u8, 69u8, 22u8, 49u8, 162u8, 194u8, 189u8,
							109u8, 68u8, 151u8, 200u8, 43u8, 23u8, 251u8, 43u8, 102u8, 85u8, 236u8,
							95u8, 195u8, 49u8, 237u8, 236u8, 208u8, 37u8, 5u8, 84u8, 45u8,
						],
					)
				}
				#[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
				pub fn key_owner(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::KeyTypeId>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"KeyOwner",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							177u8, 90u8, 148u8, 24u8, 251u8, 26u8, 65u8, 235u8, 46u8, 25u8, 109u8,
							212u8, 208u8, 218u8, 58u8, 196u8, 29u8, 73u8, 145u8, 41u8, 30u8, 251u8,
							185u8, 26u8, 205u8, 50u8, 32u8, 200u8, 206u8, 178u8, 255u8, 146u8,
						],
					)
				}
				#[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
				pub fn key_owner_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"KeyOwner",
						Vec::new(),
						[
							177u8, 90u8, 148u8, 24u8, 251u8, 26u8, 65u8, 235u8, 46u8, 25u8, 109u8,
							212u8, 208u8, 218u8, 58u8, 196u8, 29u8, 73u8, 145u8, 41u8, 30u8, 251u8,
							185u8, 26u8, 205u8, 50u8, 32u8, 200u8, 206u8, 178u8, 255u8, 146u8,
						],
					)
				}
			}
		}
	}
	pub mod technical_committee {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_collective::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_collective::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetMembers {
					pub new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
					pub prime: ::core::option::Option<::subxt::utils::AccountId32>,
					pub old_count: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetMembers {
					const PALLET: &'static str = "TechnicalCommittee";
					const CALL: &'static str = "set_members";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Execute {
					pub proposal: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					#[codec(compact)]
					pub length_bound: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Execute {
					const PALLET: &'static str = "TechnicalCommittee";
					const CALL: &'static str = "execute";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Propose {
					#[codec(compact)]
					pub threshold: ::core::primitive::u32,
					pub proposal: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					#[codec(compact)]
					pub length_bound: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Propose {
					const PALLET: &'static str = "TechnicalCommittee";
					const CALL: &'static str = "propose";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Vote {
					pub proposal: ::subxt::utils::H256,
					#[codec(compact)]
					pub index: ::core::primitive::u32,
					pub approve: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for Vote {
					const PALLET: &'static str = "TechnicalCommittee";
					const CALL: &'static str = "vote";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct DisapproveProposal {
					pub proposal_hash: ::subxt::utils::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for DisapproveProposal {
					const PALLET: &'static str = "TechnicalCommittee";
					const CALL: &'static str = "disapprove_proposal";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Close {
					pub proposal_hash: ::subxt::utils::H256,
					#[codec(compact)]
					pub index: ::core::primitive::u32,
					pub proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
					#[codec(compact)]
					pub length_bound: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Close {
					const PALLET: &'static str = "TechnicalCommittee";
					const CALL: &'static str = "close";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::set_members`]."]
				pub fn set_members(
					&self,
					new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
					prime: ::core::option::Option<::subxt::utils::AccountId32>,
					old_count: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::SetMembers> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommittee",
						"set_members",
						types::SetMembers {
							new_members,
							prime,
							old_count,
						},
						[
							141u8, 113u8, 137u8, 46u8, 75u8, 22u8, 143u8, 204u8, 50u8, 24u8, 137u8,
							25u8, 226u8, 166u8, 121u8, 161u8, 54u8, 144u8, 12u8, 145u8, 157u8,
							153u8, 47u8, 144u8, 94u8, 34u8, 217u8, 115u8, 125u8, 152u8, 110u8,
							28u8,
						],
					)
				}
				#[doc = "See [`Pallet::execute`]."]
				pub fn execute(
					&self,
					proposal: runtime_types::da_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Execute> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommittee",
						"execute",
						types::Execute {
							proposal: ::std::boxed::Box::new(proposal),
							length_bound,
						},
						[
							147u8, 68u8, 21u8, 174u8, 67u8, 231u8, 185u8, 37u8, 221u8, 148u8,
							149u8, 223u8, 234u8, 141u8, 90u8, 37u8, 109u8, 240u8, 103u8, 232u8,
							215u8, 85u8, 116u8, 15u8, 1u8, 15u8, 236u8, 12u8, 77u8, 181u8, 46u8,
							145u8,
						],
					)
				}
				#[doc = "See [`Pallet::propose`]."]
				pub fn propose(
					&self,
					threshold: ::core::primitive::u32,
					proposal: runtime_types::da_runtime::RuntimeCall,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Propose> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommittee",
						"propose",
						types::Propose {
							threshold,
							proposal: ::std::boxed::Box::new(proposal),
							length_bound,
						},
						[
							201u8, 234u8, 90u8, 229u8, 124u8, 103u8, 217u8, 254u8, 204u8, 10u8,
							14u8, 9u8, 62u8, 3u8, 30u8, 51u8, 132u8, 72u8, 141u8, 88u8, 251u8,
							248u8, 237u8, 132u8, 194u8, 99u8, 182u8, 241u8, 97u8, 106u8, 10u8,
							67u8,
						],
					)
				}
				#[doc = "See [`Pallet::vote`]."]
				pub fn vote(
					&self,
					proposal: ::subxt::utils::H256,
					index: ::core::primitive::u32,
					approve: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::Vote> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommittee",
						"vote",
						types::Vote {
							proposal,
							index,
							approve,
						},
						[
							110u8, 141u8, 24u8, 33u8, 91u8, 7u8, 89u8, 198u8, 54u8, 10u8, 76u8,
							129u8, 45u8, 20u8, 216u8, 104u8, 231u8, 246u8, 174u8, 205u8, 190u8,
							176u8, 171u8, 113u8, 33u8, 37u8, 155u8, 203u8, 251u8, 34u8, 25u8,
							120u8,
						],
					)
				}
				#[doc = "See [`Pallet::disapprove_proposal`]."]
				pub fn disapprove_proposal(
					&self,
					proposal_hash: ::subxt::utils::H256,
				) -> ::subxt::tx::Payload<types::DisapproveProposal> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommittee",
						"disapprove_proposal",
						types::DisapproveProposal { proposal_hash },
						[
							26u8, 140u8, 111u8, 193u8, 229u8, 59u8, 53u8, 196u8, 230u8, 60u8, 7u8,
							155u8, 168u8, 7u8, 201u8, 177u8, 70u8, 103u8, 190u8, 57u8, 244u8,
							156u8, 67u8, 101u8, 228u8, 6u8, 213u8, 83u8, 225u8, 95u8, 148u8, 96u8,
						],
					)
				}
				#[doc = "See [`Pallet::close`]."]
				pub fn close(
					&self,
					proposal_hash: ::subxt::utils::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Close> {
					::subxt::tx::Payload::new_static(
						"TechnicalCommittee",
						"close",
						types::Close {
							proposal_hash,
							index,
							proposal_weight_bound,
							length_bound,
						},
						[
							189u8, 149u8, 125u8, 63u8, 39u8, 201u8, 247u8, 4u8, 220u8, 74u8, 78u8,
							14u8, 113u8, 163u8, 1u8, 159u8, 81u8, 248u8, 141u8, 111u8, 34u8, 243u8,
							67u8, 70u8, 60u8, 92u8, 47u8, 70u8, 66u8, 246u8, 236u8, 153u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_collective::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A motion (given hash) has been proposed (by given account) with a threshold (given"]
			#[doc = "`MemberCount`)."]
			pub struct Proposed {
				pub account: ::subxt::utils::AccountId32,
				pub proposal_index: ::core::primitive::u32,
				pub proposal_hash: ::subxt::utils::H256,
				pub threshold: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Proposed {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Proposed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A motion (given hash) has been voted on by given account, leaving"]
			#[doc = "a tally (yes votes and no votes given respectively as `MemberCount`)."]
			pub struct Voted {
				pub account: ::subxt::utils::AccountId32,
				pub proposal_hash: ::subxt::utils::H256,
				pub voted: ::core::primitive::bool,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Voted {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Voted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A motion was approved by the required threshold."]
			pub struct Approved {
				pub proposal_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Approved {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Approved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A motion was not approved by the required threshold."]
			pub struct Disapproved {
				pub proposal_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Disapproved {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Disapproved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A motion was executed; result will be `Ok` if it returned without error."]
			pub struct Executed {
				pub proposal_hash: ::subxt::utils::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Executed {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Executed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A single member did some action; result will be `Ok` if it returned without error."]
			pub struct MemberExecuted {
				pub proposal_hash: ::subxt::utils::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for MemberExecuted {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "MemberExecuted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A proposal was closed because its threshold was reached or after its duration was up."]
			pub struct Closed {
				pub proposal_hash: ::subxt::utils::H256,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Closed {
				const PALLET: &'static str = "TechnicalCommittee";
				const EVENT: &'static str = "Closed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The hashes of the active proposals."]
				pub fn proposals(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::subxt::utils::H256,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommittee",
						"Proposals",
						vec![],
						[
							210u8, 234u8, 7u8, 29u8, 231u8, 80u8, 17u8, 36u8, 189u8, 34u8, 175u8,
							147u8, 56u8, 92u8, 201u8, 104u8, 207u8, 150u8, 58u8, 110u8, 90u8, 28u8,
							198u8, 79u8, 236u8, 245u8, 19u8, 38u8, 68u8, 59u8, 215u8, 74u8,
						],
					)
				}
				#[doc = " Actual proposal for a given hash, if it's current."]
				pub fn proposal_of(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::da_runtime::RuntimeCall,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommittee",
						"ProposalOf",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							166u8, 138u8, 231u8, 52u8, 124u8, 126u8, 77u8, 69u8, 109u8, 4u8, 196u8,
							172u8, 114u8, 174u8, 25u8, 206u8, 4u8, 246u8, 11u8, 120u8, 43u8, 7u8,
							229u8, 101u8, 100u8, 127u8, 0u8, 175u8, 224u8, 180u8, 132u8, 196u8,
						],
					)
				}
				#[doc = " Actual proposal for a given hash, if it's current."]
				pub fn proposal_of_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::da_runtime::RuntimeCall,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommittee",
						"ProposalOf",
						Vec::new(),
						[
							166u8, 138u8, 231u8, 52u8, 124u8, 126u8, 77u8, 69u8, 109u8, 4u8, 196u8,
							172u8, 114u8, 174u8, 25u8, 206u8, 4u8, 246u8, 11u8, 120u8, 43u8, 7u8,
							229u8, 101u8, 100u8, 127u8, 0u8, 175u8, 224u8, 180u8, 132u8, 196u8,
						],
					)
				}
				#[doc = " Votes on a given proposal, if it is ongoing."]
				pub fn voting(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_collective::Votes<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommittee",
						"Voting",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							56u8, 192u8, 111u8, 180u8, 253u8, 5u8, 232u8, 126u8, 177u8, 48u8,
							135u8, 39u8, 89u8, 71u8, 62u8, 239u8, 216u8, 17u8, 64u8, 82u8, 130u8,
							236u8, 96u8, 89u8, 167u8, 2u8, 118u8, 113u8, 63u8, 176u8, 124u8, 73u8,
						],
					)
				}
				#[doc = " Votes on a given proposal, if it is ongoing."]
				pub fn voting_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_collective::Votes<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommittee",
						"Voting",
						Vec::new(),
						[
							56u8, 192u8, 111u8, 180u8, 253u8, 5u8, 232u8, 126u8, 177u8, 48u8,
							135u8, 39u8, 89u8, 71u8, 62u8, 239u8, 216u8, 17u8, 64u8, 82u8, 130u8,
							236u8, 96u8, 89u8, 167u8, 2u8, 118u8, 113u8, 63u8, 176u8, 124u8, 73u8,
						],
					)
				}
				#[doc = " Proposals so far."]
				pub fn proposal_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommittee",
						"ProposalCount",
						vec![],
						[
							91u8, 238u8, 246u8, 106u8, 95u8, 66u8, 83u8, 134u8, 1u8, 225u8, 164u8,
							216u8, 113u8, 101u8, 203u8, 200u8, 113u8, 97u8, 246u8, 228u8, 140u8,
							29u8, 29u8, 48u8, 176u8, 137u8, 93u8, 230u8, 56u8, 75u8, 51u8, 149u8,
						],
					)
				}
				#[doc = " The current members of the collective. This is stored sorted (just by value)."]
				pub fn members(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommittee",
						"Members",
						vec![],
						[
							16u8, 29u8, 32u8, 222u8, 175u8, 136u8, 111u8, 101u8, 43u8, 74u8, 209u8,
							81u8, 47u8, 97u8, 129u8, 39u8, 225u8, 243u8, 110u8, 229u8, 237u8, 21u8,
							90u8, 127u8, 80u8, 239u8, 156u8, 32u8, 90u8, 109u8, 179u8, 0u8,
						],
					)
				}
				#[doc = " The prime member that helps determine the default vote behavior in case of absentations."]
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalCommittee",
						"Prime",
						vec![],
						[
							72u8, 128u8, 214u8, 72u8, 78u8, 80u8, 100u8, 198u8, 114u8, 215u8, 59u8,
							3u8, 103u8, 14u8, 152u8, 202u8, 12u8, 165u8, 224u8, 10u8, 41u8, 154u8,
							77u8, 95u8, 116u8, 143u8, 250u8, 250u8, 176u8, 92u8, 238u8, 154u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The maximum weight of a dispatch call that can be proposed and executed."]
				pub fn max_proposal_weight(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_weights::weight_v2::Weight> {
					::subxt::constants::Address::new_static(
						"TechnicalCommittee",
						"MaxProposalWeight",
						[
							222u8, 183u8, 203u8, 169u8, 31u8, 134u8, 28u8, 12u8, 47u8, 140u8, 71u8,
							74u8, 61u8, 55u8, 71u8, 236u8, 215u8, 83u8, 28u8, 70u8, 45u8, 128u8,
							184u8, 57u8, 101u8, 83u8, 42u8, 165u8, 34u8, 155u8, 64u8, 145u8,
						],
					)
				}
			}
		}
	}
	pub mod technical_membership {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_membership::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_membership::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AddMember {
					pub who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for AddMember {
					const PALLET: &'static str = "TechnicalMembership";
					const CALL: &'static str = "add_member";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RemoveMember {
					pub who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemoveMember {
					const PALLET: &'static str = "TechnicalMembership";
					const CALL: &'static str = "remove_member";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SwapMember {
					pub remove: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					pub add: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SwapMember {
					const PALLET: &'static str = "TechnicalMembership";
					const CALL: &'static str = "swap_member";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ResetMembers {
					pub members: ::std::vec::Vec<::subxt::utils::AccountId32>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ResetMembers {
					const PALLET: &'static str = "TechnicalMembership";
					const CALL: &'static str = "reset_members";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ChangeKey {
					pub new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ChangeKey {
					const PALLET: &'static str = "TechnicalMembership";
					const CALL: &'static str = "change_key";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetPrime {
					pub who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetPrime {
					const PALLET: &'static str = "TechnicalMembership";
					const CALL: &'static str = "set_prime";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ClearPrime;
				impl ::subxt::blocks::StaticExtrinsic for ClearPrime {
					const PALLET: &'static str = "TechnicalMembership";
					const CALL: &'static str = "clear_prime";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::add_member`]."]
				pub fn add_member(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::AddMember> {
					::subxt::tx::Payload::new_static(
						"TechnicalMembership",
						"add_member",
						types::AddMember { who },
						[
							138u8, 255u8, 102u8, 159u8, 20u8, 50u8, 254u8, 231u8, 96u8, 129u8,
							60u8, 16u8, 10u8, 245u8, 109u8, 255u8, 240u8, 120u8, 87u8, 88u8, 97u8,
							175u8, 3u8, 113u8, 237u8, 212u8, 69u8, 21u8, 112u8, 215u8, 89u8, 69u8,
						],
					)
				}
				#[doc = "See [`Pallet::remove_member`]."]
				pub fn remove_member(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::RemoveMember> {
					::subxt::tx::Payload::new_static(
						"TechnicalMembership",
						"remove_member",
						types::RemoveMember { who },
						[
							121u8, 104u8, 219u8, 37u8, 141u8, 96u8, 126u8, 220u8, 13u8, 241u8,
							206u8, 223u8, 111u8, 67u8, 117u8, 243u8, 74u8, 16u8, 209u8, 230u8,
							84u8, 15u8, 66u8, 106u8, 150u8, 113u8, 27u8, 26u8, 23u8, 152u8, 32u8,
							167u8,
						],
					)
				}
				#[doc = "See [`Pallet::swap_member`]."]
				pub fn swap_member(
					&self,
					remove: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					add: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::SwapMember> {
					::subxt::tx::Payload::new_static(
						"TechnicalMembership",
						"swap_member",
						types::SwapMember { remove, add },
						[
							59u8, 246u8, 217u8, 53u8, 173u8, 139u8, 220u8, 69u8, 94u8, 125u8,
							173u8, 93u8, 215u8, 1u8, 44u8, 208u8, 193u8, 18u8, 238u8, 34u8, 132u8,
							161u8, 174u8, 34u8, 135u8, 166u8, 130u8, 19u8, 108u8, 219u8, 211u8,
							202u8,
						],
					)
				}
				#[doc = "See [`Pallet::reset_members`]."]
				pub fn reset_members(
					&self,
					members: ::std::vec::Vec<::subxt::utils::AccountId32>,
				) -> ::subxt::tx::Payload<types::ResetMembers> {
					::subxt::tx::Payload::new_static(
						"TechnicalMembership",
						"reset_members",
						types::ResetMembers { members },
						[
							212u8, 144u8, 99u8, 156u8, 70u8, 4u8, 219u8, 227u8, 150u8, 25u8, 86u8,
							8u8, 215u8, 128u8, 193u8, 206u8, 33u8, 193u8, 71u8, 15u8, 20u8, 92u8,
							99u8, 89u8, 174u8, 236u8, 102u8, 82u8, 164u8, 234u8, 12u8, 45u8,
						],
					)
				}
				#[doc = "See [`Pallet::change_key`]."]
				pub fn change_key(
					&self,
					new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::ChangeKey> {
					::subxt::tx::Payload::new_static(
						"TechnicalMembership",
						"change_key",
						types::ChangeKey { new },
						[
							134u8, 113u8, 38u8, 177u8, 161u8, 80u8, 99u8, 236u8, 80u8, 200u8, 17u8,
							137u8, 246u8, 98u8, 143u8, 136u8, 177u8, 23u8, 177u8, 195u8, 36u8,
							57u8, 243u8, 197u8, 160u8, 225u8, 171u8, 114u8, 146u8, 233u8, 81u8,
							232u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_prime`]."]
				pub fn set_prime(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::SetPrime> {
					::subxt::tx::Payload::new_static(
						"TechnicalMembership",
						"set_prime",
						types::SetPrime { who },
						[
							209u8, 16u8, 241u8, 104u8, 226u8, 31u8, 133u8, 167u8, 151u8, 175u8,
							79u8, 192u8, 160u8, 21u8, 94u8, 171u8, 226u8, 187u8, 197u8, 102u8,
							156u8, 116u8, 36u8, 125u8, 139u8, 196u8, 175u8, 31u8, 22u8, 119u8,
							150u8, 53u8,
						],
					)
				}
				#[doc = "See [`Pallet::clear_prime`]."]
				pub fn clear_prime(&self) -> ::subxt::tx::Payload<types::ClearPrime> {
					::subxt::tx::Payload::new_static(
						"TechnicalMembership",
						"clear_prime",
						types::ClearPrime {},
						[
							71u8, 213u8, 34u8, 23u8, 186u8, 63u8, 240u8, 216u8, 190u8, 251u8, 84u8,
							109u8, 140u8, 137u8, 210u8, 211u8, 242u8, 231u8, 212u8, 133u8, 151u8,
							125u8, 25u8, 46u8, 210u8, 53u8, 133u8, 222u8, 21u8, 107u8, 120u8, 52u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_membership::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The given member was added; see the transaction for who."]
			pub struct MemberAdded;
			impl ::subxt::events::StaticEvent for MemberAdded {
				const PALLET: &'static str = "TechnicalMembership";
				const EVENT: &'static str = "MemberAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The given member was removed; see the transaction for who."]
			pub struct MemberRemoved;
			impl ::subxt::events::StaticEvent for MemberRemoved {
				const PALLET: &'static str = "TechnicalMembership";
				const EVENT: &'static str = "MemberRemoved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Two members were swapped; see the transaction for who."]
			pub struct MembersSwapped;
			impl ::subxt::events::StaticEvent for MembersSwapped {
				const PALLET: &'static str = "TechnicalMembership";
				const EVENT: &'static str = "MembersSwapped";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The membership was reset; see the transaction for who the new set is."]
			pub struct MembersReset;
			impl ::subxt::events::StaticEvent for MembersReset {
				const PALLET: &'static str = "TechnicalMembership";
				const EVENT: &'static str = "MembersReset";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "One of the members' keys changed."]
			pub struct KeyChanged;
			impl ::subxt::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "TechnicalMembership";
				const EVENT: &'static str = "KeyChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Phantom member, never used."]
			pub struct Dummy;
			impl ::subxt::events::StaticEvent for Dummy {
				const PALLET: &'static str = "TechnicalMembership";
				const EVENT: &'static str = "Dummy";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current membership, stored as an ordered Vec."]
				pub fn members(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::subxt::utils::AccountId32,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalMembership",
						"Members",
						vec![],
						[
							109u8, 100u8, 14u8, 195u8, 213u8, 67u8, 44u8, 218u8, 84u8, 254u8, 76u8,
							80u8, 210u8, 155u8, 155u8, 30u8, 18u8, 169u8, 195u8, 92u8, 208u8,
							223u8, 242u8, 97u8, 147u8, 20u8, 168u8, 145u8, 254u8, 115u8, 225u8,
							193u8,
						],
					)
				}
				#[doc = " The current prime member, if one exists."]
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TechnicalMembership",
						"Prime",
						vec![],
						[
							72u8, 128u8, 214u8, 72u8, 78u8, 80u8, 100u8, 198u8, 114u8, 215u8, 59u8,
							3u8, 103u8, 14u8, 152u8, 202u8, 12u8, 165u8, 224u8, 10u8, 41u8, 154u8,
							77u8, 95u8, 116u8, 143u8, 250u8, 250u8, 176u8, 92u8, 238u8, 154u8,
						],
					)
				}
			}
		}
	}
	pub mod grandpa {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_grandpa::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_grandpa::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ReportEquivocation {
					pub equivocation_proof: ::std::boxed::Box<
						runtime_types::sp_consensus_grandpa::EquivocationProof<
							::subxt::utils::H256,
							::core::primitive::u32,
						>,
					>,
					pub key_owner_proof: runtime_types::sp_session::MembershipProof,
				}
				impl ::subxt::blocks::StaticExtrinsic for ReportEquivocation {
					const PALLET: &'static str = "Grandpa";
					const CALL: &'static str = "report_equivocation";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ReportEquivocationUnsigned {
					pub equivocation_proof: ::std::boxed::Box<
						runtime_types::sp_consensus_grandpa::EquivocationProof<
							::subxt::utils::H256,
							::core::primitive::u32,
						>,
					>,
					pub key_owner_proof: runtime_types::sp_session::MembershipProof,
				}
				impl ::subxt::blocks::StaticExtrinsic for ReportEquivocationUnsigned {
					const PALLET: &'static str = "Grandpa";
					const CALL: &'static str = "report_equivocation_unsigned";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct NoteStalled {
					pub delay: ::core::primitive::u32,
					pub best_finalized_block_number: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for NoteStalled {
					const PALLET: &'static str = "Grandpa";
					const CALL: &'static str = "note_stalled";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::report_equivocation`]."]
				pub fn report_equivocation(
					&self,
					equivocation_proof: runtime_types::sp_consensus_grandpa::EquivocationProof<
						::subxt::utils::H256,
						::core::primitive::u32,
					>,
					key_owner_proof: runtime_types::sp_session::MembershipProof,
				) -> ::subxt::tx::Payload<types::ReportEquivocation> {
					::subxt::tx::Payload::new_static(
						"Grandpa",
						"report_equivocation",
						types::ReportEquivocation {
							equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
							key_owner_proof,
						},
						[
							239u8, 120u8, 210u8, 103u8, 106u8, 180u8, 41u8, 20u8, 164u8, 142u8,
							156u8, 209u8, 183u8, 254u8, 192u8, 178u8, 22u8, 64u8, 91u8, 4u8, 222u8,
							103u8, 37u8, 184u8, 252u8, 181u8, 65u8, 136u8, 103u8, 199u8, 250u8,
							66u8,
						],
					)
				}
				#[doc = "See [`Pallet::report_equivocation_unsigned`]."]
				pub fn report_equivocation_unsigned(
					&self,
					equivocation_proof: runtime_types::sp_consensus_grandpa::EquivocationProof<
						::subxt::utils::H256,
						::core::primitive::u32,
					>,
					key_owner_proof: runtime_types::sp_session::MembershipProof,
				) -> ::subxt::tx::Payload<types::ReportEquivocationUnsigned> {
					::subxt::tx::Payload::new_static(
						"Grandpa",
						"report_equivocation_unsigned",
						types::ReportEquivocationUnsigned {
							equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
							key_owner_proof,
						},
						[
							238u8, 22u8, 92u8, 27u8, 26u8, 218u8, 114u8, 129u8, 133u8, 211u8, 34u8,
							239u8, 8u8, 11u8, 62u8, 201u8, 29u8, 38u8, 231u8, 63u8, 204u8, 13u8,
							82u8, 164u8, 83u8, 149u8, 0u8, 0u8, 102u8, 113u8, 106u8, 156u8,
						],
					)
				}
				#[doc = "See [`Pallet::note_stalled`]."]
				pub fn note_stalled(
					&self,
					delay: ::core::primitive::u32,
					best_finalized_block_number: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::NoteStalled> {
					::subxt::tx::Payload::new_static(
						"Grandpa",
						"note_stalled",
						types::NoteStalled {
							delay,
							best_finalized_block_number,
						},
						[
							232u8, 162u8, 42u8, 199u8, 101u8, 116u8, 38u8, 27u8, 147u8, 15u8,
							224u8, 76u8, 229u8, 244u8, 13u8, 49u8, 218u8, 232u8, 253u8, 37u8, 7u8,
							222u8, 97u8, 158u8, 201u8, 199u8, 169u8, 218u8, 201u8, 136u8, 192u8,
							128u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_grandpa::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "New authority set has been applied."]
			pub struct NewAuthorities {
				pub authority_set: ::std::vec::Vec<(
					runtime_types::sp_consensus_grandpa::app::Public,
					::core::primitive::u64,
				)>,
			}
			impl ::subxt::events::StaticEvent for NewAuthorities {
				const PALLET: &'static str = "Grandpa";
				const EVENT: &'static str = "NewAuthorities";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Current authority set has been paused."]
			pub struct Paused;
			impl ::subxt::events::StaticEvent for Paused {
				const PALLET: &'static str = "Grandpa";
				const EVENT: &'static str = "Paused";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Current authority set has been resumed."]
			pub struct Resumed;
			impl ::subxt::events::StaticEvent for Resumed {
				const PALLET: &'static str = "Grandpa";
				const EVENT: &'static str = "Resumed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " State of the current authority set."]
				pub fn state(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"State",
						vec![],
						[
							254u8, 81u8, 54u8, 203u8, 26u8, 74u8, 162u8, 215u8, 165u8, 247u8,
							143u8, 139u8, 242u8, 164u8, 67u8, 27u8, 97u8, 172u8, 66u8, 98u8, 28u8,
							151u8, 32u8, 38u8, 209u8, 82u8, 41u8, 209u8, 72u8, 3u8, 167u8, 42u8,
						],
					)
				}
				#[doc = " Pending change: (signaled at, scheduled change)."]
				pub fn pending_change(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"PendingChange",
						vec![],
						[
							207u8, 134u8, 15u8, 77u8, 9u8, 253u8, 20u8, 132u8, 226u8, 115u8, 150u8,
							184u8, 18u8, 15u8, 143u8, 172u8, 71u8, 114u8, 221u8, 162u8, 174u8,
							205u8, 46u8, 144u8, 70u8, 116u8, 18u8, 105u8, 250u8, 44u8, 75u8, 27u8,
						],
					)
				}
				#[doc = " next block number where we can force a change."]
				pub fn next_forced(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"NextForced",
						vec![],
						[
							3u8, 231u8, 56u8, 18u8, 87u8, 112u8, 227u8, 126u8, 180u8, 131u8, 255u8,
							141u8, 82u8, 34u8, 61u8, 47u8, 234u8, 37u8, 95u8, 62u8, 33u8, 235u8,
							231u8, 122u8, 125u8, 8u8, 223u8, 95u8, 255u8, 204u8, 40u8, 97u8,
						],
					)
				}
				#[doc = " `true` if we are currently stalled."]
				pub fn stalled(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::core::primitive::u32, ::core::primitive::u32),
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"Stalled",
						vec![],
						[
							146u8, 18u8, 59u8, 59u8, 21u8, 246u8, 5u8, 167u8, 221u8, 8u8, 230u8,
							74u8, 81u8, 217u8, 67u8, 158u8, 136u8, 36u8, 23u8, 106u8, 136u8, 89u8,
							110u8, 217u8, 31u8, 138u8, 107u8, 251u8, 164u8, 10u8, 119u8, 18u8,
						],
					)
				}
				#[doc = " The number of changes (both in terms of keys and underlying economic responsibilities)"]
				#[doc = " in the \"set\" of Grandpa validators from genesis."]
				pub fn current_set_id(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"CurrentSetId",
						vec![],
						[
							234u8, 215u8, 218u8, 42u8, 30u8, 76u8, 129u8, 40u8, 125u8, 137u8,
							207u8, 47u8, 46u8, 213u8, 159u8, 50u8, 175u8, 81u8, 155u8, 123u8,
							246u8, 175u8, 156u8, 68u8, 22u8, 113u8, 135u8, 137u8, 163u8, 18u8,
							115u8, 73u8,
						],
					)
				}
				#[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
				#[doc = " members were responsible."]
				#[doc = ""]
				#[doc = " This is only used for validating equivocation proofs. An equivocation proof must"]
				#[doc = " contains a key-ownership proof for a given session, therefore we need a way to tie"]
				#[doc = " together sessions and GRANDPA set ids, i.e. we need to validate that a validator"]
				#[doc = " was the owner of a given key on a given session, and what the active set ID was"]
				#[doc = " during that session."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: `SetId` is not under user control."]
				pub fn set_id_session(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"SetIdSession",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							47u8, 0u8, 239u8, 121u8, 187u8, 213u8, 254u8, 50u8, 238u8, 10u8, 162u8,
							65u8, 189u8, 166u8, 37u8, 74u8, 82u8, 81u8, 160u8, 20u8, 180u8, 253u8,
							238u8, 18u8, 209u8, 203u8, 38u8, 148u8, 16u8, 105u8, 72u8, 169u8,
						],
					)
				}
				#[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
				#[doc = " members were responsible."]
				#[doc = ""]
				#[doc = " This is only used for validating equivocation proofs. An equivocation proof must"]
				#[doc = " contains a key-ownership proof for a given session, therefore we need a way to tie"]
				#[doc = " together sessions and GRANDPA set ids, i.e. we need to validate that a validator"]
				#[doc = " was the owner of a given key on a given session, and what the active set ID was"]
				#[doc = " during that session."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: `SetId` is not under user control."]
				pub fn set_id_session_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Grandpa",
						"SetIdSession",
						Vec::new(),
						[
							47u8, 0u8, 239u8, 121u8, 187u8, 213u8, 254u8, 50u8, 238u8, 10u8, 162u8,
							65u8, 189u8, 166u8, 37u8, 74u8, 82u8, 81u8, 160u8, 20u8, 180u8, 253u8,
							238u8, 18u8, 209u8, 203u8, 38u8, 148u8, 16u8, 105u8, 72u8, 169u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Max Authorities in use"]
				pub fn max_authorities(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Grandpa",
						"MaxAuthorities",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of entries to keep in the set id to session index mapping."]
				#[doc = ""]
				#[doc = " Since the `SetIdSession` map is only used for validating equivocations this"]
				#[doc = " value should relate to the bonding duration of whatever staking system is"]
				#[doc = " being used (if any). If equivocation handling is not enabled then this value"]
				#[doc = " can be zero."]
				pub fn max_set_id_session_entries(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"Grandpa",
						"MaxSetIdSessionEntries",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
			}
		}
	}
	pub mod treasury {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Error for the treasury pallet."]
		pub type Error = runtime_types::pallet_treasury::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_treasury::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ProposeSpend {
					#[codec(compact)]
					pub value: ::core::primitive::u128,
					pub beneficiary: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ProposeSpend {
					const PALLET: &'static str = "Treasury";
					const CALL: &'static str = "propose_spend";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RejectProposal {
					#[codec(compact)]
					pub proposal_id: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for RejectProposal {
					const PALLET: &'static str = "Treasury";
					const CALL: &'static str = "reject_proposal";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ApproveProposal {
					#[codec(compact)]
					pub proposal_id: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for ApproveProposal {
					const PALLET: &'static str = "Treasury";
					const CALL: &'static str = "approve_proposal";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Spend {
					#[codec(compact)]
					pub amount: ::core::primitive::u128,
					pub beneficiary: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Spend {
					const PALLET: &'static str = "Treasury";
					const CALL: &'static str = "spend";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RemoveApproval {
					#[codec(compact)]
					pub proposal_id: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemoveApproval {
					const PALLET: &'static str = "Treasury";
					const CALL: &'static str = "remove_approval";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::propose_spend`]."]
				pub fn propose_spend(
					&self,
					value: ::core::primitive::u128,
					beneficiary: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::ProposeSpend> {
					::subxt::tx::Payload::new_static(
						"Treasury",
						"propose_spend",
						types::ProposeSpend { value, beneficiary },
						[
							122u8, 208u8, 182u8, 218u8, 237u8, 127u8, 67u8, 90u8, 119u8, 187u8,
							190u8, 131u8, 226u8, 30u8, 123u8, 176u8, 71u8, 150u8, 85u8, 170u8,
							123u8, 65u8, 91u8, 229u8, 75u8, 53u8, 144u8, 105u8, 1u8, 167u8, 232u8,
							156u8,
						],
					)
				}
				#[doc = "See [`Pallet::reject_proposal`]."]
				pub fn reject_proposal(
					&self,
					proposal_id: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::RejectProposal> {
					::subxt::tx::Payload::new_static(
						"Treasury",
						"reject_proposal",
						types::RejectProposal { proposal_id },
						[
							18u8, 166u8, 80u8, 141u8, 222u8, 230u8, 4u8, 36u8, 7u8, 76u8, 12u8,
							40u8, 145u8, 114u8, 12u8, 43u8, 223u8, 78u8, 189u8, 222u8, 120u8, 80u8,
							225u8, 215u8, 119u8, 68u8, 200u8, 15u8, 25u8, 172u8, 192u8, 173u8,
						],
					)
				}
				#[doc = "See [`Pallet::approve_proposal`]."]
				pub fn approve_proposal(
					&self,
					proposal_id: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::ApproveProposal> {
					::subxt::tx::Payload::new_static(
						"Treasury",
						"approve_proposal",
						types::ApproveProposal { proposal_id },
						[
							154u8, 176u8, 152u8, 97u8, 167u8, 177u8, 78u8, 9u8, 235u8, 229u8,
							199u8, 193u8, 214u8, 3u8, 16u8, 30u8, 4u8, 104u8, 27u8, 184u8, 100u8,
							65u8, 179u8, 13u8, 91u8, 62u8, 115u8, 5u8, 219u8, 211u8, 251u8, 153u8,
						],
					)
				}
				#[doc = "See [`Pallet::spend`]."]
				pub fn spend(
					&self,
					amount: ::core::primitive::u128,
					beneficiary: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::Spend> {
					::subxt::tx::Payload::new_static(
						"Treasury",
						"spend",
						types::Spend {
							amount,
							beneficiary,
						},
						[
							84u8, 118u8, 232u8, 82u8, 17u8, 184u8, 100u8, 78u8, 38u8, 158u8, 78u8,
							200u8, 60u8, 9u8, 47u8, 72u8, 52u8, 70u8, 208u8, 208u8, 169u8, 217u8,
							191u8, 76u8, 215u8, 202u8, 72u8, 95u8, 221u8, 78u8, 106u8, 71u8,
						],
					)
				}
				#[doc = "See [`Pallet::remove_approval`]."]
				pub fn remove_approval(
					&self,
					proposal_id: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::RemoveApproval> {
					::subxt::tx::Payload::new_static(
						"Treasury",
						"remove_approval",
						types::RemoveApproval { proposal_id },
						[
							180u8, 20u8, 39u8, 227u8, 29u8, 228u8, 234u8, 36u8, 155u8, 114u8,
							197u8, 135u8, 185u8, 31u8, 56u8, 247u8, 224u8, 168u8, 254u8, 233u8,
							250u8, 134u8, 186u8, 155u8, 108u8, 84u8, 94u8, 226u8, 207u8, 130u8,
							196u8, 100u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_treasury::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "New proposal."]
			pub struct Proposed {
				pub proposal_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Proposed {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Proposed";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "We have ended a spend period and will now allocate funds."]
			pub struct Spending {
				pub budget_remaining: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Spending {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Spending";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some funds have been allocated."]
			pub struct Awarded {
				pub proposal_index: ::core::primitive::u32,
				pub award: ::core::primitive::u128,
				pub account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Awarded {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Awarded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A proposal was rejected; funds were slashed."]
			pub struct Rejected {
				pub proposal_index: ::core::primitive::u32,
				pub slashed: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Rejected {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Rejected";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some of our funds have been burnt."]
			pub struct Burnt {
				pub burnt_funds: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Burnt {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Burnt";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Spending has finished; this is the amount that rolls over until next spend."]
			pub struct Rollover {
				pub rollover_balance: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Rollover {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Rollover";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some funds have been deposited."]
			pub struct Deposit {
				pub value: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Deposit {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Deposit";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A new spend proposal has been approved."]
			pub struct SpendApproved {
				pub proposal_index: ::core::primitive::u32,
				pub amount: ::core::primitive::u128,
				pub beneficiary: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for SpendApproved {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "SpendApproved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The inactive funds of the pallet have been updated."]
			pub struct UpdatedInactive {
				pub reactivated: ::core::primitive::u128,
				pub deactivated: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for UpdatedInactive {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "UpdatedInactive";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Number of proposals that have been made."]
				pub fn proposal_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Treasury",
						"ProposalCount",
						vec![],
						[
							91u8, 238u8, 246u8, 106u8, 95u8, 66u8, 83u8, 134u8, 1u8, 225u8, 164u8,
							216u8, 113u8, 101u8, 203u8, 200u8, 113u8, 97u8, 246u8, 228u8, 140u8,
							29u8, 29u8, 48u8, 176u8, 137u8, 93u8, 230u8, 56u8, 75u8, 51u8, 149u8,
						],
					)
				}
				#[doc = " Proposals that have been made."]
				pub fn proposals(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_treasury::Proposal<
						::subxt::utils::AccountId32,
						::core::primitive::u128,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Treasury",
						"Proposals",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							182u8, 12u8, 98u8, 64u8, 117u8, 17u8, 90u8, 245u8, 80u8, 99u8, 161u8,
							17u8, 59u8, 80u8, 64u8, 139u8, 89u8, 179u8, 254u8, 239u8, 143u8, 114u8,
							77u8, 79u8, 75u8, 126u8, 52u8, 227u8, 1u8, 138u8, 35u8, 62u8,
						],
					)
				}
				#[doc = " Proposals that have been made."]
				pub fn proposals_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_treasury::Proposal<
						::subxt::utils::AccountId32,
						::core::primitive::u128,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Treasury",
						"Proposals",
						Vec::new(),
						[
							182u8, 12u8, 98u8, 64u8, 117u8, 17u8, 90u8, 245u8, 80u8, 99u8, 161u8,
							17u8, 59u8, 80u8, 64u8, 139u8, 89u8, 179u8, 254u8, 239u8, 143u8, 114u8,
							77u8, 79u8, 75u8, 126u8, 52u8, 227u8, 1u8, 138u8, 35u8, 62u8,
						],
					)
				}
				#[doc = " The amount which has been reported as inactive to Currency."]
				pub fn deactivated(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Treasury",
						"Deactivated",
						vec![],
						[
							120u8, 221u8, 159u8, 56u8, 161u8, 44u8, 54u8, 233u8, 47u8, 114u8,
							170u8, 150u8, 52u8, 24u8, 137u8, 212u8, 122u8, 247u8, 40u8, 17u8,
							208u8, 130u8, 42u8, 154u8, 33u8, 222u8, 59u8, 116u8, 0u8, 15u8, 79u8,
							123u8,
						],
					)
				}
				#[doc = " Proposal indices that have been approved but not yet awarded."]
				pub fn approvals(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u32,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Treasury",
						"Approvals",
						vec![],
						[
							78u8, 147u8, 186u8, 235u8, 17u8, 40u8, 247u8, 235u8, 67u8, 222u8, 3u8,
							14u8, 248u8, 17u8, 67u8, 180u8, 93u8, 161u8, 64u8, 35u8, 119u8, 194u8,
							187u8, 226u8, 135u8, 162u8, 147u8, 174u8, 139u8, 72u8, 99u8, 212u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Fraction of a proposal's value that should be bonded in order to place the proposal."]
				#[doc = " An accepted proposal gets these back. A rejected proposal does not."]
				pub fn proposal_bond(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_arithmetic::per_things::Permill>
				{
					::subxt::constants::Address::new_static(
						"Treasury",
						"ProposalBond",
						[
							65u8, 93u8, 120u8, 165u8, 204u8, 81u8, 159u8, 163u8, 93u8, 135u8,
							114u8, 121u8, 147u8, 35u8, 215u8, 213u8, 4u8, 223u8, 83u8, 37u8, 225u8,
							200u8, 189u8, 156u8, 140u8, 36u8, 58u8, 46u8, 42u8, 232u8, 155u8, 0u8,
						],
					)
				}
				#[doc = " Minimum amount of funds that should be placed in a deposit for making a proposal."]
				pub fn proposal_bond_minimum(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Treasury",
						"ProposalBondMinimum",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " Maximum amount of funds that should be placed in a deposit for making a proposal."]
				pub fn proposal_bond_maximum(
					&self,
				) -> ::subxt::constants::Address<::core::option::Option<::core::primitive::u128>> {
					::subxt::constants::Address::new_static(
						"Treasury",
						"ProposalBondMaximum",
						[
							198u8, 51u8, 89u8, 159u8, 124u8, 251u8, 51u8, 80u8, 167u8, 193u8, 44u8,
							199u8, 80u8, 36u8, 41u8, 130u8, 137u8, 229u8, 178u8, 208u8, 37u8,
							215u8, 169u8, 183u8, 180u8, 191u8, 140u8, 240u8, 250u8, 61u8, 42u8,
							147u8,
						],
					)
				}
				#[doc = " Period between successive spends."]
				pub fn spend_period(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Treasury",
						"SpendPeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Percentage of spare funds (if any) that are burnt per spend period."]
				pub fn burn(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_arithmetic::per_things::Permill>
				{
					::subxt::constants::Address::new_static(
						"Treasury",
						"Burn",
						[
							65u8, 93u8, 120u8, 165u8, 204u8, 81u8, 159u8, 163u8, 93u8, 135u8,
							114u8, 121u8, 147u8, 35u8, 215u8, 213u8, 4u8, 223u8, 83u8, 37u8, 225u8,
							200u8, 189u8, 156u8, 140u8, 36u8, 58u8, 46u8, 42u8, 232u8, 155u8, 0u8,
						],
					)
				}
				#[doc = " The treasury's pallet id, used for deriving its sovereign account ID."]
				pub fn pallet_id(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_support::PalletId> {
					::subxt::constants::Address::new_static(
						"Treasury",
						"PalletId",
						[
							56u8, 243u8, 53u8, 83u8, 154u8, 179u8, 170u8, 80u8, 133u8, 173u8, 61u8,
							161u8, 47u8, 225u8, 146u8, 21u8, 50u8, 229u8, 248u8, 27u8, 104u8, 58u8,
							129u8, 197u8, 102u8, 160u8, 168u8, 205u8, 154u8, 42u8, 217u8, 53u8,
						],
					)
				}
				#[doc = " The maximum number of approvals that can wait in the spending queue."]
				#[doc = ""]
				#[doc = " NOTE: This parameter is also used within the Bounties Pallet extension if enabled."]
				pub fn max_approvals(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Treasury",
						"MaxApprovals",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod sudo {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Error for the Sudo pallet"]
		pub type Error = runtime_types::pallet_sudo::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_sudo::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Sudo {
					pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Sudo {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "sudo";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SudoUncheckedWeight {
					pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					pub weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for SudoUncheckedWeight {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "sudo_unchecked_weight";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetKey {
					pub new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetKey {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "set_key";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SudoAs {
					pub who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SudoAs {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "sudo_as";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::sudo`]."]
				pub fn sudo(
					&self,
					call: runtime_types::da_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::Sudo> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"sudo",
						types::Sudo {
							call: ::std::boxed::Box::new(call),
						},
						[
							132u8, 69u8, 16u8, 63u8, 28u8, 117u8, 158u8, 165u8, 163u8, 82u8, 42u8,
							99u8, 115u8, 140u8, 111u8, 108u8, 190u8, 182u8, 55u8, 15u8, 173u8,
							220u8, 191u8, 110u8, 227u8, 201u8, 4u8, 82u8, 165u8, 177u8, 11u8,
							255u8,
						],
					)
				}
				#[doc = "See [`Pallet::sudo_unchecked_weight`]."]
				pub fn sudo_unchecked_weight(
					&self,
					call: runtime_types::da_runtime::RuntimeCall,
					weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::SudoUncheckedWeight> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"sudo_unchecked_weight",
						types::SudoUncheckedWeight {
							call: ::std::boxed::Box::new(call),
							weight,
						},
						[
							242u8, 34u8, 43u8, 180u8, 59u8, 18u8, 23u8, 108u8, 145u8, 91u8, 187u8,
							102u8, 250u8, 90u8, 180u8, 14u8, 15u8, 52u8, 173u8, 12u8, 49u8, 254u8,
							51u8, 211u8, 253u8, 162u8, 235u8, 123u8, 167u8, 48u8, 162u8, 98u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_key`]."]
				pub fn set_key(
					&self,
					new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::SetKey> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"set_key",
						types::SetKey { new },
						[
							46u8, 208u8, 84u8, 223u8, 141u8, 188u8, 184u8, 156u8, 101u8, 97u8,
							255u8, 166u8, 168u8, 102u8, 75u8, 3u8, 149u8, 105u8, 202u8, 220u8,
							187u8, 117u8, 156u8, 83u8, 159u8, 115u8, 231u8, 201u8, 171u8, 47u8,
							170u8, 36u8,
						],
					)
				}
				#[doc = "See [`Pallet::sudo_as`]."]
				pub fn sudo_as(
					&self,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					call: runtime_types::da_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::SudoAs> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"sudo_as",
						types::SudoAs {
							who,
							call: ::std::boxed::Box::new(call),
						},
						[
							198u8, 27u8, 114u8, 245u8, 101u8, 109u8, 97u8, 222u8, 9u8, 252u8,
							236u8, 254u8, 118u8, 42u8, 216u8, 165u8, 101u8, 14u8, 130u8, 97u8,
							38u8, 67u8, 226u8, 138u8, 8u8, 123u8, 120u8, 215u8, 10u8, 244u8, 187u8,
							195u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_sudo::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A sudo just took place. \\[result\\]"]
			pub struct Sudid {
				pub sudo_result:
					::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Sudid {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "Sudid";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
			pub struct KeyChanged {
				pub old_sudoer: ::core::option::Option<::subxt::utils::AccountId32>,
			}
			impl ::subxt::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "KeyChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A sudo just took place. \\[result\\]"]
			pub struct SudoAsDone {
				pub sudo_result:
					::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for SudoAsDone {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "SudoAsDone";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The `AccountId` of the sudo key."]
				pub fn key(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Sudo",
						"Key",
						vec![],
						[
							72u8, 14u8, 225u8, 162u8, 205u8, 247u8, 227u8, 105u8, 116u8, 57u8, 4u8,
							31u8, 84u8, 137u8, 227u8, 228u8, 133u8, 245u8, 206u8, 227u8, 117u8,
							36u8, 252u8, 151u8, 107u8, 15u8, 180u8, 4u8, 4u8, 152u8, 195u8, 144u8,
						],
					)
				}
			}
		}
	}
	pub mod im_online {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_im_online::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_im_online::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Heartbeat {
					pub heartbeat:
						runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
					pub signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
				}
				impl ::subxt::blocks::StaticExtrinsic for Heartbeat {
					const PALLET: &'static str = "ImOnline";
					const CALL: &'static str = "heartbeat";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::heartbeat`]."]
				pub fn heartbeat(
					&self,
					heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
					signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
				) -> ::subxt::tx::Payload<types::Heartbeat> {
					::subxt::tx::Payload::new_static(
						"ImOnline",
						"heartbeat",
						types::Heartbeat {
							heartbeat,
							signature,
						},
						[
							145u8, 227u8, 53u8, 178u8, 195u8, 173u8, 7u8, 209u8, 148u8, 82u8,
							125u8, 236u8, 128u8, 10u8, 134u8, 114u8, 95u8, 104u8, 111u8, 202u8,
							59u8, 192u8, 178u8, 182u8, 102u8, 86u8, 88u8, 50u8, 92u8, 66u8, 144u8,
							131u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_im_online::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A new heartbeat was received from `AuthorityId`."]
			pub struct HeartbeatReceived {
				pub authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
			}
			impl ::subxt::events::StaticEvent for HeartbeatReceived {
				const PALLET: &'static str = "ImOnline";
				const EVENT: &'static str = "HeartbeatReceived";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "At the end of the session, no offence was committed."]
			pub struct AllGood;
			impl ::subxt::events::StaticEvent for AllGood {
				const PALLET: &'static str = "ImOnline";
				const EVENT: &'static str = "AllGood";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "At the end of the session, at least one validator was found to be offline."]
			pub struct SomeOffline {
				pub offline: ::std::vec::Vec<(
					::subxt::utils::AccountId32,
					runtime_types::pallet_staking::Exposure<
						::subxt::utils::AccountId32,
						::core::primitive::u128,
					>,
				)>,
			}
			impl ::subxt::events::StaticEvent for SomeOffline {
				const PALLET: &'static str = "ImOnline";
				const EVENT: &'static str = "SomeOffline";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The block number after which it's ok to send heartbeats in the current"]
				#[doc = " session."]
				#[doc = ""]
				#[doc = " At the beginning of each session we set this to a value that should fall"]
				#[doc = " roughly in the middle of the session duration. The idea is to first wait for"]
				#[doc = " the validators to produce a block in the current session, so that the"]
				#[doc = " heartbeat later on will not be necessary."]
				#[doc = ""]
				#[doc = " This value will only be used as a fallback if we fail to get a proper session"]
				#[doc = " progress estimate from `NextSessionRotation`, as those estimates should be"]
				#[doc = " more accurate then the value we calculate for `HeartbeatAfter`."]
				pub fn heartbeat_after(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ImOnline",
						"HeartbeatAfter",
						vec![],
						[
							36u8, 179u8, 76u8, 254u8, 3u8, 184u8, 154u8, 142u8, 70u8, 104u8, 44u8,
							244u8, 39u8, 97u8, 31u8, 31u8, 93u8, 228u8, 185u8, 224u8, 13u8, 160u8,
							231u8, 210u8, 110u8, 143u8, 116u8, 29u8, 0u8, 215u8, 217u8, 137u8,
						],
					)
				}
				#[doc = " The current set of keys that may issue a heartbeat."]
				pub fn keys(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
						runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ImOnline",
						"Keys",
						vec![],
						[
							111u8, 104u8, 188u8, 46u8, 152u8, 140u8, 137u8, 244u8, 52u8, 214u8,
							115u8, 156u8, 39u8, 239u8, 15u8, 168u8, 193u8, 125u8, 57u8, 195u8,
							250u8, 156u8, 234u8, 222u8, 222u8, 253u8, 135u8, 232u8, 196u8, 163u8,
							29u8, 218u8,
						],
					)
				}
				#[doc = " For each session index, we keep a mapping of `SessionIndex` and `AuthIndex`."]
				pub fn received_heartbeats(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"ImOnline",
						"ReceivedHeartbeats",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							123u8, 182u8, 145u8, 49u8, 90u8, 110u8, 80u8, 53u8, 62u8, 45u8, 173u8,
							252u8, 126u8, 163u8, 229u8, 173u8, 54u8, 169u8, 61u8, 128u8, 10u8,
							33u8, 254u8, 78u8, 145u8, 134u8, 235u8, 26u8, 177u8, 55u8, 7u8, 75u8,
						],
					)
				}
				#[doc = " For each session index, we keep a mapping of `SessionIndex` and `AuthIndex`."]
				pub fn received_heartbeats_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"ImOnline",
						"ReceivedHeartbeats",
						Vec::new(),
						[
							123u8, 182u8, 145u8, 49u8, 90u8, 110u8, 80u8, 53u8, 62u8, 45u8, 173u8,
							252u8, 126u8, 163u8, 229u8, 173u8, 54u8, 169u8, 61u8, 128u8, 10u8,
							33u8, 254u8, 78u8, 145u8, 134u8, 235u8, 26u8, 177u8, 55u8, 7u8, 75u8,
						],
					)
				}
				#[doc = " For each session index, we keep a mapping of `ValidatorId<T>` to the"]
				#[doc = " number of blocks authored by the given authority."]
				pub fn authored_blocks(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"ImOnline",
						"AuthoredBlocks",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							121u8, 246u8, 100u8, 191u8, 5u8, 211u8, 190u8, 244u8, 61u8, 73u8,
							169u8, 127u8, 116u8, 80u8, 118u8, 139u8, 115u8, 58u8, 125u8, 81u8,
							75u8, 20u8, 194u8, 74u8, 97u8, 188u8, 55u8, 160u8, 33u8, 155u8, 186u8,
							74u8,
						],
					)
				}
				#[doc = " For each session index, we keep a mapping of `ValidatorId<T>` to the"]
				#[doc = " number of blocks authored by the given authority."]
				pub fn authored_blocks_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"ImOnline",
						"AuthoredBlocks",
						Vec::new(),
						[
							121u8, 246u8, 100u8, 191u8, 5u8, 211u8, 190u8, 244u8, 61u8, 73u8,
							169u8, 127u8, 116u8, 80u8, 118u8, 139u8, 115u8, 58u8, 125u8, 81u8,
							75u8, 20u8, 194u8, 74u8, 97u8, 188u8, 55u8, 160u8, 33u8, 155u8, 186u8,
							74u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " A configuration for base priority of unsigned transactions."]
				#[doc = ""]
				#[doc = " This is exposed so that it can be tuned for particular runtime, when"]
				#[doc = " multiple pallets send unsigned transactions."]
				pub fn unsigned_priority(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"ImOnline",
						"UnsignedPriority",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
			}
		}
	}
	pub mod authority_discovery {
		use super::root_mod;
		use super::runtime_types;
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Keys of the current authority set."]
				pub fn keys(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
						runtime_types::sp_authority_discovery::app::Public,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"AuthorityDiscovery",
						"Keys",
						vec![],
						[
							111u8, 104u8, 188u8, 46u8, 152u8, 140u8, 137u8, 244u8, 52u8, 214u8,
							115u8, 156u8, 39u8, 239u8, 15u8, 168u8, 193u8, 125u8, 57u8, 195u8,
							250u8, 156u8, 234u8, 222u8, 222u8, 253u8, 135u8, 232u8, 196u8, 163u8,
							29u8, 218u8,
						],
					)
				}
				#[doc = " Keys of the next authority set."]
				pub fn next_keys(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
						runtime_types::sp_authority_discovery::app::Public,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"AuthorityDiscovery",
						"NextKeys",
						vec![],
						[
							171u8, 107u8, 15u8, 108u8, 125u8, 102u8, 193u8, 240u8, 127u8, 160u8,
							53u8, 1u8, 208u8, 36u8, 134u8, 4u8, 216u8, 26u8, 156u8, 143u8, 154u8,
							194u8, 153u8, 199u8, 46u8, 211u8, 153u8, 222u8, 244u8, 4u8, 165u8, 2u8,
						],
					)
				}
			}
		}
	}
	pub mod offences {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Events type."]
		pub type Event = runtime_types::pallet_offences::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "There is an offence reported of the given `kind` happened at the `session_index` and"]
			#[doc = "(kind-specific) time slot. This event is not deposited for duplicate slashes."]
			#[doc = "\\[kind, timeslot\\]."]
			pub struct Offence {
				pub kind: [::core::primitive::u8; 16usize],
				pub timeslot: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for Offence {
				const PALLET: &'static str = "Offences";
				const EVENT: &'static str = "Offence";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The primary structure that holds all offence records keyed by report identifiers."]
				pub fn reports(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_staking::offence::OffenceDetails<
						::subxt::utils::AccountId32,
						(
							::subxt::utils::AccountId32,
							runtime_types::pallet_staking::Exposure<
								::subxt::utils::AccountId32,
								::core::primitive::u128,
							>,
						),
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Offences",
						"Reports",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							242u8, 69u8, 20u8, 130u8, 250u8, 223u8, 68u8, 121u8, 187u8, 215u8,
							62u8, 204u8, 100u8, 51u8, 76u8, 164u8, 188u8, 182u8, 215u8, 93u8,
							161u8, 100u8, 187u8, 205u8, 73u8, 158u8, 57u8, 198u8, 239u8, 66u8,
							42u8, 65u8,
						],
					)
				}
				#[doc = " The primary structure that holds all offence records keyed by report identifiers."]
				pub fn reports_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_staking::offence::OffenceDetails<
						::subxt::utils::AccountId32,
						(
							::subxt::utils::AccountId32,
							runtime_types::pallet_staking::Exposure<
								::subxt::utils::AccountId32,
								::core::primitive::u128,
							>,
						),
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Offences",
						"Reports",
						Vec::new(),
						[
							242u8, 69u8, 20u8, 130u8, 250u8, 223u8, 68u8, 121u8, 187u8, 215u8,
							62u8, 204u8, 100u8, 51u8, 76u8, 164u8, 188u8, 182u8, 215u8, 93u8,
							161u8, 100u8, 187u8, 205u8, 73u8, 158u8, 57u8, 198u8, 239u8, 66u8,
							42u8, 65u8,
						],
					)
				}
				#[doc = " A vector of reports of the same kind that happened at the same time slot."]
				pub fn concurrent_reports_index(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8; 16usize]>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::subxt::utils::H256>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Offences",
						"ConcurrentReportsIndex",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							125u8, 222u8, 9u8, 162u8, 38u8, 89u8, 77u8, 187u8, 129u8, 103u8, 21u8,
							31u8, 117u8, 101u8, 43u8, 115u8, 170u8, 205u8, 142u8, 26u8, 27u8,
							184u8, 152u8, 133u8, 76u8, 203u8, 78u8, 113u8, 51u8, 141u8, 118u8,
							171u8,
						],
					)
				}
				#[doc = " A vector of reports of the same kind that happened at the same time slot."]
				pub fn concurrent_reports_index_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::subxt::utils::H256>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Offences",
						"ConcurrentReportsIndex",
						Vec::new(),
						[
							125u8, 222u8, 9u8, 162u8, 38u8, 89u8, 77u8, 187u8, 129u8, 103u8, 21u8,
							31u8, 117u8, 101u8, 43u8, 115u8, 170u8, 205u8, 142u8, 26u8, 27u8,
							184u8, 152u8, 133u8, 76u8, 203u8, 78u8, 113u8, 51u8, 141u8, 118u8,
							171u8,
						],
					)
				}
			}
		}
	}
	pub mod historical {
		use super::root_mod;
		use super::runtime_types;
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Mapping from historical session indices to session-data root hash and validator count."]
				pub fn historical_sessions(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::subxt::utils::H256, ::core::primitive::u32),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Historical",
						"HistoricalSessions",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							219u8, 105u8, 245u8, 54u8, 143u8, 81u8, 56u8, 104u8, 60u8, 207u8,
							165u8, 92u8, 123u8, 2u8, 64u8, 117u8, 154u8, 135u8, 252u8, 234u8,
							129u8, 159u8, 77u8, 38u8, 238u8, 220u8, 9u8, 88u8, 70u8, 200u8, 132u8,
							152u8,
						],
					)
				}
				#[doc = " Mapping from historical session indices to session-data root hash and validator count."]
				pub fn historical_sessions_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::subxt::utils::H256, ::core::primitive::u32),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Historical",
						"HistoricalSessions",
						Vec::new(),
						[
							219u8, 105u8, 245u8, 54u8, 143u8, 81u8, 56u8, 104u8, 60u8, 207u8,
							165u8, 92u8, 123u8, 2u8, 64u8, 117u8, 154u8, 135u8, 252u8, 234u8,
							129u8, 159u8, 77u8, 38u8, 238u8, 220u8, 9u8, 88u8, 70u8, 200u8, 132u8,
							152u8,
						],
					)
				}
				#[doc = " The range of historical sessions we store. [first, last)"]
				pub fn stored_range(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::core::primitive::u32, ::core::primitive::u32),
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Historical",
						"StoredRange",
						vec![],
						[
							165u8, 141u8, 148u8, 255u8, 90u8, 168u8, 77u8, 150u8, 198u8, 127u8,
							227u8, 146u8, 107u8, 141u8, 43u8, 50u8, 190u8, 186u8, 45u8, 232u8,
							46u8, 95u8, 64u8, 155u8, 57u8, 82u8, 45u8, 255u8, 91u8, 109u8, 244u8,
							216u8,
						],
					)
				}
			}
		}
	}
	pub mod scheduler {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_scheduler::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_scheduler::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Schedule {
					pub when: ::core::primitive::u32,
					pub maybe_periodic:
						::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
					pub priority: ::core::primitive::u8,
					pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Schedule {
					const PALLET: &'static str = "Scheduler";
					const CALL: &'static str = "schedule";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Cancel {
					pub when: ::core::primitive::u32,
					pub index: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Cancel {
					const PALLET: &'static str = "Scheduler";
					const CALL: &'static str = "cancel";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ScheduleNamed {
					pub id: [::core::primitive::u8; 32usize],
					pub when: ::core::primitive::u32,
					pub maybe_periodic:
						::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
					pub priority: ::core::primitive::u8,
					pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ScheduleNamed {
					const PALLET: &'static str = "Scheduler";
					const CALL: &'static str = "schedule_named";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CancelNamed {
					pub id: [::core::primitive::u8; 32usize],
				}
				impl ::subxt::blocks::StaticExtrinsic for CancelNamed {
					const PALLET: &'static str = "Scheduler";
					const CALL: &'static str = "cancel_named";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ScheduleAfter {
					pub after: ::core::primitive::u32,
					pub maybe_periodic:
						::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
					pub priority: ::core::primitive::u8,
					pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ScheduleAfter {
					const PALLET: &'static str = "Scheduler";
					const CALL: &'static str = "schedule_after";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ScheduleNamedAfter {
					pub id: [::core::primitive::u8; 32usize],
					pub after: ::core::primitive::u32,
					pub maybe_periodic:
						::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
					pub priority: ::core::primitive::u8,
					pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ScheduleNamedAfter {
					const PALLET: &'static str = "Scheduler";
					const CALL: &'static str = "schedule_named_after";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::schedule`]."]
				pub fn schedule(
					&self,
					when: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::da_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::Schedule> {
					::subxt::tx::Payload::new_static(
						"Scheduler",
						"schedule",
						types::Schedule {
							when,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						},
						[
							18u8, 49u8, 145u8, 211u8, 200u8, 186u8, 238u8, 185u8, 154u8, 111u8,
							96u8, 56u8, 156u8, 38u8, 252u8, 152u8, 123u8, 108u8, 176u8, 175u8,
							28u8, 38u8, 203u8, 218u8, 141u8, 161u8, 244u8, 115u8, 93u8, 154u8,
							245u8, 112u8,
						],
					)
				}
				#[doc = "See [`Pallet::cancel`]."]
				pub fn cancel(
					&self,
					when: ::core::primitive::u32,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Cancel> {
					::subxt::tx::Payload::new_static(
						"Scheduler",
						"cancel",
						types::Cancel { when, index },
						[
							32u8, 107u8, 14u8, 102u8, 56u8, 200u8, 68u8, 186u8, 192u8, 100u8,
							152u8, 124u8, 171u8, 154u8, 230u8, 115u8, 62u8, 140u8, 88u8, 178u8,
							119u8, 210u8, 222u8, 31u8, 134u8, 225u8, 133u8, 241u8, 42u8, 110u8,
							147u8, 47u8,
						],
					)
				}
				#[doc = "See [`Pallet::schedule_named`]."]
				pub fn schedule_named(
					&self,
					id: [::core::primitive::u8; 32usize],
					when: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::da_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::ScheduleNamed> {
					::subxt::tx::Payload::new_static(
						"Scheduler",
						"schedule_named",
						types::ScheduleNamed {
							id,
							when,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						},
						[
							121u8, 7u8, 233u8, 108u8, 196u8, 170u8, 148u8, 225u8, 27u8, 116u8,
							51u8, 86u8, 99u8, 63u8, 244u8, 179u8, 125u8, 218u8, 210u8, 156u8,
							245u8, 75u8, 207u8, 191u8, 210u8, 150u8, 23u8, 111u8, 14u8, 4u8, 156u8,
							151u8,
						],
					)
				}
				#[doc = "See [`Pallet::cancel_named`]."]
				pub fn cancel_named(
					&self,
					id: [::core::primitive::u8; 32usize],
				) -> ::subxt::tx::Payload<types::CancelNamed> {
					::subxt::tx::Payload::new_static(
						"Scheduler",
						"cancel_named",
						types::CancelNamed { id },
						[
							205u8, 35u8, 28u8, 57u8, 224u8, 7u8, 49u8, 233u8, 236u8, 163u8, 93u8,
							236u8, 103u8, 69u8, 65u8, 51u8, 121u8, 84u8, 9u8, 196u8, 147u8, 122u8,
							227u8, 200u8, 181u8, 233u8, 62u8, 240u8, 174u8, 83u8, 129u8, 193u8,
						],
					)
				}
				#[doc = "See [`Pallet::schedule_after`]."]
				pub fn schedule_after(
					&self,
					after: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::da_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::ScheduleAfter> {
					::subxt::tx::Payload::new_static(
						"Scheduler",
						"schedule_after",
						types::ScheduleAfter {
							after,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						},
						[
							161u8, 122u8, 162u8, 47u8, 124u8, 5u8, 98u8, 177u8, 59u8, 69u8, 98u8,
							168u8, 196u8, 221u8, 210u8, 53u8, 39u8, 31u8, 228u8, 107u8, 14u8, 27u8,
							178u8, 16u8, 193u8, 8u8, 24u8, 229u8, 24u8, 168u8, 242u8, 30u8,
						],
					)
				}
				#[doc = "See [`Pallet::schedule_named_after`]."]
				pub fn schedule_named_after(
					&self,
					id: [::core::primitive::u8; 32usize],
					after: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::da_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::ScheduleNamedAfter> {
					::subxt::tx::Payload::new_static(
						"Scheduler",
						"schedule_named_after",
						types::ScheduleNamedAfter {
							id,
							after,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						},
						[
							80u8, 204u8, 70u8, 103u8, 72u8, 3u8, 206u8, 49u8, 76u8, 36u8, 84u8,
							142u8, 173u8, 109u8, 102u8, 65u8, 90u8, 13u8, 209u8, 126u8, 178u8,
							15u8, 224u8, 223u8, 41u8, 39u8, 240u8, 120u8, 187u8, 44u8, 50u8, 73u8,
						],
					)
				}
			}
		}
		#[doc = "Events type."]
		pub type Event = runtime_types::pallet_scheduler::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Scheduled some task."]
			pub struct Scheduled {
				pub when: ::core::primitive::u32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Scheduled {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "Scheduled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Canceled some task."]
			pub struct Canceled {
				pub when: ::core::primitive::u32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Canceled {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "Canceled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Dispatched some task."]
			pub struct Dispatched {
				pub task: (::core::primitive::u32, ::core::primitive::u32),
				pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Dispatched {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "Dispatched";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The call for the provided hash was not found so the task has been aborted."]
			pub struct CallUnavailable {
				pub task: (::core::primitive::u32, ::core::primitive::u32),
				pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for CallUnavailable {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "CallUnavailable";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The given task was unable to be renewed since the agenda is full at that block."]
			pub struct PeriodicFailed {
				pub task: (::core::primitive::u32, ::core::primitive::u32),
				pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for PeriodicFailed {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "PeriodicFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The given task can never be executed since it is overweight."]
			pub struct PermanentlyOverweight {
				pub task: (::core::primitive::u32, ::core::primitive::u32),
				pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for PermanentlyOverweight {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "PermanentlyOverweight";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn incomplete_since(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Scheduler",
						"IncompleteSince",
						vec![],
						[
							250u8, 83u8, 64u8, 167u8, 205u8, 59u8, 225u8, 97u8, 205u8, 12u8, 76u8,
							130u8, 197u8, 4u8, 111u8, 208u8, 92u8, 217u8, 145u8, 119u8, 38u8,
							135u8, 1u8, 242u8, 228u8, 143u8, 56u8, 25u8, 115u8, 233u8, 227u8, 66u8,
						],
					)
				}
				#[doc = " Items to be executed, indexed by the block number that they should be executed on."]
				pub fn agenda(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::option::Option<
							runtime_types::pallet_scheduler::Scheduled<
								[::core::primitive::u8; 32usize],
								runtime_types::frame_support::traits::preimages::Bounded<
									runtime_types::da_runtime::RuntimeCall,
								>,
								::core::primitive::u32,
								runtime_types::da_runtime::OriginCaller,
								::subxt::utils::AccountId32,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Scheduler",
						"Agenda",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							23u8, 81u8, 39u8, 5u8, 225u8, 208u8, 146u8, 192u8, 11u8, 82u8, 106u8,
							66u8, 228u8, 103u8, 246u8, 7u8, 227u8, 101u8, 213u8, 112u8, 244u8,
							69u8, 24u8, 166u8, 1u8, 164u8, 247u8, 131u8, 228u8, 224u8, 13u8, 215u8,
						],
					)
				}
				#[doc = " Items to be executed, indexed by the block number that they should be executed on."]
				pub fn agenda_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::option::Option<
							runtime_types::pallet_scheduler::Scheduled<
								[::core::primitive::u8; 32usize],
								runtime_types::frame_support::traits::preimages::Bounded<
									runtime_types::da_runtime::RuntimeCall,
								>,
								::core::primitive::u32,
								runtime_types::da_runtime::OriginCaller,
								::subxt::utils::AccountId32,
							>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Scheduler",
						"Agenda",
						Vec::new(),
						[
							23u8, 81u8, 39u8, 5u8, 225u8, 208u8, 146u8, 192u8, 11u8, 82u8, 106u8,
							66u8, 228u8, 103u8, 246u8, 7u8, 227u8, 101u8, 213u8, 112u8, 244u8,
							69u8, 24u8, 166u8, 1u8, 164u8, 247u8, 131u8, 228u8, 224u8, 13u8, 215u8,
						],
					)
				}
				#[doc = " Lookup from a name to the block number and index of the task."]
				#[doc = ""]
				#[doc = " For v3 -> v4 the previously unbounded identities are Blake2-256 hashed to form the v4"]
				#[doc = " identities."]
				pub fn lookup(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8; 32usize]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::core::primitive::u32, ::core::primitive::u32),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Scheduler",
						"Lookup",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							157u8, 102u8, 210u8, 65u8, 190u8, 48u8, 168u8, 20u8, 197u8, 184u8,
							74u8, 119u8, 176u8, 22u8, 244u8, 186u8, 231u8, 239u8, 97u8, 175u8,
							34u8, 133u8, 165u8, 73u8, 223u8, 113u8, 78u8, 150u8, 83u8, 127u8,
							126u8, 204u8,
						],
					)
				}
				#[doc = " Lookup from a name to the block number and index of the task."]
				#[doc = ""]
				#[doc = " For v3 -> v4 the previously unbounded identities are Blake2-256 hashed to form the v4"]
				#[doc = " identities."]
				pub fn lookup_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(::core::primitive::u32, ::core::primitive::u32),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Scheduler",
						"Lookup",
						Vec::new(),
						[
							157u8, 102u8, 210u8, 65u8, 190u8, 48u8, 168u8, 20u8, 197u8, 184u8,
							74u8, 119u8, 176u8, 22u8, 244u8, 186u8, 231u8, 239u8, 97u8, 175u8,
							34u8, 133u8, 165u8, 73u8, 223u8, 113u8, 78u8, 150u8, 83u8, 127u8,
							126u8, 204u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The maximum weight that may be scheduled per block for any dispatchables."]
				pub fn maximum_weight(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_weights::weight_v2::Weight> {
					::subxt::constants::Address::new_static(
						"Scheduler",
						"MaximumWeight",
						[
							222u8, 183u8, 203u8, 169u8, 31u8, 134u8, 28u8, 12u8, 47u8, 140u8, 71u8,
							74u8, 61u8, 55u8, 71u8, 236u8, 215u8, 83u8, 28u8, 70u8, 45u8, 128u8,
							184u8, 57u8, 101u8, 83u8, 42u8, 165u8, 34u8, 155u8, 64u8, 145u8,
						],
					)
				}
				#[doc = " The maximum number of scheduled calls in the queue for a single block."]
				#[doc = ""]
				#[doc = " NOTE:"]
				#[doc = " + Dependent pallets' benchmarks might require a higher limit for the setting. Set a"]
				#[doc = " higher limit under `runtime-benchmarks` feature."]
				pub fn max_scheduled_per_block(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Scheduler",
						"MaxScheduledPerBlock",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod bounties {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_bounties::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_bounties::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ProposeBounty {
					#[codec(compact)]
					pub value: ::core::primitive::u128,
					pub description: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ProposeBounty {
					const PALLET: &'static str = "Bounties";
					const CALL: &'static str = "propose_bounty";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ApproveBounty {
					#[codec(compact)]
					pub bounty_id: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for ApproveBounty {
					const PALLET: &'static str = "Bounties";
					const CALL: &'static str = "approve_bounty";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ProposeCurator {
					#[codec(compact)]
					pub bounty_id: ::core::primitive::u32,
					pub curator: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					#[codec(compact)]
					pub fee: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for ProposeCurator {
					const PALLET: &'static str = "Bounties";
					const CALL: &'static str = "propose_curator";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct UnassignCurator {
					#[codec(compact)]
					pub bounty_id: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for UnassignCurator {
					const PALLET: &'static str = "Bounties";
					const CALL: &'static str = "unassign_curator";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AcceptCurator {
					#[codec(compact)]
					pub bounty_id: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for AcceptCurator {
					const PALLET: &'static str = "Bounties";
					const CALL: &'static str = "accept_curator";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AwardBounty {
					#[codec(compact)]
					pub bounty_id: ::core::primitive::u32,
					pub beneficiary: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for AwardBounty {
					const PALLET: &'static str = "Bounties";
					const CALL: &'static str = "award_bounty";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ClaimBounty {
					#[codec(compact)]
					pub bounty_id: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for ClaimBounty {
					const PALLET: &'static str = "Bounties";
					const CALL: &'static str = "claim_bounty";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CloseBounty {
					#[codec(compact)]
					pub bounty_id: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for CloseBounty {
					const PALLET: &'static str = "Bounties";
					const CALL: &'static str = "close_bounty";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ExtendBountyExpiry {
					#[codec(compact)]
					pub bounty_id: ::core::primitive::u32,
					pub remark: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ExtendBountyExpiry {
					const PALLET: &'static str = "Bounties";
					const CALL: &'static str = "extend_bounty_expiry";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::propose_bounty`]."]
				pub fn propose_bounty(
					&self,
					value: ::core::primitive::u128,
					description: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::ProposeBounty> {
					::subxt::tx::Payload::new_static(
						"Bounties",
						"propose_bounty",
						types::ProposeBounty { value, description },
						[
							131u8, 169u8, 55u8, 102u8, 212u8, 139u8, 9u8, 65u8, 75u8, 112u8, 6u8,
							180u8, 92u8, 124u8, 43u8, 42u8, 38u8, 40u8, 226u8, 24u8, 28u8, 34u8,
							169u8, 220u8, 184u8, 206u8, 109u8, 227u8, 53u8, 228u8, 88u8, 25u8,
						],
					)
				}
				#[doc = "See [`Pallet::approve_bounty`]."]
				pub fn approve_bounty(
					&self,
					bounty_id: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::ApproveBounty> {
					::subxt::tx::Payload::new_static(
						"Bounties",
						"approve_bounty",
						types::ApproveBounty { bounty_id },
						[
							85u8, 12u8, 177u8, 91u8, 183u8, 124u8, 175u8, 148u8, 188u8, 200u8,
							237u8, 144u8, 6u8, 67u8, 159u8, 48u8, 177u8, 222u8, 183u8, 137u8,
							173u8, 131u8, 128u8, 219u8, 255u8, 243u8, 80u8, 224u8, 126u8, 136u8,
							90u8, 47u8,
						],
					)
				}
				#[doc = "See [`Pallet::propose_curator`]."]
				pub fn propose_curator(
					&self,
					bounty_id: ::core::primitive::u32,
					curator: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					fee: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::ProposeCurator> {
					::subxt::tx::Payload::new_static(
						"Bounties",
						"propose_curator",
						types::ProposeCurator {
							bounty_id,
							curator,
							fee,
						},
						[
							71u8, 194u8, 107u8, 158u8, 153u8, 106u8, 130u8, 173u8, 57u8, 240u8,
							43u8, 213u8, 143u8, 48u8, 179u8, 254u8, 63u8, 190u8, 105u8, 52u8,
							127u8, 238u8, 84u8, 105u8, 41u8, 180u8, 25u8, 98u8, 215u8, 197u8, 61u8,
							72u8,
						],
					)
				}
				#[doc = "See [`Pallet::unassign_curator`]."]
				pub fn unassign_curator(
					&self,
					bounty_id: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::UnassignCurator> {
					::subxt::tx::Payload::new_static(
						"Bounties",
						"unassign_curator",
						types::UnassignCurator { bounty_id },
						[
							98u8, 94u8, 107u8, 111u8, 151u8, 182u8, 71u8, 239u8, 214u8, 88u8,
							108u8, 11u8, 51u8, 163u8, 102u8, 162u8, 245u8, 247u8, 244u8, 159u8,
							197u8, 23u8, 171u8, 6u8, 60u8, 146u8, 144u8, 101u8, 68u8, 133u8, 245u8,
							74u8,
						],
					)
				}
				#[doc = "See [`Pallet::accept_curator`]."]
				pub fn accept_curator(
					&self,
					bounty_id: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::AcceptCurator> {
					::subxt::tx::Payload::new_static(
						"Bounties",
						"accept_curator",
						types::AcceptCurator { bounty_id },
						[
							178u8, 142u8, 138u8, 15u8, 243u8, 10u8, 222u8, 169u8, 150u8, 200u8,
							85u8, 185u8, 39u8, 167u8, 134u8, 3u8, 186u8, 84u8, 43u8, 140u8, 11u8,
							70u8, 56u8, 197u8, 39u8, 84u8, 138u8, 139u8, 198u8, 104u8, 41u8, 238u8,
						],
					)
				}
				#[doc = "See [`Pallet::award_bounty`]."]
				pub fn award_bounty(
					&self,
					bounty_id: ::core::primitive::u32,
					beneficiary: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::AwardBounty> {
					::subxt::tx::Payload::new_static(
						"Bounties",
						"award_bounty",
						types::AwardBounty {
							bounty_id,
							beneficiary,
						},
						[
							69u8, 87u8, 233u8, 113u8, 45u8, 84u8, 104u8, 158u8, 128u8, 111u8,
							231u8, 80u8, 175u8, 226u8, 231u8, 61u8, 106u8, 6u8, 192u8, 208u8,
							128u8, 160u8, 75u8, 27u8, 190u8, 9u8, 191u8, 90u8, 219u8, 25u8, 129u8,
							244u8,
						],
					)
				}
				#[doc = "See [`Pallet::claim_bounty`]."]
				pub fn claim_bounty(
					&self,
					bounty_id: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::ClaimBounty> {
					::subxt::tx::Payload::new_static(
						"Bounties",
						"claim_bounty",
						types::ClaimBounty { bounty_id },
						[
							211u8, 143u8, 123u8, 205u8, 140u8, 43u8, 176u8, 103u8, 110u8, 125u8,
							158u8, 131u8, 103u8, 62u8, 69u8, 215u8, 220u8, 110u8, 11u8, 3u8, 30u8,
							193u8, 235u8, 177u8, 96u8, 241u8, 140u8, 53u8, 62u8, 133u8, 170u8,
							25u8,
						],
					)
				}
				#[doc = "See [`Pallet::close_bounty`]."]
				pub fn close_bounty(
					&self,
					bounty_id: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::CloseBounty> {
					::subxt::tx::Payload::new_static(
						"Bounties",
						"close_bounty",
						types::CloseBounty { bounty_id },
						[
							144u8, 234u8, 109u8, 39u8, 227u8, 231u8, 104u8, 48u8, 45u8, 196u8,
							217u8, 220u8, 241u8, 197u8, 157u8, 227u8, 154u8, 156u8, 181u8, 69u8,
							146u8, 77u8, 203u8, 167u8, 79u8, 102u8, 15u8, 253u8, 135u8, 53u8, 96u8,
							60u8,
						],
					)
				}
				#[doc = "See [`Pallet::extend_bounty_expiry`]."]
				pub fn extend_bounty_expiry(
					&self,
					bounty_id: ::core::primitive::u32,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::ExtendBountyExpiry> {
					::subxt::tx::Payload::new_static(
						"Bounties",
						"extend_bounty_expiry",
						types::ExtendBountyExpiry { bounty_id, remark },
						[
							102u8, 118u8, 89u8, 189u8, 138u8, 157u8, 216u8, 10u8, 239u8, 3u8,
							200u8, 217u8, 219u8, 19u8, 195u8, 182u8, 105u8, 220u8, 11u8, 146u8,
							222u8, 79u8, 95u8, 136u8, 188u8, 230u8, 248u8, 119u8, 30u8, 6u8, 242u8,
							194u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_bounties::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "New bounty proposal."]
			pub struct BountyProposed {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for BountyProposed {
				const PALLET: &'static str = "Bounties";
				const EVENT: &'static str = "BountyProposed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A bounty proposal was rejected; funds were slashed."]
			pub struct BountyRejected {
				pub index: ::core::primitive::u32,
				pub bond: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for BountyRejected {
				const PALLET: &'static str = "Bounties";
				const EVENT: &'static str = "BountyRejected";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A bounty proposal is funded and became active."]
			pub struct BountyBecameActive {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for BountyBecameActive {
				const PALLET: &'static str = "Bounties";
				const EVENT: &'static str = "BountyBecameActive";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A bounty is awarded to a beneficiary."]
			pub struct BountyAwarded {
				pub index: ::core::primitive::u32,
				pub beneficiary: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for BountyAwarded {
				const PALLET: &'static str = "Bounties";
				const EVENT: &'static str = "BountyAwarded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A bounty is claimed by beneficiary."]
			pub struct BountyClaimed {
				pub index: ::core::primitive::u32,
				pub payout: ::core::primitive::u128,
				pub beneficiary: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for BountyClaimed {
				const PALLET: &'static str = "Bounties";
				const EVENT: &'static str = "BountyClaimed";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A bounty is cancelled."]
			pub struct BountyCanceled {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for BountyCanceled {
				const PALLET: &'static str = "Bounties";
				const EVENT: &'static str = "BountyCanceled";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A bounty expiry is extended."]
			pub struct BountyExtended {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for BountyExtended {
				const PALLET: &'static str = "Bounties";
				const EVENT: &'static str = "BountyExtended";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Number of bounty proposals that have been made."]
				pub fn bounty_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Bounties",
						"BountyCount",
						vec![],
						[
							120u8, 204u8, 26u8, 150u8, 37u8, 81u8, 43u8, 223u8, 180u8, 252u8,
							142u8, 144u8, 109u8, 5u8, 184u8, 72u8, 223u8, 230u8, 66u8, 196u8, 14u8,
							14u8, 164u8, 190u8, 246u8, 168u8, 190u8, 56u8, 212u8, 73u8, 175u8,
							26u8,
						],
					)
				}
				#[doc = " Bounties that have been made."]
				pub fn bounties(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_bounties::Bounty<
						::subxt::utils::AccountId32,
						::core::primitive::u128,
						::core::primitive::u32,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Bounties",
						"Bounties",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							197u8, 26u8, 141u8, 98u8, 53u8, 123u8, 87u8, 219u8, 248u8, 200u8,
							207u8, 196u8, 211u8, 159u8, 124u8, 173u8, 143u8, 144u8, 85u8, 180u8,
							227u8, 24u8, 7u8, 52u8, 130u8, 98u8, 107u8, 145u8, 162u8, 55u8, 64u8,
							199u8,
						],
					)
				}
				#[doc = " Bounties that have been made."]
				pub fn bounties_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_bounties::Bounty<
						::subxt::utils::AccountId32,
						::core::primitive::u128,
						::core::primitive::u32,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Bounties",
						"Bounties",
						Vec::new(),
						[
							197u8, 26u8, 141u8, 98u8, 53u8, 123u8, 87u8, 219u8, 248u8, 200u8,
							207u8, 196u8, 211u8, 159u8, 124u8, 173u8, 143u8, 144u8, 85u8, 180u8,
							227u8, 24u8, 7u8, 52u8, 130u8, 98u8, 107u8, 145u8, 162u8, 55u8, 64u8,
							199u8,
						],
					)
				}
				#[doc = " The description of each bounty."]
				pub fn bounty_descriptions(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Bounties",
						"BountyDescriptions",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							71u8, 40u8, 133u8, 84u8, 55u8, 207u8, 169u8, 189u8, 160u8, 51u8, 202u8,
							144u8, 15u8, 226u8, 97u8, 114u8, 54u8, 247u8, 53u8, 26u8, 36u8, 54u8,
							186u8, 163u8, 198u8, 100u8, 191u8, 121u8, 186u8, 160u8, 85u8, 97u8,
						],
					)
				}
				#[doc = " The description of each bounty."]
				pub fn bounty_descriptions_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Bounties",
						"BountyDescriptions",
						Vec::new(),
						[
							71u8, 40u8, 133u8, 84u8, 55u8, 207u8, 169u8, 189u8, 160u8, 51u8, 202u8,
							144u8, 15u8, 226u8, 97u8, 114u8, 54u8, 247u8, 53u8, 26u8, 36u8, 54u8,
							186u8, 163u8, 198u8, 100u8, 191u8, 121u8, 186u8, 160u8, 85u8, 97u8,
						],
					)
				}
				#[doc = " Bounty indices that have been approved but not yet funded."]
				pub fn bounty_approvals(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u32,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Bounties",
						"BountyApprovals",
						vec![],
						[
							182u8, 228u8, 0u8, 46u8, 176u8, 25u8, 222u8, 180u8, 51u8, 57u8, 14u8,
							0u8, 69u8, 160u8, 64u8, 27u8, 88u8, 29u8, 227u8, 146u8, 2u8, 121u8,
							27u8, 85u8, 45u8, 110u8, 244u8, 62u8, 134u8, 77u8, 175u8, 188u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The amount held on deposit for placing a bounty proposal."]
				pub fn bounty_deposit_base(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Bounties",
						"BountyDepositBase",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The delay period for which a bounty beneficiary need to wait before claim the payout."]
				pub fn bounty_deposit_payout_delay(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Bounties",
						"BountyDepositPayoutDelay",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Bounty duration in blocks."]
				pub fn bounty_update_period(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Bounties",
						"BountyUpdatePeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The curator deposit is calculated as a percentage of the curator fee."]
				#[doc = ""]
				#[doc = " This deposit has optional upper and lower bounds with `CuratorDepositMax` and"]
				#[doc = " `CuratorDepositMin`."]
				pub fn curator_deposit_multiplier(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_arithmetic::per_things::Permill>
				{
					::subxt::constants::Address::new_static(
						"Bounties",
						"CuratorDepositMultiplier",
						[
							65u8, 93u8, 120u8, 165u8, 204u8, 81u8, 159u8, 163u8, 93u8, 135u8,
							114u8, 121u8, 147u8, 35u8, 215u8, 213u8, 4u8, 223u8, 83u8, 37u8, 225u8,
							200u8, 189u8, 156u8, 140u8, 36u8, 58u8, 46u8, 42u8, 232u8, 155u8, 0u8,
						],
					)
				}
				#[doc = " Maximum amount of funds that should be placed in a deposit for making a proposal."]
				pub fn curator_deposit_max(
					&self,
				) -> ::subxt::constants::Address<::core::option::Option<::core::primitive::u128>> {
					::subxt::constants::Address::new_static(
						"Bounties",
						"CuratorDepositMax",
						[
							198u8, 51u8, 89u8, 159u8, 124u8, 251u8, 51u8, 80u8, 167u8, 193u8, 44u8,
							199u8, 80u8, 36u8, 41u8, 130u8, 137u8, 229u8, 178u8, 208u8, 37u8,
							215u8, 169u8, 183u8, 180u8, 191u8, 140u8, 240u8, 250u8, 61u8, 42u8,
							147u8,
						],
					)
				}
				#[doc = " Minimum amount of funds that should be placed in a deposit for making a proposal."]
				pub fn curator_deposit_min(
					&self,
				) -> ::subxt::constants::Address<::core::option::Option<::core::primitive::u128>> {
					::subxt::constants::Address::new_static(
						"Bounties",
						"CuratorDepositMin",
						[
							198u8, 51u8, 89u8, 159u8, 124u8, 251u8, 51u8, 80u8, 167u8, 193u8, 44u8,
							199u8, 80u8, 36u8, 41u8, 130u8, 137u8, 229u8, 178u8, 208u8, 37u8,
							215u8, 169u8, 183u8, 180u8, 191u8, 140u8, 240u8, 250u8, 61u8, 42u8,
							147u8,
						],
					)
				}
				#[doc = " Minimum value for a bounty."]
				pub fn bounty_value_minimum(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Bounties",
						"BountyValueMinimum",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The amount held on deposit per byte within the tip report reason or bounty description."]
				pub fn data_deposit_per_byte(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Bounties",
						"DataDepositPerByte",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " Maximum acceptable reason length."]
				#[doc = ""]
				#[doc = " Benchmarks depend on this value, be sure to update weights file when changing this value"]
				pub fn maximum_reason_length(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Bounties",
						"MaximumReasonLength",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod tips {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_tips::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_tips::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ReportAwesome {
					pub reason: ::std::vec::Vec<::core::primitive::u8>,
					pub who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ReportAwesome {
					const PALLET: &'static str = "Tips";
					const CALL: &'static str = "report_awesome";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RetractTip {
					pub hash: ::subxt::utils::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for RetractTip {
					const PALLET: &'static str = "Tips";
					const CALL: &'static str = "retract_tip";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct TipNew {
					pub reason: ::std::vec::Vec<::core::primitive::u8>,
					pub who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					#[codec(compact)]
					pub tip_value: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for TipNew {
					const PALLET: &'static str = "Tips";
					const CALL: &'static str = "tip_new";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Tip {
					pub hash: ::subxt::utils::H256,
					#[codec(compact)]
					pub tip_value: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for Tip {
					const PALLET: &'static str = "Tips";
					const CALL: &'static str = "tip";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CloseTip {
					pub hash: ::subxt::utils::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for CloseTip {
					const PALLET: &'static str = "Tips";
					const CALL: &'static str = "close_tip";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SlashTip {
					pub hash: ::subxt::utils::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for SlashTip {
					const PALLET: &'static str = "Tips";
					const CALL: &'static str = "slash_tip";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::report_awesome`]."]
				pub fn report_awesome(
					&self,
					reason: ::std::vec::Vec<::core::primitive::u8>,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::ReportAwesome> {
					::subxt::tx::Payload::new_static(
						"Tips",
						"report_awesome",
						types::ReportAwesome { reason, who },
						[
							162u8, 130u8, 216u8, 166u8, 87u8, 77u8, 47u8, 168u8, 146u8, 90u8, 39u8,
							41u8, 160u8, 144u8, 176u8, 42u8, 68u8, 112u8, 240u8, 144u8, 119u8,
							24u8, 69u8, 101u8, 240u8, 212u8, 170u8, 174u8, 155u8, 234u8, 184u8,
							232u8,
						],
					)
				}
				#[doc = "See [`Pallet::retract_tip`]."]
				pub fn retract_tip(
					&self,
					hash: ::subxt::utils::H256,
				) -> ::subxt::tx::Payload<types::RetractTip> {
					::subxt::tx::Payload::new_static(
						"Tips",
						"retract_tip",
						types::RetractTip { hash },
						[
							127u8, 232u8, 112u8, 136u8, 48u8, 227u8, 202u8, 51u8, 78u8, 191u8,
							248u8, 44u8, 159u8, 76u8, 101u8, 107u8, 212u8, 55u8, 85u8, 250u8,
							222u8, 181u8, 58u8, 130u8, 53u8, 103u8, 190u8, 31u8, 113u8, 195u8,
							186u8, 44u8,
						],
					)
				}
				#[doc = "See [`Pallet::tip_new`]."]
				pub fn tip_new(
					&self,
					reason: ::std::vec::Vec<::core::primitive::u8>,
					who: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					tip_value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::TipNew> {
					::subxt::tx::Payload::new_static(
						"Tips",
						"tip_new",
						types::TipNew {
							reason,
							who,
							tip_value,
						},
						[
							236u8, 82u8, 15u8, 103u8, 143u8, 84u8, 161u8, 90u8, 120u8, 185u8,
							251u8, 212u8, 168u8, 2u8, 227u8, 63u8, 222u8, 171u8, 187u8, 137u8,
							177u8, 0u8, 119u8, 97u8, 72u8, 244u8, 31u8, 252u8, 29u8, 19u8, 225u8,
							5u8,
						],
					)
				}
				#[doc = "See [`Pallet::tip`]."]
				pub fn tip(
					&self,
					hash: ::subxt::utils::H256,
					tip_value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::Tip> {
					::subxt::tx::Payload::new_static(
						"Tips",
						"tip",
						types::Tip { hash, tip_value },
						[
							241u8, 5u8, 164u8, 248u8, 140u8, 60u8, 29u8, 9u8, 63u8, 208u8, 249u8,
							210u8, 221u8, 173u8, 70u8, 240u8, 50u8, 131u8, 80u8, 236u8, 131u8,
							101u8, 191u8, 49u8, 94u8, 216u8, 74u8, 234u8, 184u8, 167u8, 159u8,
							176u8,
						],
					)
				}
				#[doc = "See [`Pallet::close_tip`]."]
				pub fn close_tip(
					&self,
					hash: ::subxt::utils::H256,
				) -> ::subxt::tx::Payload<types::CloseTip> {
					::subxt::tx::Payload::new_static(
						"Tips",
						"close_tip",
						types::CloseTip { hash },
						[
							85u8, 213u8, 248u8, 146u8, 90u8, 110u8, 217u8, 109u8, 78u8, 6u8, 104u8,
							71u8, 184u8, 209u8, 148u8, 81u8, 145u8, 71u8, 151u8, 174u8, 25u8,
							238u8, 48u8, 0u8, 51u8, 102u8, 155u8, 143u8, 130u8, 157u8, 100u8,
							246u8,
						],
					)
				}
				#[doc = "See [`Pallet::slash_tip`]."]
				pub fn slash_tip(
					&self,
					hash: ::subxt::utils::H256,
				) -> ::subxt::tx::Payload<types::SlashTip> {
					::subxt::tx::Payload::new_static(
						"Tips",
						"slash_tip",
						types::SlashTip { hash },
						[
							127u8, 21u8, 252u8, 189u8, 121u8, 103u8, 54u8, 155u8, 71u8, 81u8,
							109u8, 0u8, 159u8, 151u8, 62u8, 81u8, 104u8, 31u8, 2u8, 83u8, 248u8,
							141u8, 252u8, 162u8, 173u8, 189u8, 252u8, 249u8, 54u8, 142u8, 108u8,
							19u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_tips::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A new tip suggestion has been opened."]
			pub struct NewTip {
				pub tip_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for NewTip {
				const PALLET: &'static str = "Tips";
				const EVENT: &'static str = "NewTip";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A tip suggestion has reached threshold and is closing."]
			pub struct TipClosing {
				pub tip_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for TipClosing {
				const PALLET: &'static str = "Tips";
				const EVENT: &'static str = "TipClosing";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A tip suggestion has been closed."]
			pub struct TipClosed {
				pub tip_hash: ::subxt::utils::H256,
				pub who: ::subxt::utils::AccountId32,
				pub payout: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for TipClosed {
				const PALLET: &'static str = "Tips";
				const EVENT: &'static str = "TipClosed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A tip suggestion has been retracted."]
			pub struct TipRetracted {
				pub tip_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for TipRetracted {
				const PALLET: &'static str = "Tips";
				const EVENT: &'static str = "TipRetracted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A tip suggestion has been slashed."]
			pub struct TipSlashed {
				pub tip_hash: ::subxt::utils::H256,
				pub finder: ::subxt::utils::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for TipSlashed {
				const PALLET: &'static str = "Tips";
				const EVENT: &'static str = "TipSlashed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " TipsMap that are not yet completed. Keyed by the hash of `(reason, who)` from the value."]
				#[doc = " This has the insecure enumerable hash function since the key itself is already"]
				#[doc = " guaranteed to be a secure hash."]
				pub fn tips(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_tips::OpenTip<
						::subxt::utils::AccountId32,
						::core::primitive::u128,
						::core::primitive::u32,
						::subxt::utils::H256,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Tips",
						"Tips",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							173u8, 172u8, 116u8, 247u8, 202u8, 228u8, 47u8, 222u8, 67u8, 146u8,
							225u8, 0u8, 74u8, 189u8, 226u8, 206u8, 245u8, 209u8, 26u8, 49u8, 189u8,
							73u8, 20u8, 117u8, 30u8, 41u8, 129u8, 170u8, 5u8, 226u8, 92u8, 140u8,
						],
					)
				}
				#[doc = " TipsMap that are not yet completed. Keyed by the hash of `(reason, who)` from the value."]
				#[doc = " This has the insecure enumerable hash function since the key itself is already"]
				#[doc = " guaranteed to be a secure hash."]
				pub fn tips_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_tips::OpenTip<
						::subxt::utils::AccountId32,
						::core::primitive::u128,
						::core::primitive::u32,
						::subxt::utils::H256,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Tips",
						"Tips",
						Vec::new(),
						[
							173u8, 172u8, 116u8, 247u8, 202u8, 228u8, 47u8, 222u8, 67u8, 146u8,
							225u8, 0u8, 74u8, 189u8, 226u8, 206u8, 245u8, 209u8, 26u8, 49u8, 189u8,
							73u8, 20u8, 117u8, 30u8, 41u8, 129u8, 170u8, 5u8, 226u8, 92u8, 140u8,
						],
					)
				}
				#[doc = " Simple preimage lookup from the reason's hash to the original data. Again, has an"]
				#[doc = " insecure enumerable hash since the key is guaranteed to be the result of a secure hash."]
				pub fn reasons(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Tips",
						"Reasons",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							212u8, 224u8, 153u8, 133u8, 234u8, 213u8, 134u8, 255u8, 59u8, 61u8,
							200u8, 47u8, 186u8, 177u8, 35u8, 108u8, 85u8, 144u8, 185u8, 69u8,
							159u8, 38u8, 83u8, 166u8, 200u8, 20u8, 220u8, 234u8, 59u8, 61u8, 223u8,
							167u8,
						],
					)
				}
				#[doc = " Simple preimage lookup from the reason's hash to the original data. Again, has an"]
				#[doc = " insecure enumerable hash since the key is guaranteed to be the result of a secure hash."]
				pub fn reasons_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Tips",
						"Reasons",
						Vec::new(),
						[
							212u8, 224u8, 153u8, 133u8, 234u8, 213u8, 134u8, 255u8, 59u8, 61u8,
							200u8, 47u8, 186u8, 177u8, 35u8, 108u8, 85u8, 144u8, 185u8, 69u8,
							159u8, 38u8, 83u8, 166u8, 200u8, 20u8, 220u8, 234u8, 59u8, 61u8, 223u8,
							167u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Maximum acceptable reason length."]
				#[doc = ""]
				#[doc = " Benchmarks depend on this value, be sure to update weights file when changing this value"]
				pub fn maximum_reason_length(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Tips",
						"MaximumReasonLength",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The amount held on deposit per byte within the tip report reason or bounty description."]
				pub fn data_deposit_per_byte(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Tips",
						"DataDepositPerByte",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The period for which a tip remains open after is has achieved threshold tippers."]
				pub fn tip_countdown(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Tips",
						"TipCountdown",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The percent of the final tip which goes to the original reporter of the tip."]
				pub fn tip_finders_fee(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_arithmetic::per_things::Percent>
				{
					::subxt::constants::Address::new_static(
						"Tips",
						"TipFindersFee",
						[
							40u8, 171u8, 69u8, 196u8, 34u8, 184u8, 50u8, 128u8, 139u8, 192u8, 63u8,
							231u8, 249u8, 200u8, 252u8, 73u8, 244u8, 170u8, 51u8, 177u8, 106u8,
							47u8, 114u8, 234u8, 84u8, 104u8, 62u8, 118u8, 227u8, 50u8, 225u8,
							122u8,
						],
					)
				}
				#[doc = " The amount held on deposit for placing a tip report."]
				pub fn tip_report_deposit_base(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Tips",
						"TipReportDepositBase",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
			}
		}
	}
	pub mod mmr {
		use super::root_mod;
		use super::runtime_types;
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Latest MMR Root hash."]
				pub fn root_hash(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::H256,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Mmr",
						"RootHash",
						vec![],
						[
							111u8, 206u8, 173u8, 92u8, 67u8, 49u8, 150u8, 113u8, 90u8, 245u8, 38u8,
							254u8, 76u8, 250u8, 167u8, 66u8, 130u8, 129u8, 251u8, 220u8, 172u8,
							229u8, 162u8, 251u8, 36u8, 227u8, 43u8, 189u8, 7u8, 106u8, 23u8, 13u8,
						],
					)
				}
				#[doc = " Current size of the MMR (number of leaves)."]
				pub fn number_of_leaves(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Mmr",
						"NumberOfLeaves",
						vec![],
						[
							123u8, 58u8, 149u8, 174u8, 85u8, 45u8, 20u8, 115u8, 241u8, 0u8, 51u8,
							174u8, 234u8, 60u8, 230u8, 59u8, 237u8, 144u8, 170u8, 32u8, 4u8, 0u8,
							34u8, 163u8, 238u8, 205u8, 93u8, 208u8, 53u8, 38u8, 141u8, 195u8,
						],
					)
				}
				#[doc = " Hashes of the nodes in the MMR."]
				#[doc = ""]
				#[doc = " Note this collection only contains MMR peaks, the inner nodes (and leaves)"]
				#[doc = " are pruned and only stored in the Offchain DB."]
				pub fn nodes(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::H256,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Mmr",
						"Nodes",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							27u8, 84u8, 41u8, 195u8, 146u8, 81u8, 211u8, 189u8, 63u8, 125u8, 173u8,
							206u8, 69u8, 198u8, 202u8, 213u8, 89u8, 31u8, 89u8, 177u8, 76u8, 154u8,
							249u8, 197u8, 133u8, 78u8, 142u8, 71u8, 183u8, 3u8, 132u8, 25u8,
						],
					)
				}
				#[doc = " Hashes of the nodes in the MMR."]
				#[doc = ""]
				#[doc = " Note this collection only contains MMR peaks, the inner nodes (and leaves)"]
				#[doc = " are pruned and only stored in the Offchain DB."]
				pub fn nodes_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::H256,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Mmr",
						"Nodes",
						Vec::new(),
						[
							27u8, 84u8, 41u8, 195u8, 146u8, 81u8, 211u8, 189u8, 63u8, 125u8, 173u8,
							206u8, 69u8, 198u8, 202u8, 213u8, 89u8, 31u8, 89u8, 177u8, 76u8, 154u8,
							249u8, 197u8, 133u8, 78u8, 142u8, 71u8, 183u8, 3u8, 132u8, 25u8,
						],
					)
				}
			}
		}
	}
	pub mod data_availability {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Error for the System pallet"]
		pub type Error = runtime_types::da_control::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::da_control::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CreateApplicationKey {
					pub key: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for CreateApplicationKey {
					const PALLET: &'static str = "DataAvailability";
					const CALL: &'static str = "create_application_key";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SubmitData {
					pub data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SubmitData {
					const PALLET: &'static str = "DataAvailability";
					const CALL: &'static str = "submit_data";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SubmitBlockLengthProposal {
					pub rows: ::core::primitive::u32,
					pub cols: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for SubmitBlockLengthProposal {
					const PALLET: &'static str = "DataAvailability";
					const CALL: &'static str = "submit_block_length_proposal";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::create_application_key`]."]
				pub fn create_application_key(
					&self,
					key: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
				) -> ::subxt::tx::Payload<types::CreateApplicationKey> {
					::subxt::tx::Payload::new_static(
						"DataAvailability",
						"create_application_key",
						types::CreateApplicationKey { key },
						[
							132u8, 36u8, 80u8, 95u8, 23u8, 239u8, 223u8, 190u8, 246u8, 23u8, 106u8,
							80u8, 3u8, 253u8, 211u8, 177u8, 188u8, 199u8, 230u8, 186u8, 200u8,
							127u8, 233u8, 250u8, 15u8, 179u8, 253u8, 110u8, 155u8, 159u8, 171u8,
							147u8,
						],
					)
				}
				#[doc = "See [`Pallet::submit_data`]."]
				pub fn submit_data(
					&self,
					data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
				) -> ::subxt::tx::Payload<types::SubmitData> {
					::subxt::tx::Payload::new_static(
						"DataAvailability",
						"submit_data",
						types::SubmitData { data },
						[
							34u8, 211u8, 42u8, 117u8, 145u8, 154u8, 101u8, 75u8, 90u8, 156u8,
							139u8, 245u8, 69u8, 219u8, 231u8, 206u8, 10u8, 66u8, 187u8, 60u8,
							167u8, 77u8, 182u8, 87u8, 237u8, 160u8, 128u8, 205u8, 250u8, 174u8,
							117u8, 123u8,
						],
					)
				}
				#[doc = "See [`Pallet::submit_block_length_proposal`]."]
				pub fn submit_block_length_proposal(
					&self,
					rows: ::core::primitive::u32,
					cols: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::SubmitBlockLengthProposal> {
					::subxt::tx::Payload::new_static(
						"DataAvailability",
						"submit_block_length_proposal",
						types::SubmitBlockLengthProposal { rows, cols },
						[
							181u8, 219u8, 86u8, 153u8, 213u8, 170u8, 136u8, 226u8, 30u8, 205u8,
							69u8, 165u8, 9u8, 70u8, 242u8, 102u8, 150u8, 213u8, 158u8, 33u8, 201u8,
							108u8, 15u8, 88u8, 33u8, 113u8, 168u8, 0u8, 213u8, 196u8, 10u8, 81u8,
						],
					)
				}
			}
		}
		#[doc = "Event for the pallet."]
		pub type Event = runtime_types::da_control::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A new application key was created."]
			pub struct ApplicationKeyCreated {
				pub key: runtime_types::bounded_collections::bounded_vec::BoundedVec<
					::core::primitive::u8,
				>,
				pub owner: ::subxt::utils::AccountId32,
				pub id: runtime_types::avail_core::AppId,
			}
			impl ::subxt::events::StaticEvent for ApplicationKeyCreated {
				const PALLET: &'static str = "DataAvailability";
				const EVENT: &'static str = "ApplicationKeyCreated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct DataSubmitted {
				pub who: ::subxt::utils::AccountId32,
				pub data_hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for DataSubmitted {
				const PALLET: &'static str = "DataAvailability";
				const EVENT: &'static str = "DataSubmitted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BlockLengthProposalSubmitted {
				pub rows: runtime_types::avail_core::BlockLengthRows,
				pub cols: runtime_types::avail_core::BlockLengthColumns,
			}
			impl ::subxt::events::StaticEvent for BlockLengthProposalSubmitted {
				const PALLET: &'static str = "DataAvailability";
				const EVENT: &'static str = "BlockLengthProposalSubmitted";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Last application ID"]
				pub fn next_app_id(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::avail_core::AppId,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"DataAvailability",
						"NextAppId",
						vec![],
						[
							219u8, 175u8, 57u8, 18u8, 143u8, 3u8, 104u8, 185u8, 16u8, 203u8, 242u8,
							96u8, 101u8, 13u8, 78u8, 55u8, 86u8, 82u8, 206u8, 62u8, 9u8, 190u8,
							79u8, 84u8, 88u8, 232u8, 148u8, 190u8, 9u8, 124u8, 176u8, 164u8,
						],
					)
				}
				#[doc = " Store all application keys."]
				pub fn app_keys(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::da_control::pallet::AppKeyInfo<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"DataAvailability",
						"AppKeys",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							225u8, 43u8, 37u8, 100u8, 192u8, 189u8, 228u8, 197u8, 252u8, 31u8,
							67u8, 94u8, 250u8, 213u8, 122u8, 209u8, 171u8, 193u8, 66u8, 243u8,
							212u8, 181u8, 203u8, 254u8, 221u8, 126u8, 189u8, 139u8, 66u8, 232u8,
							181u8, 108u8,
						],
					)
				}
				#[doc = " Store all application keys."]
				pub fn app_keys_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::da_control::pallet::AppKeyInfo<::subxt::utils::AccountId32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"DataAvailability",
						"AppKeys",
						Vec::new(),
						[
							225u8, 43u8, 37u8, 100u8, 192u8, 189u8, 228u8, 197u8, 252u8, 31u8,
							67u8, 94u8, 250u8, 213u8, 122u8, 209u8, 171u8, 193u8, 66u8, 243u8,
							212u8, 181u8, 203u8, 254u8, 221u8, 126u8, 189u8, 139u8, 66u8, 232u8,
							181u8, 108u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The max length of application key."]
				pub fn max_app_key_length(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"DataAvailability",
						"MaxAppKeyLength",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The max length of app data."]
				pub fn max_app_data_length(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"DataAvailability",
						"MaxAppDataLength",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Minimum number of rows in a block."]
				pub fn min_block_rows(
					&self,
				) -> ::subxt::constants::Address<runtime_types::avail_core::BlockLengthRows> {
					::subxt::constants::Address::new_static(
						"DataAvailability",
						"MinBlockRows",
						[
							65u8, 68u8, 180u8, 146u8, 16u8, 133u8, 230u8, 51u8, 17u8, 170u8, 88u8,
							49u8, 171u8, 96u8, 82u8, 152u8, 101u8, 63u8, 46u8, 141u8, 161u8, 104u8,
							204u8, 24u8, 1u8, 20u8, 116u8, 46u8, 232u8, 62u8, 223u8, 15u8,
						],
					)
				}
				#[doc = " Maximum number of rows in a block."]
				pub fn max_block_rows(
					&self,
				) -> ::subxt::constants::Address<runtime_types::avail_core::BlockLengthRows> {
					::subxt::constants::Address::new_static(
						"DataAvailability",
						"MaxBlockRows",
						[
							65u8, 68u8, 180u8, 146u8, 16u8, 133u8, 230u8, 51u8, 17u8, 170u8, 88u8,
							49u8, 171u8, 96u8, 82u8, 152u8, 101u8, 63u8, 46u8, 141u8, 161u8, 104u8,
							204u8, 24u8, 1u8, 20u8, 116u8, 46u8, 232u8, 62u8, 223u8, 15u8,
						],
					)
				}
				#[doc = " Minimum number of cols in a block."]
				pub fn min_block_cols(
					&self,
				) -> ::subxt::constants::Address<runtime_types::avail_core::BlockLengthColumns> {
					::subxt::constants::Address::new_static(
						"DataAvailability",
						"MinBlockCols",
						[
							65u8, 68u8, 180u8, 146u8, 16u8, 133u8, 230u8, 51u8, 17u8, 170u8, 88u8,
							49u8, 171u8, 96u8, 82u8, 152u8, 101u8, 63u8, 46u8, 141u8, 161u8, 104u8,
							204u8, 24u8, 1u8, 20u8, 116u8, 46u8, 232u8, 62u8, 223u8, 15u8,
						],
					)
				}
				#[doc = " Maximum number of cols in a block."]
				pub fn max_block_cols(
					&self,
				) -> ::subxt::constants::Address<runtime_types::avail_core::BlockLengthColumns> {
					::subxt::constants::Address::new_static(
						"DataAvailability",
						"MaxBlockCols",
						[
							65u8, 68u8, 180u8, 146u8, 16u8, 133u8, 230u8, 51u8, 17u8, 170u8, 88u8,
							49u8, 171u8, 96u8, 82u8, 152u8, 101u8, 63u8, 46u8, 141u8, 161u8, 104u8,
							204u8, 24u8, 1u8, 20u8, 116u8, 46u8, 232u8, 62u8, 223u8, 15u8,
						],
					)
				}
			}
		}
	}
	pub mod nomad_updater_manager {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::nomad_updater_manager::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::nomad_updater_manager::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
			}
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::nomad_updater_manager::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NewUpdater {
				pub old_updater: ::subxt::utils::H160,
				pub new_updater: ::subxt::utils::H160,
			}
			impl ::subxt::events::StaticEvent for NewUpdater {
				const PALLET: &'static str = "NomadUpdaterManager";
				const EVENT: &'static str = "NewUpdater";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct FakeSlashed {
				pub reporter: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for FakeSlashed {
				const PALLET: &'static str = "NomadUpdaterManager";
				const EVENT: &'static str = "FakeSlashed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn updater(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::H160,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"NomadUpdaterManager",
						"Updater",
						vec![],
						[
							97u8, 46u8, 26u8, 40u8, 198u8, 108u8, 164u8, 28u8, 138u8, 86u8, 248u8,
							90u8, 34u8, 68u8, 122u8, 144u8, 154u8, 236u8, 244u8, 219u8, 57u8,
							136u8, 163u8, 184u8, 201u8, 5u8, 132u8, 141u8, 114u8, 190u8, 138u8,
							204u8,
						],
					)
				}
			}
		}
	}
	pub mod nomad_home {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::nomad_home::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::nomad_home::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Dispatch {
					#[codec(compact)]
					pub destination_domain: ::core::primitive::u32,
					pub recipient_address: ::subxt::utils::H256,
					pub message_body: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Dispatch {
					const PALLET: &'static str = "NomadHome";
					const CALL: &'static str = "dispatch";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Update {
					pub signed_update: runtime_types::nomad_core::update::SignedUpdate,
					#[codec(compact)]
					pub max_index: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Update {
					const PALLET: &'static str = "NomadHome";
					const CALL: &'static str = "update";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ImproperUpdate {
					pub signed_update: runtime_types::nomad_core::update::SignedUpdate,
				}
				impl ::subxt::blocks::StaticExtrinsic for ImproperUpdate {
					const PALLET: &'static str = "NomadHome";
					const CALL: &'static str = "improper_update";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetUpdater {
					pub new_updater: ::subxt::utils::H160,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetUpdater {
					const PALLET: &'static str = "NomadHome";
					const CALL: &'static str = "set_updater";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::dispatch`]."]
				pub fn dispatch(
					&self,
					destination_domain: ::core::primitive::u32,
					recipient_address: ::subxt::utils::H256,
					message_body: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
				) -> ::subxt::tx::Payload<types::Dispatch> {
					::subxt::tx::Payload::new_static(
						"NomadHome",
						"dispatch",
						types::Dispatch {
							destination_domain,
							recipient_address,
							message_body,
						},
						[
							7u8, 224u8, 49u8, 89u8, 228u8, 226u8, 26u8, 35u8, 45u8, 124u8, 124u8,
							55u8, 26u8, 234u8, 151u8, 239u8, 140u8, 176u8, 228u8, 35u8, 207u8,
							235u8, 97u8, 237u8, 59u8, 91u8, 231u8, 208u8, 89u8, 116u8, 166u8, 67u8,
						],
					)
				}
				#[doc = "See [`Pallet::update`]."]
				pub fn update(
					&self,
					signed_update: runtime_types::nomad_core::update::SignedUpdate,
					max_index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Update> {
					::subxt::tx::Payload::new_static(
						"NomadHome",
						"update",
						types::Update {
							signed_update,
							max_index,
						},
						[
							41u8, 107u8, 21u8, 50u8, 78u8, 61u8, 173u8, 159u8, 208u8, 99u8, 27u8,
							164u8, 166u8, 136u8, 83u8, 219u8, 146u8, 122u8, 148u8, 41u8, 45u8,
							106u8, 13u8, 251u8, 241u8, 74u8, 100u8, 7u8, 59u8, 30u8, 162u8, 174u8,
						],
					)
				}
				#[doc = "See [`Pallet::improper_update`]."]
				pub fn improper_update(
					&self,
					signed_update: runtime_types::nomad_core::update::SignedUpdate,
				) -> ::subxt::tx::Payload<types::ImproperUpdate> {
					::subxt::tx::Payload::new_static(
						"NomadHome",
						"improper_update",
						types::ImproperUpdate { signed_update },
						[
							246u8, 11u8, 33u8, 100u8, 156u8, 239u8, 209u8, 122u8, 10u8, 188u8,
							86u8, 17u8, 145u8, 60u8, 64u8, 102u8, 145u8, 76u8, 82u8, 186u8, 140u8,
							188u8, 173u8, 207u8, 122u8, 119u8, 141u8, 6u8, 82u8, 42u8, 103u8, 87u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_updater`]."]
				pub fn set_updater(
					&self,
					new_updater: ::subxt::utils::H160,
				) -> ::subxt::tx::Payload<types::SetUpdater> {
					::subxt::tx::Payload::new_static(
						"NomadHome",
						"set_updater",
						types::SetUpdater { new_updater },
						[
							127u8, 126u8, 8u8, 188u8, 162u8, 36u8, 215u8, 205u8, 243u8, 213u8,
							108u8, 196u8, 151u8, 61u8, 27u8, 146u8, 162u8, 77u8, 158u8, 85u8,
							165u8, 89u8, 197u8, 244u8, 214u8, 21u8, 109u8, 167u8, 2u8, 139u8,
							191u8, 154u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::nomad_home::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Dispatch {
				pub message_hash: ::subxt::utils::H256,
				pub leaf_index: ::core::primitive::u32,
				pub destination_and_nonce: ::core::primitive::u64,
				pub committed_root: ::subxt::utils::H256,
				pub message: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for Dispatch {
				const PALLET: &'static str = "NomadHome";
				const EVENT: &'static str = "Dispatch";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Update {
				pub home_domain: ::core::primitive::u32,
				pub previous_root: ::subxt::utils::H256,
				pub new_root: ::subxt::utils::H256,
				pub signature: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for Update {
				const PALLET: &'static str = "NomadHome";
				const EVENT: &'static str = "Update";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ImproperUpdate {
				pub previous_root: ::subxt::utils::H256,
				pub new_root: ::subxt::utils::H256,
				pub signature: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for ImproperUpdate {
				const PALLET: &'static str = "NomadHome";
				const EVENT: &'static str = "ImproperUpdate";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UpdaterSlashed {
				pub updater: ::subxt::utils::H160,
				pub reporter: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for UpdaterSlashed {
				const PALLET: &'static str = "NomadHome";
				const EVENT: &'static str = "UpdaterSlashed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn base(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::nomad_base::NomadBase,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"NomadHome",
						"Base",
						vec![],
						[
							254u8, 56u8, 220u8, 109u8, 51u8, 237u8, 70u8, 31u8, 134u8, 49u8, 145u8,
							104u8, 204u8, 243u8, 216u8, 242u8, 57u8, 113u8, 132u8, 190u8, 170u8,
							80u8, 232u8, 190u8, 37u8, 91u8, 20u8, 188u8, 41u8, 41u8, 187u8, 107u8,
						],
					)
				}
				pub fn tree(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::nomad_merkle::light::LightMerkle,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"NomadHome",
						"Tree",
						vec![],
						[
							22u8, 211u8, 114u8, 109u8, 20u8, 79u8, 40u8, 238u8, 244u8, 223u8, 95u8,
							97u8, 85u8, 113u8, 201u8, 242u8, 23u8, 140u8, 106u8, 222u8, 130u8,
							29u8, 218u8, 108u8, 210u8, 13u8, 115u8, 231u8, 22u8, 137u8, 227u8,
							175u8,
						],
					)
				}
				pub fn nonces(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"NomadHome",
						"Nonces",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							155u8, 100u8, 216u8, 144u8, 68u8, 188u8, 75u8, 83u8, 99u8, 245u8, 27u8,
							196u8, 64u8, 193u8, 166u8, 33u8, 47u8, 87u8, 188u8, 46u8, 101u8, 144u8,
							166u8, 231u8, 168u8, 236u8, 81u8, 141u8, 57u8, 185u8, 41u8, 124u8,
						],
					)
				}
				pub fn nonces_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"NomadHome",
						"Nonces",
						Vec::new(),
						[
							155u8, 100u8, 216u8, 144u8, 68u8, 188u8, 75u8, 83u8, 99u8, 245u8, 27u8,
							196u8, 64u8, 193u8, 166u8, 33u8, 47u8, 87u8, 188u8, 46u8, 101u8, 144u8,
							166u8, 231u8, 168u8, 236u8, 81u8, 141u8, 57u8, 185u8, 41u8, 124u8,
						],
					)
				}
				pub fn index_to_root(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::H256,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"NomadHome",
						"IndexToRoot",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							101u8, 71u8, 233u8, 178u8, 28u8, 89u8, 252u8, 186u8, 13u8, 210u8, 44u8,
							76u8, 161u8, 238u8, 35u8, 40u8, 182u8, 168u8, 39u8, 114u8, 91u8, 125u8,
							240u8, 115u8, 4u8, 121u8, 72u8, 90u8, 134u8, 119u8, 47u8, 116u8,
						],
					)
				}
				pub fn index_to_root_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::H256,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"NomadHome",
						"IndexToRoot",
						Vec::new(),
						[
							101u8, 71u8, 233u8, 178u8, 28u8, 89u8, 252u8, 186u8, 13u8, 210u8, 44u8,
							76u8, 161u8, 238u8, 35u8, 40u8, 182u8, 168u8, 39u8, 114u8, 91u8, 125u8,
							240u8, 115u8, 4u8, 121u8, 72u8, 90u8, 134u8, 119u8, 47u8, 116u8,
						],
					)
				}
				pub fn root_to_index(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"NomadHome",
						"RootToIndex",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							82u8, 113u8, 217u8, 4u8, 206u8, 200u8, 21u8, 79u8, 246u8, 211u8, 8u8,
							37u8, 160u8, 89u8, 10u8, 228u8, 211u8, 167u8, 119u8, 98u8, 50u8, 158u8,
							13u8, 130u8, 26u8, 12u8, 132u8, 22u8, 47u8, 54u8, 213u8, 39u8,
						],
					)
				}
				pub fn root_to_index_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"NomadHome",
						"RootToIndex",
						Vec::new(),
						[
							82u8, 113u8, 217u8, 4u8, 206u8, 200u8, 21u8, 79u8, 246u8, 211u8, 8u8,
							37u8, 160u8, 89u8, 10u8, 228u8, 211u8, 167u8, 119u8, 98u8, 50u8, 158u8,
							13u8, 130u8, 26u8, 12u8, 132u8, 22u8, 47u8, 54u8, 213u8, 39u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Max allowed message body size"]
				pub fn max_message_body_bytes(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"NomadHome",
						"MaxMessageBodyBytes",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod nomad_da_bridge {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::nomad_da_bridge::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::nomad_da_bridge::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct TryDispatchDataRoot {
					#[codec(compact)]
					pub destination_domain: ::core::primitive::u32,
					pub recipient_address: ::subxt::utils::H256,
					pub header: ::std::boxed::Box<
						runtime_types::avail_core::header::Header<
							::core::primitive::u32,
							runtime_types::sp_runtime::traits::BlakeTwo256,
						>,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for TryDispatchDataRoot {
					const PALLET: &'static str = "NomadDABridge";
					const CALL: &'static str = "try_dispatch_data_root";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::try_dispatch_data_root`]."]
				pub fn try_dispatch_data_root(
					&self,
					destination_domain: ::core::primitive::u32,
					recipient_address: ::subxt::utils::H256,
					header: runtime_types::avail_core::header::Header<
						::core::primitive::u32,
						runtime_types::sp_runtime::traits::BlakeTwo256,
					>,
				) -> ::subxt::tx::Payload<types::TryDispatchDataRoot> {
					::subxt::tx::Payload::new_static(
						"NomadDABridge",
						"try_dispatch_data_root",
						types::TryDispatchDataRoot {
							destination_domain,
							recipient_address,
							header: ::std::boxed::Box::new(header),
						},
						[
							197u8, 2u8, 92u8, 148u8, 225u8, 214u8, 117u8, 20u8, 80u8, 121u8, 114u8,
							34u8, 128u8, 101u8, 13u8, 214u8, 95u8, 192u8, 51u8, 127u8, 48u8, 138u8,
							155u8, 71u8, 149u8, 179u8, 155u8, 239u8, 247u8, 41u8, 141u8, 250u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::nomad_da_bridge::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct DataRootDispatched {
				pub destination_domain: ::core::primitive::u32,
				pub recipient_address: ::subxt::utils::H256,
				pub block_number: ::core::primitive::u32,
				pub data_root: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for DataRootDispatched {
				const PALLET: &'static str = "NomadDABridge";
				const EVENT: &'static str = "DataRootDispatched";
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn da_bridge_pallet_id(
					&self,
				) -> ::subxt::constants::Address<::subxt::utils::H256> {
					::subxt::constants::Address::new_static(
						"NomadDABridge",
						"DABridgePalletId",
						[
							115u8, 233u8, 13u8, 223u8, 88u8, 20u8, 202u8, 139u8, 153u8, 28u8,
							155u8, 157u8, 224u8, 66u8, 3u8, 250u8, 23u8, 53u8, 88u8, 168u8, 211u8,
							204u8, 122u8, 166u8, 248u8, 23u8, 174u8, 225u8, 99u8, 108u8, 89u8,
							135u8,
						],
					)
				}
			}
		}
	}
	pub mod preimage {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_preimage::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_preimage::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct NotePreimage {
					pub bytes: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for NotePreimage {
					const PALLET: &'static str = "Preimage";
					const CALL: &'static str = "note_preimage";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct UnnotePreimage {
					pub hash: ::subxt::utils::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for UnnotePreimage {
					const PALLET: &'static str = "Preimage";
					const CALL: &'static str = "unnote_preimage";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RequestPreimage {
					pub hash: ::subxt::utils::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for RequestPreimage {
					const PALLET: &'static str = "Preimage";
					const CALL: &'static str = "request_preimage";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct UnrequestPreimage {
					pub hash: ::subxt::utils::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for UnrequestPreimage {
					const PALLET: &'static str = "Preimage";
					const CALL: &'static str = "unrequest_preimage";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::note_preimage`]."]
				pub fn note_preimage(
					&self,
					bytes: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::NotePreimage> {
					::subxt::tx::Payload::new_static(
						"Preimage",
						"note_preimage",
						types::NotePreimage { bytes },
						[
							121u8, 88u8, 18u8, 92u8, 176u8, 15u8, 192u8, 198u8, 146u8, 198u8, 38u8,
							242u8, 213u8, 83u8, 7u8, 230u8, 14u8, 110u8, 235u8, 32u8, 215u8, 26u8,
							192u8, 217u8, 113u8, 224u8, 206u8, 96u8, 177u8, 198u8, 246u8, 33u8,
						],
					)
				}
				#[doc = "See [`Pallet::unnote_preimage`]."]
				pub fn unnote_preimage(
					&self,
					hash: ::subxt::utils::H256,
				) -> ::subxt::tx::Payload<types::UnnotePreimage> {
					::subxt::tx::Payload::new_static(
						"Preimage",
						"unnote_preimage",
						types::UnnotePreimage { hash },
						[
							188u8, 116u8, 222u8, 22u8, 127u8, 215u8, 2u8, 133u8, 96u8, 202u8,
							190u8, 123u8, 203u8, 43u8, 200u8, 161u8, 226u8, 24u8, 49u8, 36u8,
							221u8, 160u8, 130u8, 119u8, 30u8, 138u8, 144u8, 85u8, 5u8, 164u8,
							252u8, 222u8,
						],
					)
				}
				#[doc = "See [`Pallet::request_preimage`]."]
				pub fn request_preimage(
					&self,
					hash: ::subxt::utils::H256,
				) -> ::subxt::tx::Payload<types::RequestPreimage> {
					::subxt::tx::Payload::new_static(
						"Preimage",
						"request_preimage",
						types::RequestPreimage { hash },
						[
							87u8, 0u8, 204u8, 111u8, 43u8, 115u8, 64u8, 209u8, 133u8, 13u8, 83u8,
							45u8, 164u8, 166u8, 233u8, 105u8, 242u8, 238u8, 235u8, 208u8, 113u8,
							134u8, 93u8, 242u8, 86u8, 32u8, 7u8, 152u8, 107u8, 208u8, 79u8, 59u8,
						],
					)
				}
				#[doc = "See [`Pallet::unrequest_preimage`]."]
				pub fn unrequest_preimage(
					&self,
					hash: ::subxt::utils::H256,
				) -> ::subxt::tx::Payload<types::UnrequestPreimage> {
					::subxt::tx::Payload::new_static(
						"Preimage",
						"unrequest_preimage",
						types::UnrequestPreimage { hash },
						[
							55u8, 37u8, 224u8, 149u8, 142u8, 120u8, 8u8, 68u8, 183u8, 225u8, 255u8,
							240u8, 254u8, 111u8, 58u8, 200u8, 113u8, 217u8, 177u8, 203u8, 107u8,
							104u8, 233u8, 87u8, 252u8, 53u8, 33u8, 112u8, 116u8, 254u8, 117u8,
							134u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_preimage::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A preimage has been noted."]
			pub struct Noted {
				pub hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Noted {
				const PALLET: &'static str = "Preimage";
				const EVENT: &'static str = "Noted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A preimage has been requested."]
			pub struct Requested {
				pub hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Requested {
				const PALLET: &'static str = "Preimage";
				const EVENT: &'static str = "Requested";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A preimage has ben cleared."]
			pub struct Cleared {
				pub hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Cleared {
				const PALLET: &'static str = "Preimage";
				const EVENT: &'static str = "Cleared";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The request status of a given hash."]
				pub fn status_for(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_preimage::RequestStatus<
						::subxt::utils::AccountId32,
						::core::primitive::u128,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Preimage",
						"StatusFor",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							176u8, 174u8, 255u8, 131u8, 156u8, 64u8, 181u8, 119u8, 81u8, 243u8,
							144u8, 55u8, 19u8, 140u8, 119u8, 30u8, 210u8, 112u8, 201u8, 247u8,
							13u8, 19u8, 120u8, 190u8, 253u8, 89u8, 4u8, 109u8, 122u8, 62u8, 87u8,
							186u8,
						],
					)
				}
				#[doc = " The request status of a given hash."]
				pub fn status_for_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_preimage::RequestStatus<
						::subxt::utils::AccountId32,
						::core::primitive::u128,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Preimage",
						"StatusFor",
						Vec::new(),
						[
							176u8, 174u8, 255u8, 131u8, 156u8, 64u8, 181u8, 119u8, 81u8, 243u8,
							144u8, 55u8, 19u8, 140u8, 119u8, 30u8, 210u8, 112u8, 201u8, 247u8,
							13u8, 19u8, 120u8, 190u8, 253u8, 89u8, 4u8, 109u8, 122u8, 62u8, 87u8,
							186u8,
						],
					)
				}
				pub fn preimage_for(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Preimage",
						"PreimageFor",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							20u8, 5u8, 33u8, 71u8, 153u8, 129u8, 98u8, 23u8, 214u8, 138u8, 96u8,
							113u8, 245u8, 128u8, 51u8, 55u8, 123u8, 218u8, 165u8, 247u8, 14u8,
							104u8, 119u8, 87u8, 71u8, 222u8, 200u8, 103u8, 58u8, 10u8, 97u8, 134u8,
						],
					)
				}
				pub fn preimage_for_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Preimage",
						"PreimageFor",
						Vec::new(),
						[
							20u8, 5u8, 33u8, 71u8, 153u8, 129u8, 98u8, 23u8, 214u8, 138u8, 96u8,
							113u8, 245u8, 128u8, 51u8, 55u8, 123u8, 218u8, 165u8, 247u8, 14u8,
							104u8, 119u8, 87u8, 71u8, 222u8, 200u8, 103u8, 58u8, 10u8, 97u8, 134u8,
						],
					)
				}
			}
		}
	}
	pub mod multisig {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_multisig::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_multisig::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AsMultiThreshold1 {
					pub other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
					pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for AsMultiThreshold1 {
					const PALLET: &'static str = "Multisig";
					const CALL: &'static str = "as_multi_threshold_1";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AsMulti {
					pub threshold: ::core::primitive::u16,
					pub other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
					pub maybe_timepoint: ::core::option::Option<
						runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
					>,
					pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for AsMulti {
					const PALLET: &'static str = "Multisig";
					const CALL: &'static str = "as_multi";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ApproveAsMulti {
					pub threshold: ::core::primitive::u16,
					pub other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
					pub maybe_timepoint: ::core::option::Option<
						runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
					>,
					pub call_hash: [::core::primitive::u8; 32usize],
					pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for ApproveAsMulti {
					const PALLET: &'static str = "Multisig";
					const CALL: &'static str = "approve_as_multi";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CancelAsMulti {
					pub threshold: ::core::primitive::u16,
					pub other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
					pub timepoint:
						runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
					pub call_hash: [::core::primitive::u8; 32usize],
				}
				impl ::subxt::blocks::StaticExtrinsic for CancelAsMulti {
					const PALLET: &'static str = "Multisig";
					const CALL: &'static str = "cancel_as_multi";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::as_multi_threshold_1`]."]
				pub fn as_multi_threshold_1(
					&self,
					other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
					call: runtime_types::da_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::AsMultiThreshold1> {
					::subxt::tx::Payload::new_static(
						"Multisig",
						"as_multi_threshold_1",
						types::AsMultiThreshold1 {
							other_signatories,
							call: ::std::boxed::Box::new(call),
						},
						[
							179u8, 118u8, 81u8, 158u8, 194u8, 170u8, 17u8, 78u8, 113u8, 164u8,
							102u8, 0u8, 175u8, 8u8, 206u8, 132u8, 228u8, 77u8, 111u8, 99u8, 126u8,
							12u8, 210u8, 193u8, 173u8, 29u8, 0u8, 129u8, 233u8, 16u8, 11u8, 226u8,
						],
					)
				}
				#[doc = "See [`Pallet::as_multi`]."]
				pub fn as_multi(
					&self,
					threshold: ::core::primitive::u16,
					other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
					maybe_timepoint: ::core::option::Option<
						runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
					>,
					call: runtime_types::da_runtime::RuntimeCall,
					max_weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::AsMulti> {
					::subxt::tx::Payload::new_static(
						"Multisig",
						"as_multi",
						types::AsMulti {
							threshold,
							other_signatories,
							maybe_timepoint,
							call: ::std::boxed::Box::new(call),
							max_weight,
						},
						[
							237u8, 210u8, 18u8, 200u8, 161u8, 23u8, 250u8, 101u8, 101u8, 161u8,
							73u8, 123u8, 193u8, 112u8, 154u8, 211u8, 190u8, 168u8, 250u8, 232u8,
							189u8, 225u8, 199u8, 38u8, 87u8, 64u8, 232u8, 95u8, 31u8, 248u8, 178u8,
							44u8,
						],
					)
				}
				#[doc = "See [`Pallet::approve_as_multi`]."]
				pub fn approve_as_multi(
					&self,
					threshold: ::core::primitive::u16,
					other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
					maybe_timepoint: ::core::option::Option<
						runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
					>,
					call_hash: [::core::primitive::u8; 32usize],
					max_weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::ApproveAsMulti> {
					::subxt::tx::Payload::new_static(
						"Multisig",
						"approve_as_multi",
						types::ApproveAsMulti {
							threshold,
							other_signatories,
							maybe_timepoint,
							call_hash,
							max_weight,
						},
						[
							240u8, 17u8, 138u8, 10u8, 165u8, 3u8, 88u8, 240u8, 11u8, 208u8, 9u8,
							123u8, 95u8, 53u8, 142u8, 8u8, 30u8, 5u8, 130u8, 205u8, 102u8, 95u8,
							71u8, 92u8, 184u8, 92u8, 218u8, 224u8, 146u8, 87u8, 93u8, 224u8,
						],
					)
				}
				#[doc = "See [`Pallet::cancel_as_multi`]."]
				pub fn cancel_as_multi(
					&self,
					threshold: ::core::primitive::u16,
					other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
					timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
					call_hash: [::core::primitive::u8; 32usize],
				) -> ::subxt::tx::Payload<types::CancelAsMulti> {
					::subxt::tx::Payload::new_static(
						"Multisig",
						"cancel_as_multi",
						types::CancelAsMulti {
							threshold,
							other_signatories,
							timepoint,
							call_hash,
						},
						[
							14u8, 123u8, 126u8, 239u8, 174u8, 101u8, 28u8, 221u8, 117u8, 75u8,
							82u8, 249u8, 151u8, 59u8, 224u8, 239u8, 54u8, 196u8, 244u8, 46u8, 31u8,
							218u8, 224u8, 58u8, 146u8, 165u8, 135u8, 101u8, 189u8, 93u8, 149u8,
							130u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_multisig::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A new multisig operation has begun."]
			pub struct NewMultisig {
				pub approving: ::subxt::utils::AccountId32,
				pub multisig: ::subxt::utils::AccountId32,
				pub call_hash: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for NewMultisig {
				const PALLET: &'static str = "Multisig";
				const EVENT: &'static str = "NewMultisig";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A multisig operation has been approved by someone."]
			pub struct MultisigApproval {
				pub approving: ::subxt::utils::AccountId32,
				pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				pub multisig: ::subxt::utils::AccountId32,
				pub call_hash: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for MultisigApproval {
				const PALLET: &'static str = "Multisig";
				const EVENT: &'static str = "MultisigApproval";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A multisig operation has been executed."]
			pub struct MultisigExecuted {
				pub approving: ::subxt::utils::AccountId32,
				pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				pub multisig: ::subxt::utils::AccountId32,
				pub call_hash: [::core::primitive::u8; 32usize],
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for MultisigExecuted {
				const PALLET: &'static str = "Multisig";
				const EVENT: &'static str = "MultisigExecuted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A multisig operation has been cancelled."]
			pub struct MultisigCancelled {
				pub cancelling: ::subxt::utils::AccountId32,
				pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				pub multisig: ::subxt::utils::AccountId32,
				pub call_hash: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for MultisigCancelled {
				const PALLET: &'static str = "Multisig";
				const EVENT: &'static str = "MultisigCancelled";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The set of open multisig operations."]
				pub fn multisigs(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8; 32usize]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_multisig::Multisig<
						::core::primitive::u32,
						::core::primitive::u128,
						::subxt::utils::AccountId32,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Multisig",
						"Multisigs",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							22u8, 46u8, 92u8, 90u8, 193u8, 51u8, 12u8, 187u8, 247u8, 141u8, 101u8,
							133u8, 220u8, 5u8, 124u8, 197u8, 149u8, 81u8, 51u8, 194u8, 194u8, 72u8,
							63u8, 249u8, 227u8, 208u8, 58u8, 253u8, 33u8, 107u8, 10u8, 44u8,
						],
					)
				}
				#[doc = " The set of open multisig operations."]
				pub fn multisigs_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_multisig::Multisig<
						::core::primitive::u32,
						::core::primitive::u128,
						::subxt::utils::AccountId32,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Multisig",
						"Multisigs",
						Vec::new(),
						[
							22u8, 46u8, 92u8, 90u8, 193u8, 51u8, 12u8, 187u8, 247u8, 141u8, 101u8,
							133u8, 220u8, 5u8, 124u8, 197u8, 149u8, 81u8, 51u8, 194u8, 194u8, 72u8,
							63u8, 249u8, 227u8, 208u8, 58u8, 253u8, 33u8, 107u8, 10u8, 44u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The base amount of currency needed to reserve for creating a multisig execution or to"]
				#[doc = " store a dispatch call for later."]
				#[doc = ""]
				#[doc = " This is held for an additional storage item whose value size is"]
				#[doc = " `4 + sizeof((BlockNumber, Balance, AccountId))` bytes and whose key size is"]
				#[doc = " `32 + sizeof(AccountId)` bytes."]
				pub fn deposit_base(&self) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Multisig",
						"DepositBase",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The amount of currency needed per unit threshold when creating a multisig execution."]
				#[doc = ""]
				#[doc = " This is held for adding 32 bytes more into a pre-existing storage value."]
				pub fn deposit_factor(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Multisig",
						"DepositFactor",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The maximum amount of signatories allowed in the multisig."]
				pub fn max_signatories(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Multisig",
						"MaxSignatories",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod voter_list {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_bags_list::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_bags_list::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Rebag {
					pub dislocated: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Rebag {
					const PALLET: &'static str = "VoterList";
					const CALL: &'static str = "rebag";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PutInFrontOf {
					pub lighter: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for PutInFrontOf {
					const PALLET: &'static str = "VoterList";
					const CALL: &'static str = "put_in_front_of";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::rebag`]."]
				pub fn rebag(
					&self,
					dislocated: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::Rebag> {
					::subxt::tx::Payload::new_static(
						"VoterList",
						"rebag",
						types::Rebag { dislocated },
						[
							136u8, 83u8, 36u8, 186u8, 29u8, 197u8, 194u8, 38u8, 185u8, 103u8,
							132u8, 73u8, 180u8, 246u8, 141u8, 58u8, 14u8, 74u8, 77u8, 127u8, 235u8,
							179u8, 99u8, 149u8, 243u8, 171u8, 40u8, 97u8, 134u8, 179u8, 2u8, 117u8,
						],
					)
				}
				#[doc = "See [`Pallet::put_in_front_of`]."]
				pub fn put_in_front_of(
					&self,
					lighter: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::PutInFrontOf> {
					::subxt::tx::Payload::new_static(
						"VoterList",
						"put_in_front_of",
						types::PutInFrontOf { lighter },
						[
							61u8, 66u8, 150u8, 46u8, 249u8, 168u8, 25u8, 138u8, 201u8, 233u8,
							191u8, 216u8, 170u8, 197u8, 3u8, 123u8, 116u8, 140u8, 49u8, 226u8,
							65u8, 222u8, 235u8, 27u8, 123u8, 195u8, 169u8, 247u8, 50u8, 125u8,
							245u8, 111u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_bags_list::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Moved an account from one bag to another."]
			pub struct Rebagged {
				pub who: ::subxt::utils::AccountId32,
				pub from: ::core::primitive::u64,
				pub to: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for Rebagged {
				const PALLET: &'static str = "VoterList";
				const EVENT: &'static str = "Rebagged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Updated the score of some account to the given amount."]
			pub struct ScoreUpdated {
				pub who: ::subxt::utils::AccountId32,
				pub new_score: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for ScoreUpdated {
				const PALLET: &'static str = "VoterList";
				const EVENT: &'static str = "ScoreUpdated";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " A single node, within some bag."]
				#[doc = ""]
				#[doc = " Nodes store links forward and back within their respective bags."]
				pub fn list_nodes(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_bags_list::list::Node,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"VoterList",
						"ListNodes",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							252u8, 218u8, 186u8, 230u8, 86u8, 177u8, 112u8, 218u8, 9u8, 62u8,
							217u8, 5u8, 39u8, 70u8, 15u8, 104u8, 157u8, 19u8, 175u8, 136u8, 71u8,
							237u8, 254u8, 254u8, 119u8, 107u8, 84u8, 10u8, 104u8, 142u8, 135u8,
							35u8,
						],
					)
				}
				#[doc = " A single node, within some bag."]
				#[doc = ""]
				#[doc = " Nodes store links forward and back within their respective bags."]
				pub fn list_nodes_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_bags_list::list::Node,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"VoterList",
						"ListNodes",
						Vec::new(),
						[
							252u8, 218u8, 186u8, 230u8, 86u8, 177u8, 112u8, 218u8, 9u8, 62u8,
							217u8, 5u8, 39u8, 70u8, 15u8, 104u8, 157u8, 19u8, 175u8, 136u8, 71u8,
							237u8, 254u8, 254u8, 119u8, 107u8, 84u8, 10u8, 104u8, 142u8, 135u8,
							35u8,
						],
					)
				}
				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_list_nodes(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"VoterList",
						"CounterForListNodes",
						vec![],
						[
							126u8, 150u8, 201u8, 81u8, 155u8, 79u8, 50u8, 48u8, 120u8, 170u8, 3u8,
							104u8, 112u8, 254u8, 106u8, 46u8, 108u8, 126u8, 158u8, 245u8, 95u8,
							88u8, 236u8, 89u8, 79u8, 172u8, 13u8, 146u8, 202u8, 151u8, 122u8,
							132u8,
						],
					)
				}
				#[doc = " A bag stored in storage."]
				#[doc = ""]
				#[doc = " Stores a `Bag` struct, which stores head and tail pointers to itself."]
				pub fn list_bags(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_bags_list::list::Bag,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"VoterList",
						"ListBags",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							157u8, 147u8, 94u8, 26u8, 37u8, 89u8, 114u8, 210u8, 158u8, 36u8, 155u8,
							0u8, 137u8, 78u8, 65u8, 165u8, 226u8, 192u8, 65u8, 13u8, 244u8, 159u8,
							245u8, 15u8, 210u8, 101u8, 61u8, 111u8, 217u8, 225u8, 197u8, 158u8,
						],
					)
				}
				#[doc = " A bag stored in storage."]
				#[doc = ""]
				#[doc = " Stores a `Bag` struct, which stores head and tail pointers to itself."]
				pub fn list_bags_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_bags_list::list::Bag,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"VoterList",
						"ListBags",
						Vec::new(),
						[
							157u8, 147u8, 94u8, 26u8, 37u8, 89u8, 114u8, 210u8, 158u8, 36u8, 155u8,
							0u8, 137u8, 78u8, 65u8, 165u8, 226u8, 192u8, 65u8, 13u8, 244u8, 159u8,
							245u8, 15u8, 210u8, 101u8, 61u8, 111u8, 217u8, 225u8, 197u8, 158u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The list of thresholds separating the various bags."]
				#[doc = ""]
				#[doc = " Ids are separated into unsorted bags according to their score. This specifies the"]
				#[doc = " thresholds separating the bags. An id's bag is the largest bag for which the id's score"]
				#[doc = " is less than or equal to its upper threshold."]
				#[doc = ""]
				#[doc = " When ids are iterated, higher bags are iterated completely before lower bags. This means"]
				#[doc = " that iteration is _semi-sorted_: ids of higher score tend to come before ids of lower"]
				#[doc = " score, but peer ids within a particular bag are sorted in insertion order."]
				#[doc = ""]
				#[doc = " # Expressing the constant"]
				#[doc = ""]
				#[doc = " This constant must be sorted in strictly increasing order. Duplicate items are not"]
				#[doc = " permitted."]
				#[doc = ""]
				#[doc = " There is an implied upper limit of `Score::MAX`; that value does not need to be"]
				#[doc = " specified within the bag. For any two threshold lists, if one ends with"]
				#[doc = " `Score::MAX`, the other one does not, and they are otherwise equal, the two"]
				#[doc = " lists will behave identically."]
				#[doc = ""]
				#[doc = " # Calculation"]
				#[doc = ""]
				#[doc = " It is recommended to generate the set of thresholds in a geometric series, such that"]
				#[doc = " there exists some constant ratio such that `threshold[k + 1] == (threshold[k] *"]
				#[doc = " constant_ratio).max(threshold[k] + 1)` for all `k`."]
				#[doc = ""]
				#[doc = " The helpers in the `/utils/frame/generate-bags` module can simplify this calculation."]
				#[doc = ""]
				#[doc = " # Examples"]
				#[doc = ""]
				#[doc = " - If `BagThresholds::get().is_empty()`, then all ids are put into the same bag, and"]
				#[doc = "   iteration is strictly in insertion order."]
				#[doc = " - If `BagThresholds::get().len() == 64`, and the thresholds are determined according to"]
				#[doc = "   the procedure given above, then the constant ratio is equal to 2."]
				#[doc = " - If `BagThresholds::get().len() == 200`, and the thresholds are determined according to"]
				#[doc = "   the procedure given above, then the constant ratio is approximately equal to 1.248."]
				#[doc = " - If the threshold list begins `[1, 2, 3, ...]`, then an id with score 0 or 1 will fall"]
				#[doc = "   into bag 0, an id with score 2 will fall into bag 1, etc."]
				#[doc = ""]
				#[doc = " # Migration"]
				#[doc = ""]
				#[doc = " In the event that this list ever changes, a copy of the old bags list must be retained."]
				#[doc = " With that `List::migrate` can be called, which will perform the appropriate migration."]
				pub fn bag_thresholds(
					&self,
				) -> ::subxt::constants::Address<::std::vec::Vec<::core::primitive::u64>> {
					::subxt::constants::Address::new_static(
						"VoterList",
						"BagThresholds",
						[
							215u8, 118u8, 183u8, 172u8, 4u8, 42u8, 248u8, 108u8, 4u8, 110u8, 43u8,
							165u8, 228u8, 7u8, 36u8, 30u8, 135u8, 184u8, 56u8, 201u8, 107u8, 68u8,
							25u8, 164u8, 134u8, 32u8, 82u8, 107u8, 200u8, 219u8, 212u8, 198u8,
						],
					)
				}
			}
		}
	}
	pub mod nomination_pools {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_nomination_pools::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_nomination_pools::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Join {
					#[codec(compact)]
					pub amount: ::core::primitive::u128,
					pub pool_id: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Join {
					const PALLET: &'static str = "NominationPools";
					const CALL: &'static str = "join";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BondExtra {
					pub extra:
						runtime_types::pallet_nomination_pools::BondExtra<::core::primitive::u128>,
				}
				impl ::subxt::blocks::StaticExtrinsic for BondExtra {
					const PALLET: &'static str = "NominationPools";
					const CALL: &'static str = "bond_extra";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ClaimPayout;
				impl ::subxt::blocks::StaticExtrinsic for ClaimPayout {
					const PALLET: &'static str = "NominationPools";
					const CALL: &'static str = "claim_payout";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Unbond {
					pub member_account: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					#[codec(compact)]
					pub unbonding_points: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for Unbond {
					const PALLET: &'static str = "NominationPools";
					const CALL: &'static str = "unbond";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PoolWithdrawUnbonded {
					pub pool_id: ::core::primitive::u32,
					pub num_slashing_spans: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for PoolWithdrawUnbonded {
					const PALLET: &'static str = "NominationPools";
					const CALL: &'static str = "pool_withdraw_unbonded";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct WithdrawUnbonded {
					pub member_account: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					pub num_slashing_spans: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for WithdrawUnbonded {
					const PALLET: &'static str = "NominationPools";
					const CALL: &'static str = "withdraw_unbonded";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Create {
					#[codec(compact)]
					pub amount: ::core::primitive::u128,
					pub root: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					pub nominator: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					pub bouncer: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Create {
					const PALLET: &'static str = "NominationPools";
					const CALL: &'static str = "create";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CreateWithPoolId {
					#[codec(compact)]
					pub amount: ::core::primitive::u128,
					pub root: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					pub nominator: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					pub bouncer: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					pub pool_id: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for CreateWithPoolId {
					const PALLET: &'static str = "NominationPools";
					const CALL: &'static str = "create_with_pool_id";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Nominate {
					pub pool_id: ::core::primitive::u32,
					pub validators: ::std::vec::Vec<::subxt::utils::AccountId32>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Nominate {
					const PALLET: &'static str = "NominationPools";
					const CALL: &'static str = "nominate";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetState {
					pub pool_id: ::core::primitive::u32,
					pub state: runtime_types::pallet_nomination_pools::PoolState,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetState {
					const PALLET: &'static str = "NominationPools";
					const CALL: &'static str = "set_state";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetMetadata {
					pub pool_id: ::core::primitive::u32,
					pub metadata: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetMetadata {
					const PALLET: &'static str = "NominationPools";
					const CALL: &'static str = "set_metadata";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetConfigs {
					pub min_join_bond:
						runtime_types::pallet_nomination_pools::ConfigOp<::core::primitive::u128>,
					pub min_create_bond:
						runtime_types::pallet_nomination_pools::ConfigOp<::core::primitive::u128>,
					pub max_pools:
						runtime_types::pallet_nomination_pools::ConfigOp<::core::primitive::u32>,
					pub max_members:
						runtime_types::pallet_nomination_pools::ConfigOp<::core::primitive::u32>,
					pub max_members_per_pool:
						runtime_types::pallet_nomination_pools::ConfigOp<::core::primitive::u32>,
					pub global_max_commission: runtime_types::pallet_nomination_pools::ConfigOp<
						runtime_types::sp_arithmetic::per_things::Perbill,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetConfigs {
					const PALLET: &'static str = "NominationPools";
					const CALL: &'static str = "set_configs";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct UpdateRoles {
					pub pool_id: ::core::primitive::u32,
					pub new_root: runtime_types::pallet_nomination_pools::ConfigOp<
						::subxt::utils::AccountId32,
					>,
					pub new_nominator: runtime_types::pallet_nomination_pools::ConfigOp<
						::subxt::utils::AccountId32,
					>,
					pub new_bouncer: runtime_types::pallet_nomination_pools::ConfigOp<
						::subxt::utils::AccountId32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateRoles {
					const PALLET: &'static str = "NominationPools";
					const CALL: &'static str = "update_roles";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Chill {
					pub pool_id: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for Chill {
					const PALLET: &'static str = "NominationPools";
					const CALL: &'static str = "chill";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BondExtraOther {
					pub member: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					pub extra:
						runtime_types::pallet_nomination_pools::BondExtra<::core::primitive::u128>,
				}
				impl ::subxt::blocks::StaticExtrinsic for BondExtraOther {
					const PALLET: &'static str = "NominationPools";
					const CALL: &'static str = "bond_extra_other";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetClaimPermission {
					pub permission: runtime_types::pallet_nomination_pools::ClaimPermission,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetClaimPermission {
					const PALLET: &'static str = "NominationPools";
					const CALL: &'static str = "set_claim_permission";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ClaimPayoutOther {
					pub other: ::subxt::utils::AccountId32,
				}
				impl ::subxt::blocks::StaticExtrinsic for ClaimPayoutOther {
					const PALLET: &'static str = "NominationPools";
					const CALL: &'static str = "claim_payout_other";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetCommission {
					pub pool_id: ::core::primitive::u32,
					pub new_commission: ::core::option::Option<(
						runtime_types::sp_arithmetic::per_things::Perbill,
						::subxt::utils::AccountId32,
					)>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetCommission {
					const PALLET: &'static str = "NominationPools";
					const CALL: &'static str = "set_commission";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetCommissionMax {
					pub pool_id: ::core::primitive::u32,
					pub max_commission: runtime_types::sp_arithmetic::per_things::Perbill,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetCommissionMax {
					const PALLET: &'static str = "NominationPools";
					const CALL: &'static str = "set_commission_max";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetCommissionChangeRate {
					pub pool_id: ::core::primitive::u32,
					pub change_rate: runtime_types::pallet_nomination_pools::CommissionChangeRate<
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetCommissionChangeRate {
					const PALLET: &'static str = "NominationPools";
					const CALL: &'static str = "set_commission_change_rate";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ClaimCommission {
					pub pool_id: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for ClaimCommission {
					const PALLET: &'static str = "NominationPools";
					const CALL: &'static str = "claim_commission";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::join`]."]
				pub fn join(
					&self,
					amount: ::core::primitive::u128,
					pool_id: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Join> {
					::subxt::tx::Payload::new_static(
						"NominationPools",
						"join",
						types::Join { amount, pool_id },
						[
							9u8, 24u8, 209u8, 117u8, 242u8, 76u8, 192u8, 40u8, 196u8, 136u8, 158u8,
							182u8, 117u8, 140u8, 164u8, 64u8, 184u8, 160u8, 146u8, 143u8, 173u8,
							180u8, 6u8, 242u8, 203u8, 130u8, 41u8, 176u8, 158u8, 96u8, 94u8, 175u8,
						],
					)
				}
				#[doc = "See [`Pallet::bond_extra`]."]
				pub fn bond_extra(
					&self,
					extra: runtime_types::pallet_nomination_pools::BondExtra<
						::core::primitive::u128,
					>,
				) -> ::subxt::tx::Payload<types::BondExtra> {
					::subxt::tx::Payload::new_static(
						"NominationPools",
						"bond_extra",
						types::BondExtra { extra },
						[
							149u8, 176u8, 102u8, 52u8, 76u8, 227u8, 61u8, 60u8, 109u8, 187u8, 40u8,
							176u8, 163u8, 37u8, 10u8, 228u8, 164u8, 77u8, 155u8, 155u8, 14u8,
							106u8, 5u8, 177u8, 176u8, 224u8, 163u8, 28u8, 66u8, 237u8, 186u8,
							188u8,
						],
					)
				}
				#[doc = "See [`Pallet::claim_payout`]."]
				pub fn claim_payout(&self) -> ::subxt::tx::Payload<types::ClaimPayout> {
					::subxt::tx::Payload::new_static(
						"NominationPools",
						"claim_payout",
						types::ClaimPayout {},
						[
							28u8, 87u8, 180u8, 5u8, 69u8, 49u8, 121u8, 28u8, 34u8, 63u8, 78u8,
							228u8, 223u8, 12u8, 171u8, 41u8, 181u8, 137u8, 145u8, 141u8, 198u8,
							220u8, 5u8, 101u8, 173u8, 69u8, 222u8, 59u8, 111u8, 92u8, 182u8, 8u8,
						],
					)
				}
				#[doc = "See [`Pallet::unbond`]."]
				pub fn unbond(
					&self,
					member_account: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					unbonding_points: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::Unbond> {
					::subxt::tx::Payload::new_static(
						"NominationPools",
						"unbond",
						types::Unbond {
							member_account,
							unbonding_points,
						},
						[
							230u8, 112u8, 85u8, 216u8, 164u8, 250u8, 5u8, 18u8, 50u8, 206u8, 194u8,
							0u8, 167u8, 149u8, 220u8, 212u8, 138u8, 33u8, 117u8, 130u8, 47u8,
							197u8, 113u8, 30u8, 84u8, 83u8, 1u8, 105u8, 58u8, 182u8, 44u8, 225u8,
						],
					)
				}
				#[doc = "See [`Pallet::pool_withdraw_unbonded`]."]
				pub fn pool_withdraw_unbonded(
					&self,
					pool_id: ::core::primitive::u32,
					num_slashing_spans: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::PoolWithdrawUnbonded> {
					::subxt::tx::Payload::new_static(
						"NominationPools",
						"pool_withdraw_unbonded",
						types::PoolWithdrawUnbonded {
							pool_id,
							num_slashing_spans,
						},
						[
							234u8, 49u8, 43u8, 199u8, 55u8, 2u8, 252u8, 39u8, 147u8, 136u8, 34u8,
							239u8, 116u8, 155u8, 129u8, 72u8, 83u8, 161u8, 90u8, 207u8, 1u8, 193u8,
							254u8, 47u8, 40u8, 185u8, 67u8, 55u8, 238u8, 122u8, 140u8, 230u8,
						],
					)
				}
				#[doc = "See [`Pallet::withdraw_unbonded`]."]
				pub fn withdraw_unbonded(
					&self,
					member_account: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					num_slashing_spans: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::WithdrawUnbonded> {
					::subxt::tx::Payload::new_static(
						"NominationPools",
						"withdraw_unbonded",
						types::WithdrawUnbonded {
							member_account,
							num_slashing_spans,
						},
						[
							252u8, 38u8, 22u8, 98u8, 64u8, 133u8, 118u8, 62u8, 179u8, 226u8, 212u8,
							186u8, 32u8, 227u8, 253u8, 119u8, 200u8, 53u8, 35u8, 193u8, 20u8, 64u8,
							232u8, 35u8, 221u8, 37u8, 63u8, 10u8, 7u8, 250u8, 176u8, 238u8,
						],
					)
				}
				#[doc = "See [`Pallet::create`]."]
				pub fn create(
					&self,
					amount: ::core::primitive::u128,
					root: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					nominator: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					bouncer: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::Create> {
					::subxt::tx::Payload::new_static(
						"NominationPools",
						"create",
						types::Create {
							amount,
							root,
							nominator,
							bouncer,
						},
						[
							4u8, 111u8, 53u8, 212u8, 226u8, 159u8, 144u8, 191u8, 152u8, 252u8,
							30u8, 169u8, 185u8, 150u8, 229u8, 234u8, 140u8, 46u8, 237u8, 197u8,
							164u8, 233u8, 184u8, 60u8, 51u8, 184u8, 71u8, 40u8, 61u8, 140u8, 204u8,
							100u8,
						],
					)
				}
				#[doc = "See [`Pallet::create_with_pool_id`]."]
				pub fn create_with_pool_id(
					&self,
					amount: ::core::primitive::u128,
					root: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					nominator: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					bouncer: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					pool_id: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::CreateWithPoolId> {
					::subxt::tx::Payload::new_static(
						"NominationPools",
						"create_with_pool_id",
						types::CreateWithPoolId {
							amount,
							root,
							nominator,
							bouncer,
							pool_id,
						},
						[
							102u8, 254u8, 178u8, 47u8, 79u8, 12u8, 230u8, 142u8, 226u8, 205u8,
							25u8, 166u8, 133u8, 0u8, 172u8, 58u8, 75u8, 96u8, 71u8, 187u8, 237u8,
							173u8, 156u8, 20u8, 78u8, 48u8, 62u8, 35u8, 130u8, 8u8, 117u8, 222u8,
						],
					)
				}
				#[doc = "See [`Pallet::nominate`]."]
				pub fn nominate(
					&self,
					pool_id: ::core::primitive::u32,
					validators: ::std::vec::Vec<::subxt::utils::AccountId32>,
				) -> ::subxt::tx::Payload<types::Nominate> {
					::subxt::tx::Payload::new_static(
						"NominationPools",
						"nominate",
						types::Nominate {
							pool_id,
							validators,
						},
						[
							118u8, 80u8, 137u8, 47u8, 102u8, 9u8, 20u8, 136u8, 76u8, 164u8, 161u8,
							114u8, 33u8, 159u8, 204u8, 49u8, 233u8, 199u8, 246u8, 67u8, 144u8,
							169u8, 211u8, 67u8, 12u8, 68u8, 198u8, 149u8, 87u8, 62u8, 226u8, 72u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_state`]."]
				pub fn set_state(
					&self,
					pool_id: ::core::primitive::u32,
					state: runtime_types::pallet_nomination_pools::PoolState,
				) -> ::subxt::tx::Payload<types::SetState> {
					::subxt::tx::Payload::new_static(
						"NominationPools",
						"set_state",
						types::SetState { pool_id, state },
						[
							39u8, 221u8, 24u8, 65u8, 144u8, 230u8, 228u8, 24u8, 191u8, 53u8, 171u8,
							148u8, 131u8, 45u8, 10u8, 22u8, 222u8, 240u8, 13u8, 87u8, 123u8, 182u8,
							102u8, 26u8, 124u8, 205u8, 23u8, 31u8, 25u8, 43u8, 12u8, 140u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_metadata`]."]
				pub fn set_metadata(
					&self,
					pool_id: ::core::primitive::u32,
					metadata: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::SetMetadata> {
					::subxt::tx::Payload::new_static(
						"NominationPools",
						"set_metadata",
						types::SetMetadata { pool_id, metadata },
						[
							221u8, 189u8, 15u8, 232u8, 0u8, 49u8, 187u8, 67u8, 124u8, 26u8, 114u8,
							191u8, 81u8, 14u8, 253u8, 75u8, 88u8, 182u8, 136u8, 18u8, 238u8, 119u8,
							215u8, 248u8, 133u8, 160u8, 154u8, 193u8, 177u8, 140u8, 1u8, 16u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_configs`]."]
				pub fn set_configs(
					&self,
					min_join_bond: runtime_types::pallet_nomination_pools::ConfigOp<
						::core::primitive::u128,
					>,
					min_create_bond: runtime_types::pallet_nomination_pools::ConfigOp<
						::core::primitive::u128,
					>,
					max_pools: runtime_types::pallet_nomination_pools::ConfigOp<
						::core::primitive::u32,
					>,
					max_members: runtime_types::pallet_nomination_pools::ConfigOp<
						::core::primitive::u32,
					>,
					max_members_per_pool: runtime_types::pallet_nomination_pools::ConfigOp<
						::core::primitive::u32,
					>,
					global_max_commission: runtime_types::pallet_nomination_pools::ConfigOp<
						runtime_types::sp_arithmetic::per_things::Perbill,
					>,
				) -> ::subxt::tx::Payload<types::SetConfigs> {
					::subxt::tx::Payload::new_static(
						"NominationPools",
						"set_configs",
						types::SetConfigs {
							min_join_bond,
							min_create_bond,
							max_pools,
							max_members,
							max_members_per_pool,
							global_max_commission,
						},
						[
							60u8, 29u8, 13u8, 45u8, 37u8, 171u8, 129u8, 133u8, 127u8, 42u8, 104u8,
							45u8, 29u8, 58u8, 209u8, 48u8, 119u8, 255u8, 86u8, 13u8, 243u8, 124u8,
							57u8, 250u8, 156u8, 189u8, 59u8, 88u8, 64u8, 109u8, 219u8, 68u8,
						],
					)
				}
				#[doc = "See [`Pallet::update_roles`]."]
				pub fn update_roles(
					&self,
					pool_id: ::core::primitive::u32,
					new_root: runtime_types::pallet_nomination_pools::ConfigOp<
						::subxt::utils::AccountId32,
					>,
					new_nominator: runtime_types::pallet_nomination_pools::ConfigOp<
						::subxt::utils::AccountId32,
					>,
					new_bouncer: runtime_types::pallet_nomination_pools::ConfigOp<
						::subxt::utils::AccountId32,
					>,
				) -> ::subxt::tx::Payload<types::UpdateRoles> {
					::subxt::tx::Payload::new_static(
						"NominationPools",
						"update_roles",
						types::UpdateRoles {
							pool_id,
							new_root,
							new_nominator,
							new_bouncer,
						},
						[
							58u8, 51u8, 136u8, 162u8, 218u8, 195u8, 121u8, 6u8, 243u8, 69u8, 19u8,
							130u8, 152u8, 180u8, 226u8, 28u8, 0u8, 218u8, 237u8, 56u8, 52u8, 139u8,
							198u8, 155u8, 112u8, 165u8, 142u8, 44u8, 111u8, 197u8, 123u8, 246u8,
						],
					)
				}
				#[doc = "See [`Pallet::chill`]."]
				pub fn chill(
					&self,
					pool_id: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::Chill> {
					::subxt::tx::Payload::new_static(
						"NominationPools",
						"chill",
						types::Chill { pool_id },
						[
							65u8, 206u8, 54u8, 53u8, 37u8, 97u8, 161u8, 104u8, 62u8, 9u8, 93u8,
							236u8, 61u8, 185u8, 204u8, 245u8, 234u8, 218u8, 213u8, 40u8, 154u8,
							29u8, 244u8, 19u8, 207u8, 172u8, 142u8, 221u8, 38u8, 70u8, 39u8, 10u8,
						],
					)
				}
				#[doc = "See [`Pallet::bond_extra_other`]."]
				pub fn bond_extra_other(
					&self,
					member: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					extra: runtime_types::pallet_nomination_pools::BondExtra<
						::core::primitive::u128,
					>,
				) -> ::subxt::tx::Payload<types::BondExtraOther> {
					::subxt::tx::Payload::new_static(
						"NominationPools",
						"bond_extra_other",
						types::BondExtraOther { member, extra },
						[
							217u8, 91u8, 160u8, 244u8, 122u8, 153u8, 236u8, 132u8, 197u8, 31u8,
							124u8, 22u8, 89u8, 146u8, 173u8, 96u8, 167u8, 220u8, 91u8, 27u8, 230u8,
							128u8, 222u8, 128u8, 97u8, 53u8, 226u8, 222u8, 165u8, 133u8, 54u8,
							236u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_claim_permission`]."]
				pub fn set_claim_permission(
					&self,
					permission: runtime_types::pallet_nomination_pools::ClaimPermission,
				) -> ::subxt::tx::Payload<types::SetClaimPermission> {
					::subxt::tx::Payload::new_static(
						"NominationPools",
						"set_claim_permission",
						types::SetClaimPermission { permission },
						[
							36u8, 137u8, 193u8, 200u8, 57u8, 46u8, 87u8, 236u8, 180u8, 170u8, 90u8,
							99u8, 137u8, 123u8, 99u8, 197u8, 113u8, 119u8, 72u8, 153u8, 207u8,
							189u8, 69u8, 89u8, 225u8, 115u8, 45u8, 32u8, 216u8, 43u8, 92u8, 135u8,
						],
					)
				}
				#[doc = "See [`Pallet::claim_payout_other`]."]
				pub fn claim_payout_other(
					&self,
					other: ::subxt::utils::AccountId32,
				) -> ::subxt::tx::Payload<types::ClaimPayoutOther> {
					::subxt::tx::Payload::new_static(
						"NominationPools",
						"claim_payout_other",
						types::ClaimPayoutOther { other },
						[
							202u8, 130u8, 122u8, 10u8, 159u8, 181u8, 124u8, 215u8, 23u8, 85u8,
							234u8, 178u8, 169u8, 41u8, 204u8, 226u8, 195u8, 69u8, 168u8, 88u8,
							58u8, 15u8, 3u8, 227u8, 180u8, 183u8, 62u8, 224u8, 39u8, 218u8, 75u8,
							166u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_commission`]."]
				pub fn set_commission(
					&self,
					pool_id: ::core::primitive::u32,
					new_commission: ::core::option::Option<(
						runtime_types::sp_arithmetic::per_things::Perbill,
						::subxt::utils::AccountId32,
					)>,
				) -> ::subxt::tx::Payload<types::SetCommission> {
					::subxt::tx::Payload::new_static(
						"NominationPools",
						"set_commission",
						types::SetCommission {
							pool_id,
							new_commission,
						},
						[
							144u8, 94u8, 73u8, 69u8, 224u8, 158u8, 244u8, 77u8, 169u8, 219u8,
							101u8, 41u8, 37u8, 211u8, 198u8, 32u8, 92u8, 108u8, 7u8, 27u8, 153u8,
							37u8, 82u8, 174u8, 196u8, 176u8, 196u8, 181u8, 45u8, 81u8, 134u8,
							162u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_commission_max`]."]
				pub fn set_commission_max(
					&self,
					pool_id: ::core::primitive::u32,
					max_commission: runtime_types::sp_arithmetic::per_things::Perbill,
				) -> ::subxt::tx::Payload<types::SetCommissionMax> {
					::subxt::tx::Payload::new_static(
						"NominationPools",
						"set_commission_max",
						types::SetCommissionMax {
							pool_id,
							max_commission,
						},
						[
							180u8, 80u8, 204u8, 129u8, 141u8, 86u8, 45u8, 76u8, 224u8, 123u8,
							212u8, 38u8, 224u8, 79u8, 41u8, 143u8, 237u8, 174u8, 126u8, 1u8, 215u8,
							105u8, 50u8, 46u8, 151u8, 11u8, 118u8, 198u8, 183u8, 95u8, 47u8, 71u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_commission_change_rate`]."]
				pub fn set_commission_change_rate(
					&self,
					pool_id: ::core::primitive::u32,
					change_rate: runtime_types::pallet_nomination_pools::CommissionChangeRate<
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::SetCommissionChangeRate> {
					::subxt::tx::Payload::new_static(
						"NominationPools",
						"set_commission_change_rate",
						types::SetCommissionChangeRate {
							pool_id,
							change_rate,
						},
						[
							138u8, 30u8, 155u8, 127u8, 181u8, 99u8, 89u8, 138u8, 130u8, 53u8,
							224u8, 96u8, 190u8, 14u8, 76u8, 244u8, 142u8, 50u8, 39u8, 245u8, 144u8,
							87u8, 64u8, 206u8, 246u8, 225u8, 111u8, 197u8, 245u8, 182u8, 121u8,
							56u8,
						],
					)
				}
				#[doc = "See [`Pallet::claim_commission`]."]
				pub fn claim_commission(
					&self,
					pool_id: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::ClaimCommission> {
					::subxt::tx::Payload::new_static(
						"NominationPools",
						"claim_commission",
						types::ClaimCommission { pool_id },
						[
							51u8, 64u8, 163u8, 230u8, 2u8, 119u8, 68u8, 5u8, 154u8, 4u8, 84u8,
							149u8, 9u8, 195u8, 173u8, 37u8, 98u8, 48u8, 188u8, 65u8, 81u8, 11u8,
							64u8, 254u8, 126u8, 62u8, 29u8, 204u8, 92u8, 230u8, 240u8, 91u8,
						],
					)
				}
			}
		}
		#[doc = "Events of this pallet."]
		pub type Event = runtime_types::pallet_nomination_pools::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A pool has been created."]
			pub struct Created {
				pub depositor: ::subxt::utils::AccountId32,
				pub pool_id: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Created {
				const PALLET: &'static str = "NominationPools";
				const EVENT: &'static str = "Created";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A member has became bonded in a pool."]
			pub struct Bonded {
				pub member: ::subxt::utils::AccountId32,
				pub pool_id: ::core::primitive::u32,
				pub bonded: ::core::primitive::u128,
				pub joined: ::core::primitive::bool,
			}
			impl ::subxt::events::StaticEvent for Bonded {
				const PALLET: &'static str = "NominationPools";
				const EVENT: &'static str = "Bonded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A payout has been made to a member."]
			pub struct PaidOut {
				pub member: ::subxt::utils::AccountId32,
				pub pool_id: ::core::primitive::u32,
				pub payout: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for PaidOut {
				const PALLET: &'static str = "NominationPools";
				const EVENT: &'static str = "PaidOut";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A member has unbonded from their pool."]
			#[doc = ""]
			#[doc = "- `balance` is the corresponding balance of the number of points that has been"]
			#[doc = "  requested to be unbonded (the argument of the `unbond` transaction) from the bonded"]
			#[doc = "  pool."]
			#[doc = "- `points` is the number of points that are issued as a result of `balance` being"]
			#[doc = "dissolved into the corresponding unbonding pool."]
			#[doc = "- `era` is the era in which the balance will be unbonded."]
			#[doc = "In the absence of slashing, these values will match. In the presence of slashing, the"]
			#[doc = "number of points that are issued in the unbonding pool will be less than the amount"]
			#[doc = "requested to be unbonded."]
			pub struct Unbonded {
				pub member: ::subxt::utils::AccountId32,
				pub pool_id: ::core::primitive::u32,
				pub balance: ::core::primitive::u128,
				pub points: ::core::primitive::u128,
				pub era: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Unbonded {
				const PALLET: &'static str = "NominationPools";
				const EVENT: &'static str = "Unbonded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A member has withdrawn from their pool."]
			#[doc = ""]
			#[doc = "The given number of `points` have been dissolved in return of `balance`."]
			#[doc = ""]
			#[doc = "Similar to `Unbonded` event, in the absence of slashing, the ratio of point to balance"]
			#[doc = "will be 1."]
			pub struct Withdrawn {
				pub member: ::subxt::utils::AccountId32,
				pub pool_id: ::core::primitive::u32,
				pub balance: ::core::primitive::u128,
				pub points: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Withdrawn {
				const PALLET: &'static str = "NominationPools";
				const EVENT: &'static str = "Withdrawn";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A pool has been destroyed."]
			pub struct Destroyed {
				pub pool_id: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Destroyed {
				const PALLET: &'static str = "NominationPools";
				const EVENT: &'static str = "Destroyed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The state of a pool has changed"]
			pub struct StateChanged {
				pub pool_id: ::core::primitive::u32,
				pub new_state: runtime_types::pallet_nomination_pools::PoolState,
			}
			impl ::subxt::events::StaticEvent for StateChanged {
				const PALLET: &'static str = "NominationPools";
				const EVENT: &'static str = "StateChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A member has been removed from a pool."]
			#[doc = ""]
			#[doc = "The removal can be voluntary (withdrawn all unbonded funds) or involuntary (kicked)."]
			pub struct MemberRemoved {
				pub pool_id: ::core::primitive::u32,
				pub member: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for MemberRemoved {
				const PALLET: &'static str = "NominationPools";
				const EVENT: &'static str = "MemberRemoved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The roles of a pool have been updated to the given new roles. Note that the depositor"]
			#[doc = "can never change."]
			pub struct RolesUpdated {
				pub root: ::core::option::Option<::subxt::utils::AccountId32>,
				pub bouncer: ::core::option::Option<::subxt::utils::AccountId32>,
				pub nominator: ::core::option::Option<::subxt::utils::AccountId32>,
			}
			impl ::subxt::events::StaticEvent for RolesUpdated {
				const PALLET: &'static str = "NominationPools";
				const EVENT: &'static str = "RolesUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The active balance of pool `pool_id` has been slashed to `balance`."]
			pub struct PoolSlashed {
				pub pool_id: ::core::primitive::u32,
				pub balance: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for PoolSlashed {
				const PALLET: &'static str = "NominationPools";
				const EVENT: &'static str = "PoolSlashed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The unbond pool at `era` of pool `pool_id` has been slashed to `balance`."]
			pub struct UnbondingPoolSlashed {
				pub pool_id: ::core::primitive::u32,
				pub era: ::core::primitive::u32,
				pub balance: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for UnbondingPoolSlashed {
				const PALLET: &'static str = "NominationPools";
				const EVENT: &'static str = "UnbondingPoolSlashed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A pool's commission setting has been changed."]
			pub struct PoolCommissionUpdated {
				pub pool_id: ::core::primitive::u32,
				pub current: ::core::option::Option<(
					runtime_types::sp_arithmetic::per_things::Perbill,
					::subxt::utils::AccountId32,
				)>,
			}
			impl ::subxt::events::StaticEvent for PoolCommissionUpdated {
				const PALLET: &'static str = "NominationPools";
				const EVENT: &'static str = "PoolCommissionUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A pool's maximum commission setting has been changed."]
			pub struct PoolMaxCommissionUpdated {
				pub pool_id: ::core::primitive::u32,
				pub max_commission: runtime_types::sp_arithmetic::per_things::Perbill,
			}
			impl ::subxt::events::StaticEvent for PoolMaxCommissionUpdated {
				const PALLET: &'static str = "NominationPools";
				const EVENT: &'static str = "PoolMaxCommissionUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A pool's commission `change_rate` has been changed."]
			pub struct PoolCommissionChangeRateUpdated {
				pub pool_id: ::core::primitive::u32,
				pub change_rate: runtime_types::pallet_nomination_pools::CommissionChangeRate<
					::core::primitive::u32,
				>,
			}
			impl ::subxt::events::StaticEvent for PoolCommissionChangeRateUpdated {
				const PALLET: &'static str = "NominationPools";
				const EVENT: &'static str = "PoolCommissionChangeRateUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Pool commission has been claimed."]
			pub struct PoolCommissionClaimed {
				pub pool_id: ::core::primitive::u32,
				pub commission: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for PoolCommissionClaimed {
				const PALLET: &'static str = "NominationPools";
				const EVENT: &'static str = "PoolCommissionClaimed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Minimum amount to bond to join a pool."]
				pub fn min_join_bond(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"MinJoinBond",
						vec![],
						[
							64u8, 180u8, 71u8, 185u8, 81u8, 46u8, 155u8, 26u8, 251u8, 84u8, 108u8,
							80u8, 128u8, 44u8, 163u8, 118u8, 107u8, 79u8, 250u8, 211u8, 194u8,
							71u8, 87u8, 16u8, 247u8, 9u8, 76u8, 95u8, 103u8, 227u8, 180u8, 231u8,
						],
					)
				}
				#[doc = " Minimum bond required to create a pool."]
				#[doc = ""]
				#[doc = " This is the amount that the depositor must put as their initial stake in the pool, as an"]
				#[doc = " indication of \"skin in the game\"."]
				#[doc = ""]
				#[doc = " This is the value that will always exist in the staking ledger of the pool bonded account"]
				#[doc = " while all other accounts leave."]
				pub fn min_create_bond(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"MinCreateBond",
						vec![],
						[
							210u8, 67u8, 92u8, 230u8, 231u8, 105u8, 54u8, 249u8, 154u8, 192u8,
							29u8, 217u8, 233u8, 79u8, 170u8, 126u8, 133u8, 98u8, 253u8, 153u8,
							248u8, 189u8, 63u8, 107u8, 170u8, 224u8, 12u8, 42u8, 198u8, 185u8,
							85u8, 46u8,
						],
					)
				}
				#[doc = " Maximum number of nomination pools that can exist. If `None`, then an unbounded number of"]
				#[doc = " pools can exist."]
				pub fn max_pools(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"MaxPools",
						vec![],
						[
							230u8, 184u8, 242u8, 91u8, 118u8, 111u8, 90u8, 204u8, 136u8, 61u8,
							228u8, 50u8, 212u8, 40u8, 83u8, 49u8, 121u8, 161u8, 245u8, 80u8, 46u8,
							184u8, 105u8, 134u8, 249u8, 225u8, 39u8, 3u8, 123u8, 137u8, 156u8,
							240u8,
						],
					)
				}
				#[doc = " Maximum number of members that can exist in the system. If `None`, then the count"]
				#[doc = " members are not bound on a system wide basis."]
				pub fn max_pool_members(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"MaxPoolMembers",
						vec![],
						[
							210u8, 222u8, 181u8, 146u8, 137u8, 200u8, 71u8, 196u8, 74u8, 38u8,
							36u8, 122u8, 187u8, 164u8, 218u8, 116u8, 216u8, 143u8, 182u8, 15u8,
							23u8, 124u8, 57u8, 121u8, 81u8, 151u8, 8u8, 247u8, 80u8, 136u8, 115u8,
							2u8,
						],
					)
				}
				#[doc = " Maximum number of members that may belong to pool. If `None`, then the count of"]
				#[doc = " members is not bound on a per pool basis."]
				pub fn max_pool_members_per_pool(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"MaxPoolMembersPerPool",
						vec![],
						[
							250u8, 255u8, 136u8, 223u8, 61u8, 119u8, 117u8, 240u8, 68u8, 114u8,
							55u8, 1u8, 176u8, 120u8, 143u8, 48u8, 232u8, 125u8, 218u8, 105u8, 28u8,
							230u8, 253u8, 36u8, 9u8, 44u8, 129u8, 225u8, 147u8, 33u8, 181u8, 68u8,
						],
					)
				}
				#[doc = " The maximum commission that can be charged by a pool. Used on commission payouts to bound"]
				#[doc = " pool commissions that are > `GlobalMaxCommission`, necessary if a future"]
				#[doc = " `GlobalMaxCommission` is lower than some current pool commissions."]
				pub fn global_max_commission(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::per_things::Perbill,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"GlobalMaxCommission",
						vec![],
						[
							2u8, 112u8, 8u8, 116u8, 114u8, 97u8, 250u8, 106u8, 170u8, 215u8, 218u8,
							217u8, 80u8, 235u8, 149u8, 81u8, 85u8, 185u8, 201u8, 127u8, 107u8,
							251u8, 191u8, 231u8, 142u8, 74u8, 8u8, 70u8, 151u8, 238u8, 117u8,
							173u8,
						],
					)
				}
				#[doc = " Active members."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
				pub fn pool_members(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_nomination_pools::PoolMember,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"PoolMembers",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							116u8, 41u8, 89u8, 74u8, 35u8, 243u8, 213u8, 178u8, 41u8, 249u8, 62u8,
							119u8, 72u8, 34u8, 197u8, 168u8, 147u8, 178u8, 159u8, 10u8, 181u8,
							255u8, 40u8, 211u8, 206u8, 32u8, 130u8, 25u8, 201u8, 54u8, 212u8, 25u8,
						],
					)
				}
				#[doc = " Active members."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
				pub fn pool_members_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_nomination_pools::PoolMember,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"PoolMembers",
						Vec::new(),
						[
							116u8, 41u8, 89u8, 74u8, 35u8, 243u8, 213u8, 178u8, 41u8, 249u8, 62u8,
							119u8, 72u8, 34u8, 197u8, 168u8, 147u8, 178u8, 159u8, 10u8, 181u8,
							255u8, 40u8, 211u8, 206u8, 32u8, 130u8, 25u8, 201u8, 54u8, 212u8, 25u8,
						],
					)
				}
				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_pool_members(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"CounterForPoolMembers",
						vec![],
						[
							165u8, 158u8, 130u8, 19u8, 106u8, 227u8, 134u8, 73u8, 36u8, 237u8,
							103u8, 146u8, 198u8, 68u8, 219u8, 186u8, 134u8, 224u8, 89u8, 251u8,
							200u8, 46u8, 87u8, 232u8, 53u8, 152u8, 13u8, 10u8, 105u8, 49u8, 150u8,
							212u8,
						],
					)
				}
				#[doc = " Storage for bonded pools."]
				pub fn bonded_pools(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_nomination_pools::BondedPoolInner,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"BondedPools",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							171u8, 143u8, 96u8, 95u8, 196u8, 228u8, 116u8, 22u8, 63u8, 105u8,
							193u8, 77u8, 171u8, 99u8, 144u8, 70u8, 166u8, 55u8, 14u8, 191u8, 156u8,
							17u8, 237u8, 193u8, 228u8, 243u8, 164u8, 187u8, 127u8, 245u8, 117u8,
							238u8,
						],
					)
				}
				#[doc = " Storage for bonded pools."]
				pub fn bonded_pools_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_nomination_pools::BondedPoolInner,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"BondedPools",
						Vec::new(),
						[
							171u8, 143u8, 96u8, 95u8, 196u8, 228u8, 116u8, 22u8, 63u8, 105u8,
							193u8, 77u8, 171u8, 99u8, 144u8, 70u8, 166u8, 55u8, 14u8, 191u8, 156u8,
							17u8, 237u8, 193u8, 228u8, 243u8, 164u8, 187u8, 127u8, 245u8, 117u8,
							238u8,
						],
					)
				}
				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_bonded_pools(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"CounterForBondedPools",
						vec![],
						[
							198u8, 6u8, 213u8, 92u8, 4u8, 114u8, 164u8, 244u8, 51u8, 55u8, 157u8,
							20u8, 224u8, 183u8, 40u8, 236u8, 115u8, 86u8, 171u8, 207u8, 31u8,
							111u8, 0u8, 210u8, 48u8, 198u8, 243u8, 153u8, 5u8, 216u8, 107u8, 113u8,
						],
					)
				}
				#[doc = " Reward pools. This is where there rewards for each pool accumulate. When a members payout is"]
				#[doc = " claimed, the balance comes out fo the reward pool. Keyed by the bonded pools account."]
				pub fn reward_pools(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_nomination_pools::RewardPool,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"RewardPools",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							150u8, 53u8, 204u8, 26u8, 187u8, 118u8, 80u8, 133u8, 94u8, 127u8,
							155u8, 78u8, 71u8, 72u8, 0u8, 220u8, 174u8, 174u8, 109u8, 238u8, 13u8,
							120u8, 193u8, 102u8, 219u8, 22u8, 89u8, 117u8, 169u8, 212u8, 64u8,
							204u8,
						],
					)
				}
				#[doc = " Reward pools. This is where there rewards for each pool accumulate. When a members payout is"]
				#[doc = " claimed, the balance comes out fo the reward pool. Keyed by the bonded pools account."]
				pub fn reward_pools_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_nomination_pools::RewardPool,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"RewardPools",
						Vec::new(),
						[
							150u8, 53u8, 204u8, 26u8, 187u8, 118u8, 80u8, 133u8, 94u8, 127u8,
							155u8, 78u8, 71u8, 72u8, 0u8, 220u8, 174u8, 174u8, 109u8, 238u8, 13u8,
							120u8, 193u8, 102u8, 219u8, 22u8, 89u8, 117u8, 169u8, 212u8, 64u8,
							204u8,
						],
					)
				}
				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_reward_pools(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"CounterForRewardPools",
						vec![],
						[
							218u8, 186u8, 28u8, 97u8, 205u8, 249u8, 187u8, 10u8, 127u8, 190u8,
							213u8, 152u8, 103u8, 20u8, 157u8, 183u8, 86u8, 104u8, 186u8, 236u8,
							84u8, 159u8, 117u8, 78u8, 5u8, 242u8, 193u8, 59u8, 112u8, 200u8, 34u8,
							166u8,
						],
					)
				}
				#[doc = " Groups of unbonding pools. Each group of unbonding pools belongs to a"]
				#[doc = " bonded pool, hence the name sub-pools. Keyed by the bonded pools account."]
				pub fn sub_pools_storage(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_nomination_pools::SubPools,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"SubPoolsStorage",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							248u8, 37u8, 232u8, 231u8, 14u8, 140u8, 12u8, 27u8, 61u8, 222u8, 185u8,
							128u8, 158u8, 30u8, 57u8, 121u8, 35u8, 11u8, 42u8, 242u8, 56u8, 1u8,
							61u8, 0u8, 67u8, 140u8, 55u8, 62u8, 165u8, 134u8, 136u8, 4u8,
						],
					)
				}
				#[doc = " Groups of unbonding pools. Each group of unbonding pools belongs to a"]
				#[doc = " bonded pool, hence the name sub-pools. Keyed by the bonded pools account."]
				pub fn sub_pools_storage_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_nomination_pools::SubPools,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"SubPoolsStorage",
						Vec::new(),
						[
							248u8, 37u8, 232u8, 231u8, 14u8, 140u8, 12u8, 27u8, 61u8, 222u8, 185u8,
							128u8, 158u8, 30u8, 57u8, 121u8, 35u8, 11u8, 42u8, 242u8, 56u8, 1u8,
							61u8, 0u8, 67u8, 140u8, 55u8, 62u8, 165u8, 134u8, 136u8, 4u8,
						],
					)
				}
				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_sub_pools_storage(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"CounterForSubPoolsStorage",
						vec![],
						[
							137u8, 162u8, 32u8, 44u8, 163u8, 30u8, 54u8, 158u8, 169u8, 118u8,
							196u8, 101u8, 78u8, 28u8, 184u8, 78u8, 185u8, 225u8, 226u8, 207u8,
							14u8, 119u8, 0u8, 116u8, 140u8, 141u8, 116u8, 106u8, 71u8, 161u8,
							200u8, 228u8,
						],
					)
				}
				#[doc = " Metadata for the pool."]
				pub fn metadata(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"Metadata",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							10u8, 171u8, 251u8, 5u8, 72u8, 74u8, 86u8, 144u8, 59u8, 67u8, 92u8,
							111u8, 217u8, 111u8, 175u8, 107u8, 119u8, 206u8, 199u8, 78u8, 182u8,
							84u8, 12u8, 102u8, 10u8, 124u8, 103u8, 9u8, 86u8, 199u8, 233u8, 54u8,
						],
					)
				}
				#[doc = " Metadata for the pool."]
				pub fn metadata_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"Metadata",
						Vec::new(),
						[
							10u8, 171u8, 251u8, 5u8, 72u8, 74u8, 86u8, 144u8, 59u8, 67u8, 92u8,
							111u8, 217u8, 111u8, 175u8, 107u8, 119u8, 206u8, 199u8, 78u8, 182u8,
							84u8, 12u8, 102u8, 10u8, 124u8, 103u8, 9u8, 86u8, 199u8, 233u8, 54u8,
						],
					)
				}
				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_metadata(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"CounterForMetadata",
						vec![],
						[
							49u8, 76u8, 175u8, 236u8, 99u8, 120u8, 156u8, 116u8, 153u8, 173u8,
							10u8, 102u8, 194u8, 139u8, 25u8, 149u8, 109u8, 195u8, 150u8, 21u8,
							43u8, 24u8, 196u8, 180u8, 231u8, 101u8, 69u8, 98u8, 82u8, 159u8, 183u8,
							174u8,
						],
					)
				}
				#[doc = " Ever increasing number of all pools created so far."]
				pub fn last_pool_id(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"LastPoolId",
						vec![],
						[
							178u8, 198u8, 245u8, 157u8, 176u8, 45u8, 214u8, 86u8, 73u8, 154u8,
							217u8, 39u8, 191u8, 53u8, 233u8, 145u8, 57u8, 100u8, 31u8, 13u8, 202u8,
							122u8, 115u8, 16u8, 205u8, 69u8, 157u8, 250u8, 216u8, 180u8, 113u8,
							30u8,
						],
					)
				}
				#[doc = " A reverse lookup from the pool's account id to its id."]
				#[doc = ""]
				#[doc = " This is only used for slashing. In all other instances, the pool id is used, and the"]
				#[doc = " accounts are deterministically derived from it."]
				pub fn reverse_pool_id_lookup(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"ReversePoolIdLookup",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							76u8, 76u8, 150u8, 33u8, 64u8, 81u8, 90u8, 75u8, 212u8, 221u8, 59u8,
							83u8, 178u8, 45u8, 86u8, 206u8, 196u8, 221u8, 117u8, 94u8, 229u8,
							160u8, 52u8, 54u8, 11u8, 64u8, 0u8, 103u8, 85u8, 86u8, 5u8, 71u8,
						],
					)
				}
				#[doc = " A reverse lookup from the pool's account id to its id."]
				#[doc = ""]
				#[doc = " This is only used for slashing. In all other instances, the pool id is used, and the"]
				#[doc = " accounts are deterministically derived from it."]
				pub fn reverse_pool_id_lookup_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"ReversePoolIdLookup",
						Vec::new(),
						[
							76u8, 76u8, 150u8, 33u8, 64u8, 81u8, 90u8, 75u8, 212u8, 221u8, 59u8,
							83u8, 178u8, 45u8, 86u8, 206u8, 196u8, 221u8, 117u8, 94u8, 229u8,
							160u8, 52u8, 54u8, 11u8, 64u8, 0u8, 103u8, 85u8, 86u8, 5u8, 71u8,
						],
					)
				}
				#[doc = "Counter for the related counted storage map"]
				pub fn counter_for_reverse_pool_id_lookup(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"CounterForReversePoolIdLookup",
						vec![],
						[
							135u8, 72u8, 203u8, 197u8, 101u8, 135u8, 114u8, 202u8, 122u8, 231u8,
							128u8, 17u8, 81u8, 70u8, 22u8, 146u8, 100u8, 138u8, 16u8, 74u8, 31u8,
							250u8, 110u8, 184u8, 250u8, 75u8, 249u8, 71u8, 171u8, 77u8, 95u8,
							251u8,
						],
					)
				}
				#[doc = " Map from a pool member account to their opted claim permission."]
				pub fn claim_permissions(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_nomination_pools::ClaimPermission,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"ClaimPermissions",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							98u8, 241u8, 185u8, 102u8, 61u8, 53u8, 215u8, 105u8, 2u8, 148u8, 197u8,
							17u8, 107u8, 253u8, 74u8, 159u8, 14u8, 30u8, 213u8, 38u8, 35u8, 163u8,
							249u8, 19u8, 140u8, 201u8, 182u8, 106u8, 0u8, 21u8, 102u8, 15u8,
						],
					)
				}
				#[doc = " Map from a pool member account to their opted claim permission."]
				pub fn claim_permissions_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_nomination_pools::ClaimPermission,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"NominationPools",
						"ClaimPermissions",
						Vec::new(),
						[
							98u8, 241u8, 185u8, 102u8, 61u8, 53u8, 215u8, 105u8, 2u8, 148u8, 197u8,
							17u8, 107u8, 253u8, 74u8, 159u8, 14u8, 30u8, 213u8, 38u8, 35u8, 163u8,
							249u8, 19u8, 140u8, 201u8, 182u8, 106u8, 0u8, 21u8, 102u8, 15u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The nomination pool's pallet id."]
				pub fn pallet_id(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_support::PalletId> {
					::subxt::constants::Address::new_static(
						"NominationPools",
						"PalletId",
						[
							56u8, 243u8, 53u8, 83u8, 154u8, 179u8, 170u8, 80u8, 133u8, 173u8, 61u8,
							161u8, 47u8, 225u8, 146u8, 21u8, 50u8, 229u8, 248u8, 27u8, 104u8, 58u8,
							129u8, 197u8, 102u8, 160u8, 168u8, 205u8, 154u8, 42u8, 217u8, 53u8,
						],
					)
				}
				#[doc = " The maximum pool points-to-balance ratio that an `open` pool can have."]
				#[doc = ""]
				#[doc = " This is important in the event slashing takes place and the pool's points-to-balance"]
				#[doc = " ratio becomes disproportional."]
				#[doc = ""]
				#[doc = " Moreover, this relates to the `RewardCounter` type as well, as the arithmetic operations"]
				#[doc = " are a function of number of points, and by setting this value to e.g. 10, you ensure"]
				#[doc = " that the total number of points in the system are at most 10 times the total_issuance of"]
				#[doc = " the chain, in the absolute worse case."]
				#[doc = ""]
				#[doc = " For a value of 10, the threshold would be a pool points-to-balance ratio of 10:1."]
				#[doc = " Such a scenario would also be the equivalent of the pool being 90% slashed."]
				pub fn max_points_to_balance(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u8> {
					::subxt::constants::Address::new_static(
						"NominationPools",
						"MaxPointsToBalance",
						[
							141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
							28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
							114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
							165u8,
						],
					)
				}
			}
		}
	}
	pub mod identity {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_identity::pallet::Error;
		#[doc = "Identity pallet declaration."]
		pub type Call = runtime_types::pallet_identity::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AddRegistrar {
					pub account: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for AddRegistrar {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "add_registrar";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetIdentity {
					pub info:
						::std::boxed::Box<runtime_types::pallet_identity::types::IdentityInfo>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetIdentity {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "set_identity";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetSubs {
					pub subs: ::std::vec::Vec<(
						::subxt::utils::AccountId32,
						runtime_types::pallet_identity::types::Data,
					)>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetSubs {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "set_subs";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ClearIdentity;
				impl ::subxt::blocks::StaticExtrinsic for ClearIdentity {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "clear_identity";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RequestJudgement {
					#[codec(compact)]
					pub reg_index: ::core::primitive::u32,
					#[codec(compact)]
					pub max_fee: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for RequestJudgement {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "request_judgement";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CancelRequest {
					pub reg_index: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for CancelRequest {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "cancel_request";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetFee {
					#[codec(compact)]
					pub index: ::core::primitive::u32,
					#[codec(compact)]
					pub fee: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetFee {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "set_fee";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetAccountId {
					#[codec(compact)]
					pub index: ::core::primitive::u32,
					pub new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetAccountId {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "set_account_id";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetFields {
					#[codec(compact)]
					pub index: ::core::primitive::u32,
					pub fields: runtime_types::pallet_identity::types::BitFlags<
						runtime_types::pallet_identity::types::IdentityField,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetFields {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "set_fields";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ProvideJudgement {
					#[codec(compact)]
					pub reg_index: ::core::primitive::u32,
					pub target: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					pub judgement:
						runtime_types::pallet_identity::types::Judgement<::core::primitive::u128>,
					pub identity: ::subxt::utils::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for ProvideJudgement {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "provide_judgement";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct KillIdentity {
					pub target: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for KillIdentity {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "kill_identity";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AddSub {
					pub sub: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					pub data: runtime_types::pallet_identity::types::Data,
				}
				impl ::subxt::blocks::StaticExtrinsic for AddSub {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "add_sub";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RenameSub {
					pub sub: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					pub data: runtime_types::pallet_identity::types::Data,
				}
				impl ::subxt::blocks::StaticExtrinsic for RenameSub {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "rename_sub";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RemoveSub {
					pub sub: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemoveSub {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "remove_sub";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct QuitSub;
				impl ::subxt::blocks::StaticExtrinsic for QuitSub {
					const PALLET: &'static str = "Identity";
					const CALL: &'static str = "quit_sub";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::add_registrar`]."]
				pub fn add_registrar(
					&self,
					account: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::AddRegistrar> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"add_registrar",
						types::AddRegistrar { account },
						[
							34u8, 133u8, 250u8, 186u8, 71u8, 219u8, 66u8, 250u8, 41u8, 190u8,
							237u8, 96u8, 40u8, 51u8, 98u8, 54u8, 242u8, 65u8, 216u8, 135u8, 211u8,
							24u8, 57u8, 161u8, 229u8, 174u8, 48u8, 1u8, 88u8, 2u8, 38u8, 206u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_identity`]."]
				pub fn set_identity(
					&self,
					info: runtime_types::pallet_identity::types::IdentityInfo,
				) -> ::subxt::tx::Payload<types::SetIdentity> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"set_identity",
						types::SetIdentity {
							info: ::std::boxed::Box::new(info),
						},
						[
							205u8, 7u8, 54u8, 226u8, 123u8, 160u8, 173u8, 25u8, 179u8, 93u8, 172u8,
							37u8, 222u8, 143u8, 209u8, 1u8, 230u8, 32u8, 84u8, 80u8, 110u8, 195u8,
							87u8, 185u8, 27u8, 31u8, 185u8, 161u8, 154u8, 166u8, 177u8, 190u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_subs`]."]
				pub fn set_subs(
					&self,
					subs: ::std::vec::Vec<(
						::subxt::utils::AccountId32,
						runtime_types::pallet_identity::types::Data,
					)>,
				) -> ::subxt::tx::Payload<types::SetSubs> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"set_subs",
						types::SetSubs { subs },
						[
							76u8, 193u8, 92u8, 120u8, 9u8, 99u8, 102u8, 220u8, 177u8, 29u8, 65u8,
							14u8, 250u8, 101u8, 118u8, 59u8, 251u8, 153u8, 136u8, 141u8, 89u8,
							250u8, 74u8, 254u8, 111u8, 220u8, 132u8, 228u8, 248u8, 132u8, 177u8,
							128u8,
						],
					)
				}
				#[doc = "See [`Pallet::clear_identity`]."]
				pub fn clear_identity(&self) -> ::subxt::tx::Payload<types::ClearIdentity> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"clear_identity",
						types::ClearIdentity {},
						[
							43u8, 115u8, 205u8, 44u8, 24u8, 130u8, 220u8, 69u8, 247u8, 176u8,
							200u8, 175u8, 67u8, 183u8, 36u8, 200u8, 162u8, 132u8, 242u8, 25u8,
							21u8, 106u8, 197u8, 219u8, 141u8, 51u8, 204u8, 13u8, 191u8, 201u8,
							31u8, 31u8,
						],
					)
				}
				#[doc = "See [`Pallet::request_judgement`]."]
				pub fn request_judgement(
					&self,
					reg_index: ::core::primitive::u32,
					max_fee: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::RequestJudgement> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"request_judgement",
						types::RequestJudgement { reg_index, max_fee },
						[
							83u8, 85u8, 55u8, 184u8, 14u8, 54u8, 49u8, 212u8, 26u8, 148u8, 33u8,
							147u8, 182u8, 54u8, 180u8, 12u8, 61u8, 179u8, 216u8, 157u8, 103u8,
							52u8, 120u8, 252u8, 83u8, 203u8, 144u8, 65u8, 15u8, 3u8, 21u8, 33u8,
						],
					)
				}
				#[doc = "See [`Pallet::cancel_request`]."]
				pub fn cancel_request(
					&self,
					reg_index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::CancelRequest> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"cancel_request",
						types::CancelRequest { reg_index },
						[
							81u8, 14u8, 133u8, 219u8, 43u8, 84u8, 163u8, 208u8, 21u8, 185u8, 75u8,
							117u8, 126u8, 33u8, 210u8, 106u8, 122u8, 210u8, 35u8, 207u8, 104u8,
							206u8, 41u8, 117u8, 247u8, 108u8, 56u8, 23u8, 123u8, 169u8, 169u8,
							61u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_fee`]."]
				pub fn set_fee(
					&self,
					index: ::core::primitive::u32,
					fee: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::SetFee> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"set_fee",
						types::SetFee { index, fee },
						[
							131u8, 20u8, 17u8, 127u8, 180u8, 65u8, 225u8, 144u8, 193u8, 60u8,
							131u8, 241u8, 30u8, 149u8, 8u8, 76u8, 29u8, 52u8, 102u8, 108u8, 127u8,
							130u8, 70u8, 18u8, 94u8, 145u8, 179u8, 109u8, 252u8, 219u8, 58u8,
							163u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_account_id`]."]
				pub fn set_account_id(
					&self,
					index: ::core::primitive::u32,
					new: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::SetAccountId> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"set_account_id",
						types::SetAccountId { index, new },
						[
							127u8, 225u8, 228u8, 150u8, 6u8, 49u8, 67u8, 109u8, 19u8, 172u8, 100u8,
							239u8, 81u8, 65u8, 5u8, 126u8, 239u8, 5u8, 203u8, 160u8, 241u8, 250u8,
							243u8, 218u8, 208u8, 227u8, 239u8, 124u8, 78u8, 166u8, 40u8, 156u8,
						],
					)
				}
				#[doc = "See [`Pallet::set_fields`]."]
				pub fn set_fields(
					&self,
					index: ::core::primitive::u32,
					fields: runtime_types::pallet_identity::types::BitFlags<
						runtime_types::pallet_identity::types::IdentityField,
					>,
				) -> ::subxt::tx::Payload<types::SetFields> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"set_fields",
						types::SetFields { index, fields },
						[
							25u8, 129u8, 119u8, 232u8, 18u8, 32u8, 77u8, 23u8, 185u8, 56u8, 32u8,
							199u8, 74u8, 174u8, 104u8, 203u8, 171u8, 253u8, 19u8, 225u8, 101u8,
							239u8, 14u8, 242u8, 157u8, 51u8, 203u8, 74u8, 1u8, 65u8, 165u8, 205u8,
						],
					)
				}
				#[doc = "See [`Pallet::provide_judgement`]."]
				pub fn provide_judgement(
					&self,
					reg_index: ::core::primitive::u32,
					target: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					judgement: runtime_types::pallet_identity::types::Judgement<
						::core::primitive::u128,
					>,
					identity: ::subxt::utils::H256,
				) -> ::subxt::tx::Payload<types::ProvideJudgement> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"provide_judgement",
						types::ProvideJudgement {
							reg_index,
							target,
							judgement,
							identity,
						},
						[
							51u8, 75u8, 24u8, 146u8, 141u8, 86u8, 141u8, 188u8, 77u8, 71u8, 5u8,
							76u8, 44u8, 35u8, 208u8, 98u8, 217u8, 80u8, 248u8, 201u8, 225u8, 230u8,
							162u8, 20u8, 202u8, 242u8, 4u8, 209u8, 9u8, 200u8, 178u8, 5u8,
						],
					)
				}
				#[doc = "See [`Pallet::kill_identity`]."]
				pub fn kill_identity(
					&self,
					target: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::KillIdentity> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"kill_identity",
						types::KillIdentity { target },
						[
							111u8, 40u8, 34u8, 131u8, 14u8, 102u8, 209u8, 175u8, 202u8, 50u8, 12u8,
							19u8, 37u8, 230u8, 120u8, 91u8, 248u8, 67u8, 25u8, 245u8, 228u8, 19u8,
							57u8, 187u8, 244u8, 30u8, 59u8, 178u8, 29u8, 114u8, 197u8, 24u8,
						],
					)
				}
				#[doc = "See [`Pallet::add_sub`]."]
				pub fn add_sub(
					&self,
					sub: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					data: runtime_types::pallet_identity::types::Data,
				) -> ::subxt::tx::Payload<types::AddSub> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"add_sub",
						types::AddSub { sub, data },
						[
							63u8, 142u8, 248u8, 45u8, 29u8, 195u8, 238u8, 249u8, 23u8, 242u8, 54u8,
							254u8, 141u8, 12u8, 202u8, 156u8, 52u8, 180u8, 121u8, 168u8, 198u8,
							5u8, 44u8, 129u8, 117u8, 178u8, 201u8, 125u8, 209u8, 211u8, 86u8,
							140u8,
						],
					)
				}
				#[doc = "See [`Pallet::rename_sub`]."]
				pub fn rename_sub(
					&self,
					sub: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
					data: runtime_types::pallet_identity::types::Data,
				) -> ::subxt::tx::Payload<types::RenameSub> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"rename_sub",
						types::RenameSub { sub, data },
						[
							201u8, 113u8, 118u8, 210u8, 110u8, 248u8, 42u8, 56u8, 35u8, 208u8,
							102u8, 171u8, 128u8, 158u8, 105u8, 232u8, 254u8, 73u8, 44u8, 84u8,
							16u8, 32u8, 112u8, 41u8, 19u8, 128u8, 200u8, 118u8, 95u8, 115u8, 220u8,
							100u8,
						],
					)
				}
				#[doc = "See [`Pallet::remove_sub`]."]
				pub fn remove_sub(
					&self,
					sub: ::subxt::utils::MultiAddress<
						::subxt::utils::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::tx::Payload<types::RemoveSub> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"remove_sub",
						types::RemoveSub { sub },
						[
							20u8, 227u8, 205u8, 59u8, 130u8, 76u8, 62u8, 11u8, 69u8, 169u8, 10u8,
							199u8, 9u8, 194u8, 142u8, 181u8, 170u8, 170u8, 254u8, 197u8, 74u8,
							123u8, 216u8, 87u8, 163u8, 156u8, 153u8, 41u8, 20u8, 173u8, 87u8, 10u8,
						],
					)
				}
				#[doc = "See [`Pallet::quit_sub`]."]
				pub fn quit_sub(&self) -> ::subxt::tx::Payload<types::QuitSub> {
					::subxt::tx::Payload::new_static(
						"Identity",
						"quit_sub",
						types::QuitSub {},
						[
							147u8, 131u8, 175u8, 171u8, 187u8, 201u8, 240u8, 26u8, 146u8, 224u8,
							74u8, 166u8, 242u8, 193u8, 204u8, 247u8, 168u8, 93u8, 18u8, 32u8, 27u8,
							208u8, 149u8, 146u8, 179u8, 172u8, 75u8, 112u8, 84u8, 141u8, 233u8,
							223u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_identity::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A name was set or reset (which will remove all judgements)."]
			pub struct IdentitySet {
				pub who: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for IdentitySet {
				const PALLET: &'static str = "Identity";
				const EVENT: &'static str = "IdentitySet";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A name was cleared, and the given balance returned."]
			pub struct IdentityCleared {
				pub who: ::subxt::utils::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for IdentityCleared {
				const PALLET: &'static str = "Identity";
				const EVENT: &'static str = "IdentityCleared";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A name was removed and the given balance slashed."]
			pub struct IdentityKilled {
				pub who: ::subxt::utils::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for IdentityKilled {
				const PALLET: &'static str = "Identity";
				const EVENT: &'static str = "IdentityKilled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A judgement was asked from a registrar."]
			pub struct JudgementRequested {
				pub who: ::subxt::utils::AccountId32,
				pub registrar_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for JudgementRequested {
				const PALLET: &'static str = "Identity";
				const EVENT: &'static str = "JudgementRequested";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A judgement request was retracted."]
			pub struct JudgementUnrequested {
				pub who: ::subxt::utils::AccountId32,
				pub registrar_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for JudgementUnrequested {
				const PALLET: &'static str = "Identity";
				const EVENT: &'static str = "JudgementUnrequested";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A judgement was given by a registrar."]
			pub struct JudgementGiven {
				pub target: ::subxt::utils::AccountId32,
				pub registrar_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for JudgementGiven {
				const PALLET: &'static str = "Identity";
				const EVENT: &'static str = "JudgementGiven";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A registrar was added."]
			pub struct RegistrarAdded {
				pub registrar_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for RegistrarAdded {
				const PALLET: &'static str = "Identity";
				const EVENT: &'static str = "RegistrarAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A sub-identity was added to an identity and the deposit paid."]
			pub struct SubIdentityAdded {
				pub sub: ::subxt::utils::AccountId32,
				pub main: ::subxt::utils::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for SubIdentityAdded {
				const PALLET: &'static str = "Identity";
				const EVENT: &'static str = "SubIdentityAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A sub-identity was removed from an identity and the deposit freed."]
			pub struct SubIdentityRemoved {
				pub sub: ::subxt::utils::AccountId32,
				pub main: ::subxt::utils::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for SubIdentityRemoved {
				const PALLET: &'static str = "Identity";
				const EVENT: &'static str = "SubIdentityRemoved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A sub-identity was cleared, and the given deposit repatriated from the"]
			#[doc = "main identity account to the sub-identity account."]
			pub struct SubIdentityRevoked {
				pub sub: ::subxt::utils::AccountId32,
				pub main: ::subxt::utils::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for SubIdentityRevoked {
				const PALLET: &'static str = "Identity";
				const EVENT: &'static str = "SubIdentityRevoked";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Information that is pertinent to identify the entity behind an account."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: OK ― `AccountId` is a secure hash."]
				pub fn identity_of(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_identity::types::Registration<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Identity",
						"IdentityOf",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							239u8, 55u8, 5u8, 97u8, 227u8, 243u8, 118u8, 13u8, 98u8, 30u8, 141u8,
							84u8, 170u8, 90u8, 166u8, 116u8, 17u8, 122u8, 190u8, 76u8, 34u8, 51u8,
							239u8, 41u8, 14u8, 135u8, 11u8, 164u8, 106u8, 228u8, 48u8, 26u8,
						],
					)
				}
				#[doc = " Information that is pertinent to identify the entity behind an account."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: OK ― `AccountId` is a secure hash."]
				pub fn identity_of_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_identity::types::Registration<::core::primitive::u128>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Identity",
						"IdentityOf",
						Vec::new(),
						[
							239u8, 55u8, 5u8, 97u8, 227u8, 243u8, 118u8, 13u8, 98u8, 30u8, 141u8,
							84u8, 170u8, 90u8, 166u8, 116u8, 17u8, 122u8, 190u8, 76u8, 34u8, 51u8,
							239u8, 41u8, 14u8, 135u8, 11u8, 164u8, 106u8, 228u8, 48u8, 26u8,
						],
					)
				}
				#[doc = " The super-identity of an alternative \"sub\" identity together with its name, within that"]
				#[doc = " context. If the account is not some other account's sub-identity, then just `None`."]
				pub fn super_of(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						::subxt::utils::AccountId32,
						runtime_types::pallet_identity::types::Data,
					),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Identity",
						"SuperOf",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							51u8, 225u8, 21u8, 92u8, 85u8, 14u8, 14u8, 211u8, 61u8, 99u8, 176u8,
							236u8, 212u8, 156u8, 103u8, 175u8, 208u8, 105u8, 94u8, 226u8, 136u8,
							69u8, 162u8, 170u8, 11u8, 116u8, 72u8, 242u8, 119u8, 14u8, 14u8, 142u8,
						],
					)
				}
				#[doc = " The super-identity of an alternative \"sub\" identity together with its name, within that"]
				#[doc = " context. If the account is not some other account's sub-identity, then just `None`."]
				pub fn super_of_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						::subxt::utils::AccountId32,
						runtime_types::pallet_identity::types::Data,
					),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Identity",
						"SuperOf",
						Vec::new(),
						[
							51u8, 225u8, 21u8, 92u8, 85u8, 14u8, 14u8, 211u8, 61u8, 99u8, 176u8,
							236u8, 212u8, 156u8, 103u8, 175u8, 208u8, 105u8, 94u8, 226u8, 136u8,
							69u8, 162u8, 170u8, 11u8, 116u8, 72u8, 242u8, 119u8, 14u8, 14u8, 142u8,
						],
					)
				}
				#[doc = " Alternative \"sub\" identities of this account."]
				#[doc = ""]
				#[doc = " The first item is the deposit, the second is a vector of the accounts."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: OK ― `AccountId` is a secure hash."]
				pub fn subs_of(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						::core::primitive::u128,
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::subxt::utils::AccountId32,
						>,
					),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Identity",
						"SubsOf",
						vec![::subxt::storage::address::make_static_storage_map_key(
							_0.borrow(),
						)],
						[
							93u8, 124u8, 154u8, 157u8, 159u8, 103u8, 233u8, 225u8, 59u8, 20u8,
							201u8, 239u8, 128u8, 209u8, 207u8, 38u8, 123u8, 48u8, 119u8, 102u8,
							88u8, 42u8, 245u8, 187u8, 244u8, 206u8, 124u8, 216u8, 185u8, 155u8,
							207u8, 0u8,
						],
					)
				}
				#[doc = " Alternative \"sub\" identities of this account."]
				#[doc = ""]
				#[doc = " The first item is the deposit, the second is a vector of the accounts."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: OK ― `AccountId` is a secure hash."]
				pub fn subs_of_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						::core::primitive::u128,
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::subxt::utils::AccountId32,
						>,
					),
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Identity",
						"SubsOf",
						Vec::new(),
						[
							93u8, 124u8, 154u8, 157u8, 159u8, 103u8, 233u8, 225u8, 59u8, 20u8,
							201u8, 239u8, 128u8, 209u8, 207u8, 38u8, 123u8, 48u8, 119u8, 102u8,
							88u8, 42u8, 245u8, 187u8, 244u8, 206u8, 124u8, 216u8, 185u8, 155u8,
							207u8, 0u8,
						],
					)
				}
				#[doc = " The set of registrars. Not expected to get very big as can only be added through a"]
				#[doc = " special origin (likely a council motion)."]
				#[doc = ""]
				#[doc = " The index into this can be cast to `RegistrarIndex` to get a valid value."]
				pub fn registrars(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::option::Option<
							runtime_types::pallet_identity::types::RegistrarInfo<
								::core::primitive::u128,
								::subxt::utils::AccountId32,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Identity",
						"Registrars",
						vec![],
						[
							207u8, 253u8, 229u8, 237u8, 228u8, 85u8, 173u8, 74u8, 164u8, 67u8,
							144u8, 144u8, 5u8, 242u8, 84u8, 187u8, 110u8, 181u8, 2u8, 162u8, 239u8,
							212u8, 72u8, 233u8, 160u8, 196u8, 121u8, 218u8, 100u8, 0u8, 219u8,
							181u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The amount held on deposit for a registered identity"]
				pub fn basic_deposit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Identity",
						"BasicDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The amount held on deposit per additional field for a registered identity."]
				pub fn field_deposit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Identity",
						"FieldDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The amount held on deposit for a registered subaccount. This should account for the fact"]
				#[doc = " that one storage item's value will increase by the size of an account ID, and there will"]
				#[doc = " be another trie item whose value is the size of an account ID plus 32 bytes."]
				pub fn sub_account_deposit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Identity",
						"SubAccountDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The maximum number of sub-accounts allowed per identified account."]
				pub fn max_sub_accounts(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Identity",
						"MaxSubAccounts",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Maximum number of additional fields that may be stored in an ID. Needed to bound the I/O"]
				#[doc = " required to access an identity, but can be pretty high."]
				pub fn max_additional_fields(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Identity",
						"MaxAdditionalFields",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Maxmimum number of registrars allowed in the system. Needed to bound the complexity"]
				#[doc = " of, e.g., updating judgements."]
				pub fn max_registrars(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Identity",
						"MaxRegistrars",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod mandate {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_mandate::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Mandate {
					pub call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Mandate {
					const PALLET: &'static str = "Mandate";
					const CALL: &'static str = "mandate";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "See [`Pallet::mandate`]."]
				pub fn mandate(
					&self,
					call: runtime_types::da_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::Mandate> {
					::subxt::tx::Payload::new_static(
						"Mandate",
						"mandate",
						types::Mandate {
							call: ::std::boxed::Box::new(call),
						},
						[
							101u8, 107u8, 216u8, 70u8, 143u8, 185u8, 154u8, 128u8, 127u8, 61u8,
							82u8, 133u8, 217u8, 231u8, 97u8, 86u8, 197u8, 246u8, 136u8, 216u8,
							194u8, 241u8, 82u8, 99u8, 154u8, 205u8, 57u8, 21u8, 153u8, 217u8,
							185u8, 152u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_mandate::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A root operation was executed, show result"]
			pub struct RootOp {
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for RootOp {
				const PALLET: &'static str = "Mandate";
				const EVENT: &'static str = "RootOp";
			}
		}
	}
	pub mod runtime_types {
		use super::runtime_types;
		pub mod avail_core {
			use super::runtime_types;
			pub mod asdr {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AppUncheckedExtrinsic<_0, _1, _2, _3>(
					pub ::std::vec::Vec<::core::primitive::u8>,
					#[codec(skip)] pub ::core::marker::PhantomData<(_1, _0, _2, _3)>,
				);
			}
			pub mod data_lookup {
				use super::runtime_types;
				pub mod compact {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Default,
						Eq,
						PartialEq,
						serde :: Deserialize,
						serde :: Serialize,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CompactDataLookup {
						#[codec(compact)]
						pub size: ::core::primitive::u32,
						pub index: ::std::vec::Vec<
							runtime_types::avail_core::data_lookup::compact::DataLookupItem,
						>,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
						serde :: Deserialize,
						serde :: Serialize,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					#[serde(rename_all = "camelCase")]
					pub struct DataLookupItem {
						pub app_id: runtime_types::avail_core::AppId,
						#[codec(compact)]
						pub start: ::core::primitive::u32,
					}
				}
			}
			pub mod header {
				use super::runtime_types;
				pub mod extension {
					use super::runtime_types;
					pub mod v1 {
						use super::runtime_types;
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							:: subxt :: ext :: scale_decode :: DecodeAsType,
							:: subxt :: ext :: scale_encode :: EncodeAsType,
							Clone,
							Debug,
							Default,
							Eq,
							PartialEq,
							serde :: Deserialize,
							serde :: Serialize,
						)]
						# [codec (crate = :: subxt :: ext :: codec)]
						#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
						#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
						#[serde(rename_all = "camelCase")]
						pub struct HeaderExtension {
							pub commitment:
								runtime_types::avail_core::kate_commitment::v1::KateCommitment,
							pub app_lookup:
								runtime_types::avail_core::data_lookup::compact::CompactDataLookup,
						}
					}
					pub mod v2 {
						use super::runtime_types;
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							:: subxt :: ext :: scale_decode :: DecodeAsType,
							:: subxt :: ext :: scale_encode :: EncodeAsType,
							Clone,
							Debug,
							Default,
							Eq,
							PartialEq,
							serde :: Deserialize,
							serde :: Serialize,
						)]
						# [codec (crate = :: subxt :: ext :: codec)]
						#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
						#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
						pub struct HeaderExtension {
							pub commitment:
								runtime_types::avail_core::kate_commitment::v2::KateCommitment,
							pub app_lookup:
								runtime_types::avail_core::data_lookup::compact::CompactDataLookup,
						}
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
						serde :: Deserialize,
						serde :: Serialize,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum HeaderExtension {
						#[codec(index = 0)]
						V1(runtime_types::avail_core::header::extension::v1::HeaderExtension),
						#[codec(index = 1)]
						V2(runtime_types::avail_core::header::extension::v2::HeaderExtension),
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Header<_0, _1> {
					pub parent_hash: ::subxt::utils::H256,
					#[codec(compact)]
					pub number: _0,
					pub state_root: ::subxt::utils::H256,
					pub extrinsics_root: ::subxt::utils::H256,
					pub digest: runtime_types::sp_runtime::generic::digest::Digest,
					pub extension: runtime_types::avail_core::header::extension::HeaderExtension,
					#[codec(skip)]
					pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
				}
			}
			pub mod kate_commitment {
				use super::runtime_types;
				pub mod v1 {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Default,
						Eq,
						PartialEq,
						serde :: Deserialize,
						serde :: Serialize,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					#[serde(rename_all = "camelCase")]
					pub struct KateCommitment {
						#[codec(compact)]
						pub rows: ::core::primitive::u16,
						#[codec(compact)]
						pub cols: ::core::primitive::u16,
						pub data_root: ::subxt::utils::H256,
						pub commitment: ::std::vec::Vec<::core::primitive::u8>,
					}
				}
				pub mod v2 {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Default,
						Eq,
						PartialEq,
						serde :: Deserialize,
						serde :: Serialize,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct KateCommitment {
						#[codec(compact)]
						pub rows: ::core::primitive::u16,
						#[codec(compact)]
						pub cols: ::core::primitive::u16,
						pub data_root: ::core::option::Option<::subxt::utils::H256>,
						pub commitment: ::std::vec::Vec<::core::primitive::u8>,
					}
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Copy,
				Debug,
				Default,
				Eq,
				PartialEq,
				derive_more :: From,
				serde :: Deserialize,
				serde :: Serialize,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AppId(#[codec(compact)] pub ::core::primitive::u32);
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BlockLengthColumns(#[codec(compact)] pub ::core::primitive::u32);
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BlockLengthRows(#[codec(compact)] pub ::core::primitive::u32);
		}
		pub mod bounded_collections {
			use super::runtime_types;
			pub mod bounded_btree_map {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BoundedBTreeMap<_0, _1>(pub ::subxt::utils::KeyedVec<_0, _1>);
			}
			pub mod bounded_vec {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
			}
			pub mod weak_bounded_vec {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
			}
		}
		pub mod da_control {
			use super::runtime_types;
			pub mod extensions {
				use super::runtime_types;
				pub mod check_app_id {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckAppId(pub runtime_types::avail_core::AppId);
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AppKeyInfo<_0> {
					pub owner: _0,
					pub id: runtime_types::avail_core::AppId,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::create_application_key`]."]
					create_application_key {
						key: runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::submit_data`]."]
					submit_data {
						data: runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::submit_block_length_proposal`]."]
					submit_block_length_proposal {
						rows: ::core::primitive::u32,
						cols: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Error for the System pallet"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The application key already exists."]
					AppKeyAlreadyExists,
					#[codec(index = 1)]
					#[doc = "The application key is an empty string."]
					AppKeyCannotBeEmpty,
					#[codec(index = 2)]
					#[doc = "The last application ID overflowed."]
					LastAppIdOverflowed,
					#[codec(index = 3)]
					#[doc = "The submitted data is empty."]
					DataCannotBeEmpty,
					#[codec(index = 4)]
					#[doc = "The last block length proposal Id overflowed."]
					LastBlockLenProposalIdOverflowed,
					#[codec(index = 5)]
					#[doc = "The proposed block dimensions are out of bounds."]
					BlockDimensionsOutOfBounds,
					#[codec(index = 6)]
					#[doc = "The proposed block dimensions are too small."]
					BlockDimensionsTooSmall,
					#[codec(index = 7)]
					#[doc = "The request to reduce block dimensions was made in a non-empty block"]
					InvalidBlockWeightReduction,
					#[codec(index = 8)]
					#[doc = "Submit data call outside of block execution context."]
					BadContext,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Event for the pallet."]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A new application key was created."]
					ApplicationKeyCreated {
						key: runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
						owner: ::subxt::utils::AccountId32,
						id: runtime_types::avail_core::AppId,
					},
					#[codec(index = 1)]
					DataSubmitted {
						who: ::subxt::utils::AccountId32,
						data_hash: ::subxt::utils::H256,
					},
					#[codec(index = 2)]
					BlockLengthProposalSubmitted {
						rows: runtime_types::avail_core::BlockLengthRows,
						cols: runtime_types::avail_core::BlockLengthColumns,
					},
				}
			}
		}
		pub mod da_runtime {
			use super::runtime_types;
			pub mod constants {
				use super::runtime_types;
				pub mod staking {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct NposSolution16 {
						pub votes1: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes2: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							),
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes3: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 2usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes4: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 3usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes5: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 4usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes6: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 5usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes7: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 6usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes8: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 7usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes9: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 8usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes10: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 9usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes11: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 10usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes12: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 11usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes13: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 12usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes14: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 13usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes15: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 14usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
						pub votes16: ::std::vec::Vec<(
							::subxt::ext::codec::Compact<::core::primitive::u32>,
							[(
								::subxt::ext::codec::Compact<::core::primitive::u16>,
								::subxt::ext::codec::Compact<
									runtime_types::sp_arithmetic::per_things::PerU16,
								>,
							); 15usize],
							::subxt::ext::codec::Compact<::core::primitive::u16>,
						)>,
					}
				}
			}
			pub mod primitives {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SessionKeys {
					pub babe: runtime_types::sp_consensus_babe::app::Public,
					pub grandpa: runtime_types::sp_consensus_grandpa::app::Public,
					pub im_online: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
					pub authority_discovery: runtime_types::sp_authority_discovery::app::Public,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum OriginCaller {
				#[codec(index = 0)]
				system(
					runtime_types::frame_support::dispatch::RawOrigin<::subxt::utils::AccountId32>,
				),
				#[codec(index = 14)]
				TechnicalCommittee(
					runtime_types::pallet_collective::RawOrigin<::subxt::utils::AccountId32>,
				),
				#[codec(index = 2)]
				Void(runtime_types::sp_core::Void),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Runtime;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RuntimeCall {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Call),
				#[codec(index = 1)]
				Utility(runtime_types::pallet_utility::pallet::Call),
				#[codec(index = 2)]
				Babe(runtime_types::pallet_babe::pallet::Call),
				#[codec(index = 3)]
				Timestamp(runtime_types::pallet_timestamp::pallet::Call),
				#[codec(index = 5)]
				Indices(runtime_types::pallet_indices::pallet::Call),
				#[codec(index = 6)]
				Balances(runtime_types::pallet_balances::pallet::Call),
				#[codec(index = 9)]
				ElectionProviderMultiPhase(
					runtime_types::pallet_election_provider_multi_phase::pallet::Call,
				),
				#[codec(index = 10)]
				Staking(runtime_types::pallet_staking::pallet::pallet::Call),
				#[codec(index = 11)]
				Session(runtime_types::pallet_session::pallet::Call),
				#[codec(index = 14)]
				TechnicalCommittee(runtime_types::pallet_collective::pallet::Call),
				#[codec(index = 16)]
				TechnicalMembership(runtime_types::pallet_membership::pallet::Call),
				#[codec(index = 17)]
				Grandpa(runtime_types::pallet_grandpa::pallet::Call),
				#[codec(index = 18)]
				Treasury(runtime_types::pallet_treasury::pallet::Call),
				#[codec(index = 19)]
				Sudo(runtime_types::pallet_sudo::pallet::Call),
				#[codec(index = 20)]
				ImOnline(runtime_types::pallet_im_online::pallet::Call),
				#[codec(index = 24)]
				Scheduler(runtime_types::pallet_scheduler::pallet::Call),
				#[codec(index = 25)]
				Bounties(runtime_types::pallet_bounties::pallet::Call),
				#[codec(index = 26)]
				Tips(runtime_types::pallet_tips::pallet::Call),
				#[codec(index = 29)]
				DataAvailability(runtime_types::da_control::pallet::Call),
				#[codec(index = 30)]
				NomadUpdaterManager(runtime_types::nomad_updater_manager::pallet::Call),
				#[codec(index = 31)]
				NomadHome(runtime_types::nomad_home::pallet::Call),
				#[codec(index = 32)]
				NomadDABridge(runtime_types::nomad_da_bridge::pallet::Call),
				#[codec(index = 33)]
				Preimage(runtime_types::pallet_preimage::pallet::Call),
				#[codec(index = 34)]
				Multisig(runtime_types::pallet_multisig::pallet::Call),
				#[codec(index = 35)]
				VoterList(runtime_types::pallet_bags_list::pallet::Call),
				#[codec(index = 36)]
				NominationPools(runtime_types::pallet_nomination_pools::pallet::Call),
				#[codec(index = 37)]
				Identity(runtime_types::pallet_identity::pallet::Call),
				#[codec(index = 38)]
				Mandate(runtime_types::pallet_mandate::pallet::Call),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RuntimeEvent {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Event),
				#[codec(index = 1)]
				Utility(runtime_types::pallet_utility::pallet::Event),
				#[codec(index = 5)]
				Indices(runtime_types::pallet_indices::pallet::Event),
				#[codec(index = 6)]
				Balances(runtime_types::pallet_balances::pallet::Event),
				#[codec(index = 7)]
				TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
				#[codec(index = 9)]
				ElectionProviderMultiPhase(
					runtime_types::pallet_election_provider_multi_phase::pallet::Event,
				),
				#[codec(index = 10)]
				Staking(runtime_types::pallet_staking::pallet::pallet::Event),
				#[codec(index = 11)]
				Session(runtime_types::pallet_session::pallet::Event),
				#[codec(index = 14)]
				TechnicalCommittee(runtime_types::pallet_collective::pallet::Event),
				#[codec(index = 16)]
				TechnicalMembership(runtime_types::pallet_membership::pallet::Event),
				#[codec(index = 17)]
				Grandpa(runtime_types::pallet_grandpa::pallet::Event),
				#[codec(index = 18)]
				Treasury(runtime_types::pallet_treasury::pallet::Event),
				#[codec(index = 19)]
				Sudo(runtime_types::pallet_sudo::pallet::Event),
				#[codec(index = 20)]
				ImOnline(runtime_types::pallet_im_online::pallet::Event),
				#[codec(index = 22)]
				Offences(runtime_types::pallet_offences::pallet::Event),
				#[codec(index = 24)]
				Scheduler(runtime_types::pallet_scheduler::pallet::Event),
				#[codec(index = 25)]
				Bounties(runtime_types::pallet_bounties::pallet::Event),
				#[codec(index = 26)]
				Tips(runtime_types::pallet_tips::pallet::Event),
				#[codec(index = 29)]
				DataAvailability(runtime_types::da_control::pallet::Event),
				#[codec(index = 30)]
				NomadUpdaterManager(runtime_types::nomad_updater_manager::pallet::Event),
				#[codec(index = 31)]
				NomadHome(runtime_types::nomad_home::pallet::Event),
				#[codec(index = 32)]
				NomadDABridge(runtime_types::nomad_da_bridge::pallet::Event),
				#[codec(index = 33)]
				Preimage(runtime_types::pallet_preimage::pallet::Event),
				#[codec(index = 34)]
				Multisig(runtime_types::pallet_multisig::pallet::Event),
				#[codec(index = 35)]
				VoterList(runtime_types::pallet_bags_list::pallet::Event),
				#[codec(index = 36)]
				NominationPools(runtime_types::pallet_nomination_pools::pallet::Event),
				#[codec(index = 37)]
				Identity(runtime_types::pallet_identity::pallet::Event),
				#[codec(index = 38)]
				Mandate(runtime_types::pallet_mandate::pallet::Event),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RuntimeHoldReason {}
		}
		pub mod finality_grandpa {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Equivocation<_0, _1, _2> {
				pub round_number: ::core::primitive::u64,
				pub identity: _0,
				pub first: (_1, _2),
				pub second: (_1, _2),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Precommit<_0, _1> {
				pub target_hash: _0,
				pub target_number: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Prevote<_0, _1> {
				pub target_hash: _0,
				pub target_number: _1,
			}
		}
		pub mod frame_support {
			use super::runtime_types;
			pub mod dispatch {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum DispatchClass {
					#[codec(index = 0)]
					Normal,
					#[codec(index = 1)]
					Operational,
					#[codec(index = 2)]
					Mandatory,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct DispatchInfo {
					pub weight: runtime_types::sp_weights::weight_v2::Weight,
					pub class: runtime_types::frame_support::dispatch::DispatchClass,
					pub pays_fee: runtime_types::frame_support::dispatch::Pays,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Pays {
					#[codec(index = 0)]
					Yes,
					#[codec(index = 1)]
					No,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PerDispatchClass<_0> {
					pub normal: _0,
					pub operational: _0,
					pub mandatory: _0,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum RawOrigin<_0> {
					#[codec(index = 0)]
					Root,
					#[codec(index = 1)]
					Signed(_0),
					#[codec(index = 2)]
					None,
				}
			}
			pub mod traits {
				use super::runtime_types;
				pub mod preimages {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Bounded<_0> {
						#[codec(index = 0)]
						Legacy {
							hash: ::subxt::utils::H256,
						},
						#[codec(index = 1)]
						Inline(
							runtime_types::bounded_collections::bounded_vec::BoundedVec<
								::core::primitive::u8,
							>,
						),
						#[codec(index = 2)]
						Lookup {
							hash: ::subxt::utils::H256,
							len: ::core::primitive::u32,
						},
						__Ignore(::core::marker::PhantomData<_0>),
					}
				}
				pub mod tokens {
					use super::runtime_types;
					pub mod misc {
						use super::runtime_types;
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							:: subxt :: ext :: scale_decode :: DecodeAsType,
							:: subxt :: ext :: scale_encode :: EncodeAsType,
							Clone,
							Debug,
							Eq,
							PartialEq,
						)]
						# [codec (crate = :: subxt :: ext :: codec)]
						#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
						#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
						pub enum BalanceStatus {
							#[codec(index = 0)]
							Free,
							#[codec(index = 1)]
							Reserved,
						}
					}
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct PalletId(pub [::core::primitive::u8; 8usize]);
		}
		pub mod frame_system {
			use super::runtime_types;
			pub mod extensions {
				use super::runtime_types;
				pub mod check_genesis {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckGenesis;
				}
				pub mod check_mortality {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
				}
				pub mod check_non_zero_sender {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckNonZeroSender;
				}
				pub mod check_nonce {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
				}
				pub mod check_spec_version {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckSpecVersion;
				}
				pub mod check_tx_version {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckTxVersion;
				}
				pub mod check_weight {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckWeight;
				}
			}
			pub mod limits {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BlockLength {
					pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
						::core::primitive::u32,
					>,
					pub cols: runtime_types::avail_core::BlockLengthColumns,
					pub rows: runtime_types::avail_core::BlockLengthRows,
					#[codec(compact)]
					pub chunk_size: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BlockWeights {
					pub base_block: runtime_types::sp_weights::weight_v2::Weight,
					pub max_block: runtime_types::sp_weights::weight_v2::Weight,
					pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
						runtime_types::frame_system::limits::WeightsPerClass,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct WeightsPerClass {
					pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
					pub max_extrinsic:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
					pub max_total:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
					pub reserved:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::remark`]."]
					remark {
						remark: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::set_heap_pages`]."]
					set_heap_pages { pages: ::core::primitive::u64 },
					#[codec(index = 2)]
					#[doc = "See [`Pallet::set_code`]."]
					set_code {
						code: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::set_code_without_checks`]."]
					set_code_without_checks {
						code: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 4)]
					#[doc = "See [`Pallet::set_storage`]."]
					set_storage {
						items: ::std::vec::Vec<(
							::std::vec::Vec<::core::primitive::u8>,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					},
					#[codec(index = 5)]
					#[doc = "See [`Pallet::kill_storage`]."]
					kill_storage {
						keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					},
					#[codec(index = 6)]
					#[doc = "See [`Pallet::kill_prefix`]."]
					kill_prefix {
						prefix: ::std::vec::Vec<::core::primitive::u8>,
						subkeys: ::core::primitive::u32,
					},
					#[codec(index = 7)]
					#[doc = "See [`Pallet::remark_with_event`]."]
					remark_with_event {
						remark: ::std::vec::Vec<::core::primitive::u8>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Error for the System pallet"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The name of specification does not match between the current runtime"]
					#[doc = "and the new runtime."]
					InvalidSpecName,
					#[codec(index = 1)]
					#[doc = "The specification version is not allowed to decrease between the current runtime"]
					#[doc = "and the new runtime."]
					SpecVersionNeedsToIncrease,
					#[codec(index = 2)]
					#[doc = "Failed to extract the runtime version from the new runtime."]
					#[doc = ""]
					#[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
					FailedToExtractRuntimeVersion,
					#[codec(index = 3)]
					#[doc = "Suicide called when the account has non-default composite data."]
					NonDefaultComposite,
					#[codec(index = 4)]
					#[doc = "There is a non-zero reference count preventing the account from being purged."]
					NonZeroRefCount,
					#[codec(index = 5)]
					#[doc = "The origin filter prevent the call to be dispatched."]
					CallFiltered,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Event for the System pallet."]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An extrinsic completed successfully."]
					ExtrinsicSuccess {
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec(index = 1)]
					#[doc = "An extrinsic failed."]
					ExtrinsicFailed {
						dispatch_error: runtime_types::sp_runtime::DispatchError,
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec(index = 2)]
					#[doc = "`:code` was updated."]
					CodeUpdated,
					#[codec(index = 3)]
					#[doc = "A new account was created."]
					NewAccount {
						account: ::subxt::utils::AccountId32,
					},
					#[codec(index = 4)]
					#[doc = "An account was reaped."]
					KilledAccount {
						account: ::subxt::utils::AccountId32,
					},
					#[codec(index = 5)]
					#[doc = "On on-chain remark happened."]
					Remarked {
						sender: ::subxt::utils::AccountId32,
						hash: ::subxt::utils::H256,
					},
					#[codec(index = 6)]
					#[doc = "On on-chain remark happend called by Root."]
					RemarkedByRoot { hash: ::subxt::utils::H256 },
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AccountInfo<_0, _1> {
				pub nonce: _0,
				pub consumers: _0,
				pub providers: _0,
				pub sufficients: _0,
				pub data: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct EventRecord<_0, _1> {
				pub phase: runtime_types::frame_system::Phase,
				pub event: _0,
				pub topics: ::std::vec::Vec<_1>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ExtrinsicLen {
				pub raw: ::core::primitive::u32,
				pub padded: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct LastRuntimeUpgradeInfo {
				#[codec(compact)]
				pub spec_version: ::core::primitive::u32,
				pub spec_name: ::std::string::String,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Phase {
				#[codec(index = 0)]
				ApplyExtrinsic(::core::primitive::u32),
				#[codec(index = 1)]
				Finalization,
				#[codec(index = 2)]
				Initialization,
			}
		}
		pub mod nomad_base {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct NomadBase {
				pub state: runtime_types::nomad_core::state::NomadState,
				pub local_domain: ::core::primitive::u32,
				pub committed_root: ::subxt::utils::H256,
				pub updater: ::subxt::utils::H160,
			}
		}
		pub mod nomad_core {
			use super::runtime_types;
			pub mod state {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum NomadState {
					#[codec(index = 0)]
					Active,
					#[codec(index = 1)]
					Failed,
				}
			}
			pub mod update {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SignedUpdate {
					pub update: runtime_types::nomad_core::update::Update,
					pub signature: runtime_types::nomad_signature::signature::Signature,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Update {
					pub home_domain: ::core::primitive::u32,
					pub previous_root: ::subxt::utils::H256,
					pub new_root: ::subxt::utils::H256,
				}
			}
		}
		pub mod nomad_da_bridge {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::try_dispatch_data_root`]."]
					try_dispatch_data_root {
						#[codec(compact)]
						destination_domain: ::core::primitive::u32,
						recipient_address: ::subxt::utils::H256,
						header: ::std::boxed::Box<
							runtime_types::avail_core::header::Header<
								::core::primitive::u32,
								runtime_types::sp_runtime::traits::BlakeTwo256,
							>,
						>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					InitializationError,
					#[codec(index = 1)]
					HashOfBlockNotMatchBlockNumber,
					#[codec(index = 2)]
					DABridgeMessageExceedsMaxMessageSize,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					DataRootDispatched {
						destination_domain: ::core::primitive::u32,
						recipient_address: ::subxt::utils::H256,
						block_number: ::core::primitive::u32,
						data_root: ::subxt::utils::H256,
					},
				}
			}
		}
		pub mod nomad_home {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::dispatch`]."]
					dispatch {
						#[codec(compact)]
						destination_domain: ::core::primitive::u32,
						recipient_address: ::subxt::utils::H256,
						message_body: runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::update`]."]
					update {
						signed_update: runtime_types::nomad_core::update::SignedUpdate,
						#[codec(compact)]
						max_index: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::improper_update`]."]
					improper_update {
						signed_update: runtime_types::nomad_core::update::SignedUpdate,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::set_updater`]."]
					set_updater { new_updater: ::subxt::utils::H160 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					InitializationError,
					#[codec(index = 1)]
					IngestionError,
					#[codec(index = 2)]
					SignatureRecoveryError,
					#[codec(index = 3)]
					MessageTooLarge,
					#[codec(index = 4)]
					InvalidUpdaterSignature,
					#[codec(index = 5)]
					CommittedRootNotMatchUpdatePrevious,
					#[codec(index = 6)]
					RootForIndexNotFound,
					#[codec(index = 7)]
					IndexForRootNotFound,
					#[codec(index = 8)]
					FailedState,
					#[codec(index = 9)]
					MaxIndexWitnessExhausted,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					Dispatch {
						message_hash: ::subxt::utils::H256,
						leaf_index: ::core::primitive::u32,
						destination_and_nonce: ::core::primitive::u64,
						committed_root: ::subxt::utils::H256,
						message: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					Update {
						home_domain: ::core::primitive::u32,
						previous_root: ::subxt::utils::H256,
						new_root: ::subxt::utils::H256,
						signature: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 2)]
					ImproperUpdate {
						previous_root: ::subxt::utils::H256,
						new_root: ::subxt::utils::H256,
						signature: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 3)]
					UpdaterSlashed {
						updater: ::subxt::utils::H160,
						reporter: ::subxt::utils::AccountId32,
					},
				}
			}
		}
		pub mod nomad_merkle {
			use super::runtime_types;
			pub mod light {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct LightMerkle {
					pub branch: [::subxt::utils::H256; 32usize],
					pub count: ::core::primitive::u32,
				}
			}
		}
		pub mod nomad_signature {
			use super::runtime_types;
			pub mod signature {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Signature {
					pub r: runtime_types::primitive_types::U256,
					pub s: runtime_types::primitive_types::U256,
					pub v: ::core::primitive::u64,
				}
			}
		}
		pub mod nomad_updater_manager {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					InitializationError,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					NewUpdater {
						old_updater: ::subxt::utils::H160,
						new_updater: ::subxt::utils::H160,
					},
					#[codec(index = 1)]
					FakeSlashed {
						reporter: ::subxt::utils::AccountId32,
					},
				}
			}
		}
		pub mod pallet_babe {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::report_equivocation`]."]
					report_equivocation {
						equivocation_proof: ::std::boxed::Box<
							runtime_types::sp_consensus_slots::EquivocationProof<
								runtime_types::avail_core::header::Header<
									::core::primitive::u32,
									runtime_types::sp_runtime::traits::BlakeTwo256,
								>,
								runtime_types::sp_consensus_babe::app::Public,
							>,
						>,
						key_owner_proof: runtime_types::sp_session::MembershipProof,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::report_equivocation_unsigned`]."]
					report_equivocation_unsigned {
						equivocation_proof: ::std::boxed::Box<
							runtime_types::sp_consensus_slots::EquivocationProof<
								runtime_types::avail_core::header::Header<
									::core::primitive::u32,
									runtime_types::sp_runtime::traits::BlakeTwo256,
								>,
								runtime_types::sp_consensus_babe::app::Public,
							>,
						>,
						key_owner_proof: runtime_types::sp_session::MembershipProof,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::plan_config_change`]."]
					plan_config_change {
						config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "An equivocation proof provided as part of an equivocation report is invalid."]
					InvalidEquivocationProof,
					#[codec(index = 1)]
					#[doc = "A key ownership proof provided as part of an equivocation report is invalid."]
					InvalidKeyOwnershipProof,
					#[codec(index = 2)]
					#[doc = "A given equivocation report is valid but already previously reported."]
					DuplicateOffenceReport,
					#[codec(index = 3)]
					#[doc = "Submitted configuration is invalid."]
					InvalidConfiguration,
				}
			}
		}
		pub mod pallet_bags_list {
			use super::runtime_types;
			pub mod list {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Bag {
					pub head: ::core::option::Option<::subxt::utils::AccountId32>,
					pub tail: ::core::option::Option<::subxt::utils::AccountId32>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum ListError {
					#[codec(index = 0)]
					Duplicate,
					#[codec(index = 1)]
					NotHeavier,
					#[codec(index = 2)]
					NotInSameBag,
					#[codec(index = 3)]
					NodeNotFound,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Node {
					pub id: ::subxt::utils::AccountId32,
					pub prev: ::core::option::Option<::subxt::utils::AccountId32>,
					pub next: ::core::option::Option<::subxt::utils::AccountId32>,
					pub bag_upper: ::core::primitive::u64,
					pub score: ::core::primitive::u64,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::rebag`]."]
					rebag {
						dislocated: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::put_in_front_of`]."]
					put_in_front_of {
						lighter: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "A error in the list interface implementation."]
					List(runtime_types::pallet_bags_list::list::ListError),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Moved an account from one bag to another."]
					Rebagged {
						who: ::subxt::utils::AccountId32,
						from: ::core::primitive::u64,
						to: ::core::primitive::u64,
					},
					#[codec(index = 1)]
					#[doc = "Updated the score of some account to the given amount."]
					ScoreUpdated {
						who: ::subxt::utils::AccountId32,
						new_score: ::core::primitive::u64,
					},
				}
			}
		}
		pub mod pallet_balances {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::transfer_allow_death`]."]
					transfer_allow_death {
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::set_balance_deprecated`]."]
					set_balance_deprecated {
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
						#[codec(compact)]
						old_reserved: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::force_transfer`]."]
					force_transfer {
						source: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::transfer_keep_alive`]."]
					transfer_keep_alive {
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "See [`Pallet::transfer_all`]."]
					transfer_all {
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					#[doc = "See [`Pallet::force_unreserve`]."]
					force_unreserve {
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					#[doc = "See [`Pallet::upgrade_accounts`]."]
					upgrade_accounts {
						who: ::std::vec::Vec<::subxt::utils::AccountId32>,
					},
					#[codec(index = 7)]
					#[doc = "See [`Pallet::transfer`]."]
					transfer {
						dest: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					#[doc = "See [`Pallet::force_set_balance`]."]
					force_set_balance {
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Vesting balance too high to send value."]
					VestingBalance,
					#[codec(index = 1)]
					#[doc = "Account liquidity restrictions prevent withdrawal."]
					LiquidityRestrictions,
					#[codec(index = 2)]
					#[doc = "Balance too low to send value."]
					InsufficientBalance,
					#[codec(index = 3)]
					#[doc = "Value too low to create account due to existential deposit."]
					ExistentialDeposit,
					#[codec(index = 4)]
					#[doc = "Transfer/payment would kill account."]
					Expendability,
					#[codec(index = 5)]
					#[doc = "A vesting schedule already exists for this account."]
					ExistingVestingSchedule,
					#[codec(index = 6)]
					#[doc = "Beneficiary account must pre-exist."]
					DeadAccount,
					#[codec(index = 7)]
					#[doc = "Number of named reserves exceed `MaxReserves`."]
					TooManyReserves,
					#[codec(index = 8)]
					#[doc = "Number of holds exceed `MaxHolds`."]
					TooManyHolds,
					#[codec(index = 9)]
					#[doc = "Number of freezes exceed `MaxFreezes`."]
					TooManyFreezes,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An account was created with some free balance."]
					Endowed {
						account: ::subxt::utils::AccountId32,
						free_balance: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
					#[doc = "resulting in an outright loss."]
					DustLost {
						account: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Transfer succeeded."]
					Transfer {
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "A balance was set by root."]
					BalanceSet {
						who: ::subxt::utils::AccountId32,
						free: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "Some balance was reserved (moved from free to reserved)."]
					Reserved {
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					#[doc = "Some balance was unreserved (moved from reserved to free)."]
					Unreserved {
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					#[doc = "Some balance was moved from the reserve of the first account to the second account."]
					#[doc = "Final argument indicates the destination balance type."]
					ReserveRepatriated {
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
						destination_status:
							runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
					},
					#[codec(index = 7)]
					#[doc = "Some amount was deposited (e.g. for transaction fees)."]
					Deposit {
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					#[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
					Withdraw {
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					#[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
					Slashed {
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 10)]
					#[doc = "Some amount was minted into an account."]
					Minted {
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 11)]
					#[doc = "Some amount was burned from an account."]
					Burned {
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 12)]
					#[doc = "Some amount was suspended from an account (it can be restored later)."]
					Suspended {
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 13)]
					#[doc = "Some amount was restored into an account."]
					Restored {
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 14)]
					#[doc = "An account was upgraded."]
					Upgraded { who: ::subxt::utils::AccountId32 },
					#[codec(index = 15)]
					#[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
					Issued { amount: ::core::primitive::u128 },
					#[codec(index = 16)]
					#[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
					Rescinded { amount: ::core::primitive::u128 },
					#[codec(index = 17)]
					#[doc = "Some balance was locked."]
					Locked {
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 18)]
					#[doc = "Some balance was unlocked."]
					Unlocked {
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 19)]
					#[doc = "Some balance was frozen."]
					Frozen {
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 20)]
					#[doc = "Some balance was thawed."]
					Thawed {
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AccountData<_0> {
					pub free: _0,
					pub reserved: _0,
					pub frozen: _0,
					pub flags: runtime_types::pallet_balances::types::ExtraFlags,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BalanceLock<_0> {
					pub id: [::core::primitive::u8; 8usize],
					pub amount: _0,
					pub reasons: runtime_types::pallet_balances::types::Reasons,
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ExtraFlags(pub ::core::primitive::u128);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct IdAmount<_0, _1> {
					pub id: _0,
					pub amount: _1,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Reasons {
					#[codec(index = 0)]
					Fee,
					#[codec(index = 1)]
					Misc,
					#[codec(index = 2)]
					All,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ReserveData<_0, _1> {
					pub id: _0,
					pub amount: _1,
				}
			}
		}
		pub mod pallet_bounties {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::propose_bounty`]."]
					propose_bounty {
						#[codec(compact)]
						value: ::core::primitive::u128,
						description: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::approve_bounty`]."]
					approve_bounty {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::propose_curator`]."]
					propose_curator {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
						curator: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						fee: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::unassign_curator`]."]
					unassign_curator {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					#[doc = "See [`Pallet::accept_curator`]."]
					accept_curator {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					#[doc = "See [`Pallet::award_bounty`]."]
					award_bounty {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
						beneficiary: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 6)]
					#[doc = "See [`Pallet::claim_bounty`]."]
					claim_bounty {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 7)]
					#[doc = "See [`Pallet::close_bounty`]."]
					close_bounty {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					#[doc = "See [`Pallet::extend_bounty_expiry`]."]
					extend_bounty_expiry {
						#[codec(compact)]
						bounty_id: ::core::primitive::u32,
						remark: ::std::vec::Vec<::core::primitive::u8>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Proposer's balance is too low."]
					InsufficientProposersBalance,
					#[codec(index = 1)]
					#[doc = "No proposal or bounty at that index."]
					InvalidIndex,
					#[codec(index = 2)]
					#[doc = "The reason given is just too big."]
					ReasonTooBig,
					#[codec(index = 3)]
					#[doc = "The bounty status is unexpected."]
					UnexpectedStatus,
					#[codec(index = 4)]
					#[doc = "Require bounty curator."]
					RequireCurator,
					#[codec(index = 5)]
					#[doc = "Invalid bounty value."]
					InvalidValue,
					#[codec(index = 6)]
					#[doc = "Invalid bounty fee."]
					InvalidFee,
					#[codec(index = 7)]
					#[doc = "A bounty payout is pending."]
					#[doc = "To cancel the bounty, you must unassign and slash the curator."]
					PendingPayout,
					#[codec(index = 8)]
					#[doc = "The bounties cannot be claimed/closed because it's still in the countdown period."]
					Premature,
					#[codec(index = 9)]
					#[doc = "The bounty cannot be closed because it has active child bounties."]
					HasActiveChildBounty,
					#[codec(index = 10)]
					#[doc = "Too many approvals are already queued."]
					TooManyQueued,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New bounty proposal."]
					BountyProposed { index: ::core::primitive::u32 },
					#[codec(index = 1)]
					#[doc = "A bounty proposal was rejected; funds were slashed."]
					BountyRejected {
						index: ::core::primitive::u32,
						bond: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "A bounty proposal is funded and became active."]
					BountyBecameActive { index: ::core::primitive::u32 },
					#[codec(index = 3)]
					#[doc = "A bounty is awarded to a beneficiary."]
					BountyAwarded {
						index: ::core::primitive::u32,
						beneficiary: ::subxt::utils::AccountId32,
					},
					#[codec(index = 4)]
					#[doc = "A bounty is claimed by beneficiary."]
					BountyClaimed {
						index: ::core::primitive::u32,
						payout: ::core::primitive::u128,
						beneficiary: ::subxt::utils::AccountId32,
					},
					#[codec(index = 5)]
					#[doc = "A bounty is cancelled."]
					BountyCanceled { index: ::core::primitive::u32 },
					#[codec(index = 6)]
					#[doc = "A bounty expiry is extended."]
					BountyExtended { index: ::core::primitive::u32 },
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Bounty<_0, _1, _2> {
				pub proposer: _0,
				pub value: _1,
				pub fee: _1,
				pub curator_deposit: _1,
				pub bond: _1,
				pub status: runtime_types::pallet_bounties::BountyStatus<_0, _2>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum BountyStatus<_0, _1> {
				#[codec(index = 0)]
				Proposed,
				#[codec(index = 1)]
				Approved,
				#[codec(index = 2)]
				Funded,
				#[codec(index = 3)]
				CuratorProposed { curator: _0 },
				#[codec(index = 4)]
				Active { curator: _0, update_due: _1 },
				#[codec(index = 5)]
				PendingPayout {
					curator: _0,
					beneficiary: _0,
					unlock_at: _1,
				},
			}
		}
		pub mod pallet_collective {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::set_members`]."]
					set_members {
						new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
						prime: ::core::option::Option<::subxt::utils::AccountId32>,
						old_count: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::execute`]."]
					execute {
						proposal: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::propose`]."]
					propose {
						#[codec(compact)]
						threshold: ::core::primitive::u32,
						proposal: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::vote`]."]
					vote {
						proposal: ::subxt::utils::H256,
						#[codec(compact)]
						index: ::core::primitive::u32,
						approve: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					#[doc = "See [`Pallet::disapprove_proposal`]."]
					disapprove_proposal { proposal_hash: ::subxt::utils::H256 },
					#[codec(index = 6)]
					#[doc = "See [`Pallet::close`]."]
					close {
						proposal_hash: ::subxt::utils::H256,
						#[codec(compact)]
						index: ::core::primitive::u32,
						proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Account is not a member"]
					NotMember,
					#[codec(index = 1)]
					#[doc = "Duplicate proposals not allowed"]
					DuplicateProposal,
					#[codec(index = 2)]
					#[doc = "Proposal must exist"]
					ProposalMissing,
					#[codec(index = 3)]
					#[doc = "Mismatched index"]
					WrongIndex,
					#[codec(index = 4)]
					#[doc = "Duplicate vote ignored"]
					DuplicateVote,
					#[codec(index = 5)]
					#[doc = "Members are already initialized!"]
					AlreadyInitialized,
					#[codec(index = 6)]
					#[doc = "The close call was made too early, before the end of the voting."]
					TooEarly,
					#[codec(index = 7)]
					#[doc = "There can only be a maximum of `MaxProposals` active proposals."]
					TooManyProposals,
					#[codec(index = 8)]
					#[doc = "The given weight bound for the proposal was too low."]
					WrongProposalWeight,
					#[codec(index = 9)]
					#[doc = "The given length bound for the proposal was too low."]
					WrongProposalLength,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A motion (given hash) has been proposed (by given account) with a threshold (given"]
					#[doc = "`MemberCount`)."]
					Proposed {
						account: ::subxt::utils::AccountId32,
						proposal_index: ::core::primitive::u32,
						proposal_hash: ::subxt::utils::H256,
						threshold: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "A motion (given hash) has been voted on by given account, leaving"]
					#[doc = "a tally (yes votes and no votes given respectively as `MemberCount`)."]
					Voted {
						account: ::subxt::utils::AccountId32,
						proposal_hash: ::subxt::utils::H256,
						voted: ::core::primitive::bool,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "A motion was approved by the required threshold."]
					Approved { proposal_hash: ::subxt::utils::H256 },
					#[codec(index = 3)]
					#[doc = "A motion was not approved by the required threshold."]
					Disapproved { proposal_hash: ::subxt::utils::H256 },
					#[codec(index = 4)]
					#[doc = "A motion was executed; result will be `Ok` if it returned without error."]
					Executed {
						proposal_hash: ::subxt::utils::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 5)]
					#[doc = "A single member did some action; result will be `Ok` if it returned without error."]
					MemberExecuted {
						proposal_hash: ::subxt::utils::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 6)]
					#[doc = "A proposal was closed because its threshold was reached or after its duration was up."]
					Closed {
						proposal_hash: ::subxt::utils::H256,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RawOrigin<_0> {
				#[codec(index = 0)]
				Members(::core::primitive::u32, ::core::primitive::u32),
				#[codec(index = 1)]
				Member(_0),
				#[codec(index = 2)]
				_Phantom,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Votes<_0, _1> {
				pub index: _1,
				pub threshold: _1,
				pub ayes: ::std::vec::Vec<_0>,
				pub nays: ::std::vec::Vec<_0>,
				pub end: _1,
			}
		}
		pub mod pallet_election_provider_multi_phase {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					# [codec (index = 0)] # [doc = "See [`Pallet::submit_unsigned`]."] submit_unsigned { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: da_runtime :: constants :: staking :: NposSolution16 > > , witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize , } , # [codec (index = 1)] # [doc = "See [`Pallet::set_minimum_untrusted_score`]."] set_minimum_untrusted_score { maybe_next_score : :: core :: option :: Option < runtime_types :: sp_npos_elections :: ElectionScore > , } , # [codec (index = 2)] # [doc = "See [`Pallet::set_emergency_election_result`]."] set_emergency_election_result { supports : :: std :: vec :: Vec < (:: subxt :: utils :: AccountId32 , runtime_types :: sp_npos_elections :: Support < :: subxt :: utils :: AccountId32 > ,) > , } , # [codec (index = 3)] # [doc = "See [`Pallet::submit`]."] submit { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: da_runtime :: constants :: staking :: NposSolution16 > > , } , # [codec (index = 4)] # [doc = "See [`Pallet::governance_fallback`]."] governance_fallback { maybe_max_voters : :: core :: option :: Option < :: core :: primitive :: u32 > , maybe_max_targets : :: core :: option :: Option < :: core :: primitive :: u32 > , } , }
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Error of the pallet that can be returned in response to dispatches."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Submission was too early."]
					PreDispatchEarlySubmission,
					#[codec(index = 1)]
					#[doc = "Wrong number of winners presented."]
					PreDispatchWrongWinnerCount,
					#[codec(index = 2)]
					#[doc = "Submission was too weak, score-wise."]
					PreDispatchWeakSubmission,
					#[codec(index = 3)]
					#[doc = "The queue was full, and the solution was not better than any of the existing ones."]
					SignedQueueFull,
					#[codec(index = 4)]
					#[doc = "The origin failed to pay the deposit."]
					SignedCannotPayDeposit,
					#[codec(index = 5)]
					#[doc = "Witness data to dispatchable is invalid."]
					SignedInvalidWitness,
					#[codec(index = 6)]
					#[doc = "The signed submission consumes too much weight"]
					SignedTooMuchWeight,
					#[codec(index = 7)]
					#[doc = "OCW submitted solution for wrong round"]
					OcwCallWrongEra,
					#[codec(index = 8)]
					#[doc = "Snapshot metadata should exist but didn't."]
					MissingSnapshotMetadata,
					#[codec(index = 9)]
					#[doc = "`Self::insert_submission` returned an invalid index."]
					InvalidSubmissionIndex,
					#[codec(index = 10)]
					#[doc = "The call is not allowed at this point."]
					CallNotAllowed,
					#[codec(index = 11)]
					#[doc = "The fallback failed"]
					FallbackFailed,
					#[codec(index = 12)]
					#[doc = "Some bound not met"]
					BoundNotMet,
					#[codec(index = 13)]
					#[doc = "Submitted solution has too many winners"]
					TooManyWinners,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A solution was stored with the given compute."]
					#[doc = ""]
					#[doc = "The `origin` indicates the origin of the solution. If `origin` is `Some(AccountId)`,"]
					#[doc = "the stored solution was submited in the signed phase by a miner with the `AccountId`."]
					#[doc = "Otherwise, the solution was stored either during the unsigned phase or by"]
					#[doc = "`T::ForceOrigin`. The `bool` is `true` when a previous solution was ejected to make"]
					#[doc = "room for this one."]
					SolutionStored {
						compute:
							runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
						origin: ::core::option::Option<::subxt::utils::AccountId32>,
						prev_ejected: ::core::primitive::bool,
					},
					#[codec(index = 1)]
					#[doc = "The election has been finalized, with the given computation and score."]
					ElectionFinalized {
						compute:
							runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
						score: runtime_types::sp_npos_elections::ElectionScore,
					},
					#[codec(index = 2)]
					#[doc = "An election failed."]
					#[doc = ""]
					#[doc = "Not much can be said about which computes failed in the process."]
					ElectionFailed,
					#[codec(index = 3)]
					#[doc = "An account has been rewarded for their signed submission being finalized."]
					Rewarded {
						account: ::subxt::utils::AccountId32,
						value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "An account has been slashed for submitting an invalid signed submission."]
					Slashed {
						account: ::subxt::utils::AccountId32,
						value: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					#[doc = "There was a phase transition in a given round."]
					PhaseTransitioned {
						from: runtime_types::pallet_election_provider_multi_phase::Phase<
							::core::primitive::u32,
						>,
						to: runtime_types::pallet_election_provider_multi_phase::Phase<
							::core::primitive::u32,
						>,
						round: ::core::primitive::u32,
					},
				}
			}
			pub mod signed {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SignedSubmission<_0, _1, _2> {
					pub who: _0,
					pub deposit: _1,
					pub raw_solution:
						runtime_types::pallet_election_provider_multi_phase::RawSolution<_2>,
					pub call_fee: _1,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum ElectionCompute {
				#[codec(index = 0)]
				OnChain,
				#[codec(index = 1)]
				Signed,
				#[codec(index = 2)]
				Unsigned,
				#[codec(index = 3)]
				Fallback,
				#[codec(index = 4)]
				Emergency,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Phase<_0> {
				#[codec(index = 0)]
				Off,
				#[codec(index = 1)]
				Signed,
				#[codec(index = 2)]
				Unsigned((::core::primitive::bool, _0)),
				#[codec(index = 3)]
				Emergency,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RawSolution<_0> {
				pub solution: _0,
				pub score: runtime_types::sp_npos_elections::ElectionScore,
				pub round: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ReadySolution {
				pub supports: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
					::subxt::utils::AccountId32,
					runtime_types::sp_npos_elections::Support<::subxt::utils::AccountId32>,
				)>,
				pub score: runtime_types::sp_npos_elections::ElectionScore,
				pub compute: runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RoundSnapshot<_0, _1> {
				pub voters: ::std::vec::Vec<_1>,
				pub targets: ::std::vec::Vec<_0>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SolutionOrSnapshotSize {
				#[codec(compact)]
				pub voters: ::core::primitive::u32,
				#[codec(compact)]
				pub targets: ::core::primitive::u32,
			}
		}
		pub mod pallet_grandpa {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::report_equivocation`]."]
					report_equivocation {
						equivocation_proof: ::std::boxed::Box<
							runtime_types::sp_consensus_grandpa::EquivocationProof<
								::subxt::utils::H256,
								::core::primitive::u32,
							>,
						>,
						key_owner_proof: runtime_types::sp_session::MembershipProof,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::report_equivocation_unsigned`]."]
					report_equivocation_unsigned {
						equivocation_proof: ::std::boxed::Box<
							runtime_types::sp_consensus_grandpa::EquivocationProof<
								::subxt::utils::H256,
								::core::primitive::u32,
							>,
						>,
						key_owner_proof: runtime_types::sp_session::MembershipProof,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::note_stalled`]."]
					note_stalled {
						delay: ::core::primitive::u32,
						best_finalized_block_number: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Attempt to signal GRANDPA pause when the authority set isn't live"]
					#[doc = "(either paused or already pending pause)."]
					PauseFailed,
					#[codec(index = 1)]
					#[doc = "Attempt to signal GRANDPA resume when the authority set isn't paused"]
					#[doc = "(either live or already pending resume)."]
					ResumeFailed,
					#[codec(index = 2)]
					#[doc = "Attempt to signal GRANDPA change with one already pending."]
					ChangePending,
					#[codec(index = 3)]
					#[doc = "Cannot signal forced change so soon after last."]
					TooSoon,
					#[codec(index = 4)]
					#[doc = "A key ownership proof provided as part of an equivocation report is invalid."]
					InvalidKeyOwnershipProof,
					#[codec(index = 5)]
					#[doc = "An equivocation proof provided as part of an equivocation report is invalid."]
					InvalidEquivocationProof,
					#[codec(index = 6)]
					#[doc = "A given equivocation report is valid but already previously reported."]
					DuplicateOffenceReport,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New authority set has been applied."]
					NewAuthorities {
						authority_set: ::std::vec::Vec<(
							runtime_types::sp_consensus_grandpa::app::Public,
							::core::primitive::u64,
						)>,
					},
					#[codec(index = 1)]
					#[doc = "Current authority set has been paused."]
					Paused,
					#[codec(index = 2)]
					#[doc = "Current authority set has been resumed."]
					Resumed,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct StoredPendingChange<_0> {
				pub scheduled_at: _0,
				pub delay: _0,
				pub next_authorities:
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
						runtime_types::sp_consensus_grandpa::app::Public,
						::core::primitive::u64,
					)>,
				pub forced: ::core::option::Option<_0>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum StoredState<_0> {
				#[codec(index = 0)]
				Live,
				#[codec(index = 1)]
				PendingPause { scheduled_at: _0, delay: _0 },
				#[codec(index = 2)]
				Paused,
				#[codec(index = 3)]
				PendingResume { scheduled_at: _0, delay: _0 },
			}
		}
		pub mod pallet_identity {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Identity pallet declaration."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::add_registrar`]."]
					add_registrar {
						account: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::set_identity`]."]
					set_identity {
						info:
							::std::boxed::Box<runtime_types::pallet_identity::types::IdentityInfo>,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::set_subs`]."]
					set_subs {
						subs: ::std::vec::Vec<(
							::subxt::utils::AccountId32,
							runtime_types::pallet_identity::types::Data,
						)>,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::clear_identity`]."]
					clear_identity,
					#[codec(index = 4)]
					#[doc = "See [`Pallet::request_judgement`]."]
					request_judgement {
						#[codec(compact)]
						reg_index: ::core::primitive::u32,
						#[codec(compact)]
						max_fee: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					#[doc = "See [`Pallet::cancel_request`]."]
					cancel_request { reg_index: ::core::primitive::u32 },
					#[codec(index = 6)]
					#[doc = "See [`Pallet::set_fee`]."]
					set_fee {
						#[codec(compact)]
						index: ::core::primitive::u32,
						#[codec(compact)]
						fee: ::core::primitive::u128,
					},
					#[codec(index = 7)]
					#[doc = "See [`Pallet::set_account_id`]."]
					set_account_id {
						#[codec(compact)]
						index: ::core::primitive::u32,
						new: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 8)]
					#[doc = "See [`Pallet::set_fields`]."]
					set_fields {
						#[codec(compact)]
						index: ::core::primitive::u32,
						fields: runtime_types::pallet_identity::types::BitFlags<
							runtime_types::pallet_identity::types::IdentityField,
						>,
					},
					#[codec(index = 9)]
					#[doc = "See [`Pallet::provide_judgement`]."]
					provide_judgement {
						#[codec(compact)]
						reg_index: ::core::primitive::u32,
						target: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						judgement: runtime_types::pallet_identity::types::Judgement<
							::core::primitive::u128,
						>,
						identity: ::subxt::utils::H256,
					},
					#[codec(index = 10)]
					#[doc = "See [`Pallet::kill_identity`]."]
					kill_identity {
						target: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 11)]
					#[doc = "See [`Pallet::add_sub`]."]
					add_sub {
						sub: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						data: runtime_types::pallet_identity::types::Data,
					},
					#[codec(index = 12)]
					#[doc = "See [`Pallet::rename_sub`]."]
					rename_sub {
						sub: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						data: runtime_types::pallet_identity::types::Data,
					},
					#[codec(index = 13)]
					#[doc = "See [`Pallet::remove_sub`]."]
					remove_sub {
						sub: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 14)]
					#[doc = "See [`Pallet::quit_sub`]."]
					quit_sub,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Too many subs-accounts."]
					TooManySubAccounts,
					#[codec(index = 1)]
					#[doc = "Account isn't found."]
					NotFound,
					#[codec(index = 2)]
					#[doc = "Account isn't named."]
					NotNamed,
					#[codec(index = 3)]
					#[doc = "Empty index."]
					EmptyIndex,
					#[codec(index = 4)]
					#[doc = "Fee is changed."]
					FeeChanged,
					#[codec(index = 5)]
					#[doc = "No identity found."]
					NoIdentity,
					#[codec(index = 6)]
					#[doc = "Sticky judgement."]
					StickyJudgement,
					#[codec(index = 7)]
					#[doc = "Judgement given."]
					JudgementGiven,
					#[codec(index = 8)]
					#[doc = "Invalid judgement."]
					InvalidJudgement,
					#[codec(index = 9)]
					#[doc = "The index is invalid."]
					InvalidIndex,
					#[codec(index = 10)]
					#[doc = "The target is invalid."]
					InvalidTarget,
					#[codec(index = 11)]
					#[doc = "Too many additional fields."]
					TooManyFields,
					#[codec(index = 12)]
					#[doc = "Maximum amount of registrars reached. Cannot add any more."]
					TooManyRegistrars,
					#[codec(index = 13)]
					#[doc = "Account ID is already named."]
					AlreadyClaimed,
					#[codec(index = 14)]
					#[doc = "Sender is not a sub-account."]
					NotSub,
					#[codec(index = 15)]
					#[doc = "Sub-account isn't owned by sender."]
					NotOwned,
					#[codec(index = 16)]
					#[doc = "The provided judgement was for a different identity."]
					JudgementForDifferentIdentity,
					#[codec(index = 17)]
					#[doc = "Error that occurs when there is an issue paying for judgement."]
					JudgementPaymentFailed,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A name was set or reset (which will remove all judgements)."]
					IdentitySet { who: ::subxt::utils::AccountId32 },
					#[codec(index = 1)]
					#[doc = "A name was cleared, and the given balance returned."]
					IdentityCleared {
						who: ::subxt::utils::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "A name was removed and the given balance slashed."]
					IdentityKilled {
						who: ::subxt::utils::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "A judgement was asked from a registrar."]
					JudgementRequested {
						who: ::subxt::utils::AccountId32,
						registrar_index: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					#[doc = "A judgement request was retracted."]
					JudgementUnrequested {
						who: ::subxt::utils::AccountId32,
						registrar_index: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					#[doc = "A judgement was given by a registrar."]
					JudgementGiven {
						target: ::subxt::utils::AccountId32,
						registrar_index: ::core::primitive::u32,
					},
					#[codec(index = 6)]
					#[doc = "A registrar was added."]
					RegistrarAdded {
						registrar_index: ::core::primitive::u32,
					},
					#[codec(index = 7)]
					#[doc = "A sub-identity was added to an identity and the deposit paid."]
					SubIdentityAdded {
						sub: ::subxt::utils::AccountId32,
						main: ::subxt::utils::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					#[doc = "A sub-identity was removed from an identity and the deposit freed."]
					SubIdentityRemoved {
						sub: ::subxt::utils::AccountId32,
						main: ::subxt::utils::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					#[doc = "A sub-identity was cleared, and the given deposit repatriated from the"]
					#[doc = "main identity account to the sub-identity account."]
					SubIdentityRevoked {
						sub: ::subxt::utils::AccountId32,
						main: ::subxt::utils::AccountId32,
						deposit: ::core::primitive::u128,
					},
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BitFlags<_0>(
					pub ::core::primitive::u64,
					#[codec(skip)] pub ::core::marker::PhantomData<_0>,
				);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Data {
					#[codec(index = 0)]
					None,
					#[codec(index = 1)]
					Raw0([::core::primitive::u8; 0usize]),
					#[codec(index = 2)]
					Raw1([::core::primitive::u8; 1usize]),
					#[codec(index = 3)]
					Raw2([::core::primitive::u8; 2usize]),
					#[codec(index = 4)]
					Raw3([::core::primitive::u8; 3usize]),
					#[codec(index = 5)]
					Raw4([::core::primitive::u8; 4usize]),
					#[codec(index = 6)]
					Raw5([::core::primitive::u8; 5usize]),
					#[codec(index = 7)]
					Raw6([::core::primitive::u8; 6usize]),
					#[codec(index = 8)]
					Raw7([::core::primitive::u8; 7usize]),
					#[codec(index = 9)]
					Raw8([::core::primitive::u8; 8usize]),
					#[codec(index = 10)]
					Raw9([::core::primitive::u8; 9usize]),
					#[codec(index = 11)]
					Raw10([::core::primitive::u8; 10usize]),
					#[codec(index = 12)]
					Raw11([::core::primitive::u8; 11usize]),
					#[codec(index = 13)]
					Raw12([::core::primitive::u8; 12usize]),
					#[codec(index = 14)]
					Raw13([::core::primitive::u8; 13usize]),
					#[codec(index = 15)]
					Raw14([::core::primitive::u8; 14usize]),
					#[codec(index = 16)]
					Raw15([::core::primitive::u8; 15usize]),
					#[codec(index = 17)]
					Raw16([::core::primitive::u8; 16usize]),
					#[codec(index = 18)]
					Raw17([::core::primitive::u8; 17usize]),
					#[codec(index = 19)]
					Raw18([::core::primitive::u8; 18usize]),
					#[codec(index = 20)]
					Raw19([::core::primitive::u8; 19usize]),
					#[codec(index = 21)]
					Raw20([::core::primitive::u8; 20usize]),
					#[codec(index = 22)]
					Raw21([::core::primitive::u8; 21usize]),
					#[codec(index = 23)]
					Raw22([::core::primitive::u8; 22usize]),
					#[codec(index = 24)]
					Raw23([::core::primitive::u8; 23usize]),
					#[codec(index = 25)]
					Raw24([::core::primitive::u8; 24usize]),
					#[codec(index = 26)]
					Raw25([::core::primitive::u8; 25usize]),
					#[codec(index = 27)]
					Raw26([::core::primitive::u8; 26usize]),
					#[codec(index = 28)]
					Raw27([::core::primitive::u8; 27usize]),
					#[codec(index = 29)]
					Raw28([::core::primitive::u8; 28usize]),
					#[codec(index = 30)]
					Raw29([::core::primitive::u8; 29usize]),
					#[codec(index = 31)]
					Raw30([::core::primitive::u8; 30usize]),
					#[codec(index = 32)]
					Raw31([::core::primitive::u8; 31usize]),
					#[codec(index = 33)]
					Raw32([::core::primitive::u8; 32usize]),
					#[codec(index = 34)]
					BlakeTwo256([::core::primitive::u8; 32usize]),
					#[codec(index = 35)]
					Sha256([::core::primitive::u8; 32usize]),
					#[codec(index = 36)]
					Keccak256([::core::primitive::u8; 32usize]),
					#[codec(index = 37)]
					ShaThree256([::core::primitive::u8; 32usize]),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum IdentityField {
					#[codec(index = 1)]
					Display,
					#[codec(index = 2)]
					Legal,
					#[codec(index = 4)]
					Web,
					#[codec(index = 8)]
					Riot,
					#[codec(index = 16)]
					Email,
					#[codec(index = 32)]
					PgpFingerprint,
					#[codec(index = 64)]
					Image,
					#[codec(index = 128)]
					Twitter,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct IdentityInfo {
					pub additional: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						runtime_types::pallet_identity::types::Data,
						runtime_types::pallet_identity::types::Data,
					)>,
					pub display: runtime_types::pallet_identity::types::Data,
					pub legal: runtime_types::pallet_identity::types::Data,
					pub web: runtime_types::pallet_identity::types::Data,
					pub riot: runtime_types::pallet_identity::types::Data,
					pub email: runtime_types::pallet_identity::types::Data,
					pub pgp_fingerprint: ::core::option::Option<[::core::primitive::u8; 20usize]>,
					pub image: runtime_types::pallet_identity::types::Data,
					pub twitter: runtime_types::pallet_identity::types::Data,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Judgement<_0> {
					#[codec(index = 0)]
					Unknown,
					#[codec(index = 1)]
					FeePaid(_0),
					#[codec(index = 2)]
					Reasonable,
					#[codec(index = 3)]
					KnownGood,
					#[codec(index = 4)]
					OutOfDate,
					#[codec(index = 5)]
					LowQuality,
					#[codec(index = 6)]
					Erroneous,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RegistrarInfo<_0, _1> {
					pub account: _1,
					pub fee: _0,
					pub fields: runtime_types::pallet_identity::types::BitFlags<
						runtime_types::pallet_identity::types::IdentityField,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Registration<_0> {
					pub judgements: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						::core::primitive::u32,
						runtime_types::pallet_identity::types::Judgement<_0>,
					)>,
					pub deposit: _0,
					pub info: runtime_types::pallet_identity::types::IdentityInfo,
				}
			}
		}
		pub mod pallet_im_online {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::heartbeat`]."]
					heartbeat {
						heartbeat:
							runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
						signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Non existent public key."]
					InvalidKey,
					#[codec(index = 1)]
					#[doc = "Duplicated heartbeat."]
					DuplicatedHeartbeat,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A new heartbeat was received from `AuthorityId`."]
					HeartbeatReceived {
						authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
					},
					#[codec(index = 1)]
					#[doc = "At the end of the session, no offence was committed."]
					AllGood,
					#[codec(index = 2)]
					#[doc = "At the end of the session, at least one validator was found to be offline."]
					SomeOffline {
						offline: ::std::vec::Vec<(
							::subxt::utils::AccountId32,
							runtime_types::pallet_staking::Exposure<
								::subxt::utils::AccountId32,
								::core::primitive::u128,
							>,
						)>,
					},
				}
			}
			pub mod sr25519 {
				use super::runtime_types;
				pub mod app_sr25519 {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Public(pub runtime_types::sp_core::sr25519::Public);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Heartbeat<_0> {
				pub block_number: _0,
				pub session_index: _0,
				pub authority_index: _0,
				pub validators_len: _0,
			}
		}
		pub mod pallet_indices {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::claim`]."]
					claim { index: ::core::primitive::u32 },
					#[codec(index = 1)]
					#[doc = "See [`Pallet::transfer`]."]
					transfer {
						new: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						index: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::free`]."]
					free { index: ::core::primitive::u32 },
					#[codec(index = 3)]
					#[doc = "See [`Pallet::force_transfer`]."]
					force_transfer {
						new: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						index: ::core::primitive::u32,
						freeze: ::core::primitive::bool,
					},
					#[codec(index = 4)]
					#[doc = "See [`Pallet::freeze`]."]
					freeze { index: ::core::primitive::u32 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The index was not already assigned."]
					NotAssigned,
					#[codec(index = 1)]
					#[doc = "The index is assigned to another account."]
					NotOwner,
					#[codec(index = 2)]
					#[doc = "The index was not available."]
					InUse,
					#[codec(index = 3)]
					#[doc = "The source and destination accounts are identical."]
					NotTransfer,
					#[codec(index = 4)]
					#[doc = "The index is permanent and may not be freed/changed."]
					Permanent,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A account index was assigned."]
					IndexAssigned {
						who: ::subxt::utils::AccountId32,
						index: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "A account index has been freed up (unassigned)."]
					IndexFreed { index: ::core::primitive::u32 },
					#[codec(index = 2)]
					#[doc = "A account index has been frozen to its current account ID."]
					IndexFrozen {
						index: ::core::primitive::u32,
						who: ::subxt::utils::AccountId32,
					},
				}
			}
		}
		pub mod pallet_mandate {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::mandate`]."]
					mandate {
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A root operation was executed, show result"]
					RootOp {
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
				}
			}
		}
		pub mod pallet_membership {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::add_member`]."]
					add_member {
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::remove_member`]."]
					remove_member {
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::swap_member`]."]
					swap_member {
						remove: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						add: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::reset_members`]."]
					reset_members {
						members: ::std::vec::Vec<::subxt::utils::AccountId32>,
					},
					#[codec(index = 4)]
					#[doc = "See [`Pallet::change_key`]."]
					change_key {
						new: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 5)]
					#[doc = "See [`Pallet::set_prime`]."]
					set_prime {
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 6)]
					#[doc = "See [`Pallet::clear_prime`]."]
					clear_prime,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Already a member."]
					AlreadyMember,
					#[codec(index = 1)]
					#[doc = "Not a member."]
					NotMember,
					#[codec(index = 2)]
					#[doc = "Too many members."]
					TooManyMembers,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "The given member was added; see the transaction for who."]
					MemberAdded,
					#[codec(index = 1)]
					#[doc = "The given member was removed; see the transaction for who."]
					MemberRemoved,
					#[codec(index = 2)]
					#[doc = "Two members were swapped; see the transaction for who."]
					MembersSwapped,
					#[codec(index = 3)]
					#[doc = "The membership was reset; see the transaction for who the new set is."]
					MembersReset,
					#[codec(index = 4)]
					#[doc = "One of the members' keys changed."]
					KeyChanged,
					#[codec(index = 5)]
					#[doc = "Phantom member, never used."]
					Dummy,
				}
			}
		}
		pub mod pallet_multisig {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::as_multi_threshold_1`]."]
					as_multi_threshold_1 {
						other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::as_multi`]."]
					as_multi {
						threshold: ::core::primitive::u16,
						other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
						maybe_timepoint: ::core::option::Option<
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						>,
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::approve_as_multi`]."]
					approve_as_multi {
						threshold: ::core::primitive::u16,
						other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
						maybe_timepoint: ::core::option::Option<
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						>,
						call_hash: [::core::primitive::u8; 32usize],
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::cancel_as_multi`]."]
					cancel_as_multi {
						threshold: ::core::primitive::u16,
						other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						call_hash: [::core::primitive::u8; 32usize],
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Threshold must be 2 or greater."]
					MinimumThreshold,
					#[codec(index = 1)]
					#[doc = "Call is already approved by this signatory."]
					AlreadyApproved,
					#[codec(index = 2)]
					#[doc = "Call doesn't need any (more) approvals."]
					NoApprovalsNeeded,
					#[codec(index = 3)]
					#[doc = "There are too few signatories in the list."]
					TooFewSignatories,
					#[codec(index = 4)]
					#[doc = "There are too many signatories in the list."]
					TooManySignatories,
					#[codec(index = 5)]
					#[doc = "The signatories were provided out of order; they should be ordered."]
					SignatoriesOutOfOrder,
					#[codec(index = 6)]
					#[doc = "The sender was contained in the other signatories; it shouldn't be."]
					SenderInSignatories,
					#[codec(index = 7)]
					#[doc = "Multisig operation not found when attempting to cancel."]
					NotFound,
					#[codec(index = 8)]
					#[doc = "Only the account that originally created the multisig is able to cancel it."]
					NotOwner,
					#[codec(index = 9)]
					#[doc = "No timepoint was given, yet the multisig operation is already underway."]
					NoTimepoint,
					#[codec(index = 10)]
					#[doc = "A different timepoint was given to the multisig operation that is underway."]
					WrongTimepoint,
					#[codec(index = 11)]
					#[doc = "A timepoint was given, yet no multisig operation is underway."]
					UnexpectedTimepoint,
					#[codec(index = 12)]
					#[doc = "The maximum weight information provided was too low."]
					MaxWeightTooLow,
					#[codec(index = 13)]
					#[doc = "The data to be stored is already stored."]
					AlreadyStored,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A new multisig operation has begun."]
					NewMultisig {
						approving: ::subxt::utils::AccountId32,
						multisig: ::subxt::utils::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 1)]
					#[doc = "A multisig operation has been approved by someone."]
					MultisigApproval {
						approving: ::subxt::utils::AccountId32,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						multisig: ::subxt::utils::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 2)]
					#[doc = "A multisig operation has been executed."]
					MultisigExecuted {
						approving: ::subxt::utils::AccountId32,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						multisig: ::subxt::utils::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 3)]
					#[doc = "A multisig operation has been cancelled."]
					MultisigCancelled {
						cancelling: ::subxt::utils::AccountId32,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						multisig: ::subxt::utils::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Multisig<_0, _1, _2> {
				pub when: runtime_types::pallet_multisig::Timepoint<_0>,
				pub deposit: _1,
				pub depositor: _2,
				pub approvals: runtime_types::bounded_collections::bounded_vec::BoundedVec<_2>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Timepoint<_0> {
				pub height: _0,
				pub index: _0,
			}
		}
		pub mod pallet_nomination_pools {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::join`]."]
					join {
						#[codec(compact)]
						amount: ::core::primitive::u128,
						pool_id: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::bond_extra`]."]
					bond_extra {
						extra: runtime_types::pallet_nomination_pools::BondExtra<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::claim_payout`]."]
					claim_payout,
					#[codec(index = 3)]
					#[doc = "See [`Pallet::unbond`]."]
					unbond {
						member_account: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						unbonding_points: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "See [`Pallet::pool_withdraw_unbonded`]."]
					pool_withdraw_unbonded {
						pool_id: ::core::primitive::u32,
						num_slashing_spans: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					#[doc = "See [`Pallet::withdraw_unbonded`]."]
					withdraw_unbonded {
						member_account: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						num_slashing_spans: ::core::primitive::u32,
					},
					#[codec(index = 6)]
					#[doc = "See [`Pallet::create`]."]
					create {
						#[codec(compact)]
						amount: ::core::primitive::u128,
						root: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						nominator: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						bouncer: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 7)]
					#[doc = "See [`Pallet::create_with_pool_id`]."]
					create_with_pool_id {
						#[codec(compact)]
						amount: ::core::primitive::u128,
						root: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						nominator: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						bouncer: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						pool_id: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					#[doc = "See [`Pallet::nominate`]."]
					nominate {
						pool_id: ::core::primitive::u32,
						validators: ::std::vec::Vec<::subxt::utils::AccountId32>,
					},
					#[codec(index = 9)]
					#[doc = "See [`Pallet::set_state`]."]
					set_state {
						pool_id: ::core::primitive::u32,
						state: runtime_types::pallet_nomination_pools::PoolState,
					},
					#[codec(index = 10)]
					#[doc = "See [`Pallet::set_metadata`]."]
					set_metadata {
						pool_id: ::core::primitive::u32,
						metadata: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 11)]
					#[doc = "See [`Pallet::set_configs`]."]
					set_configs {
						min_join_bond: runtime_types::pallet_nomination_pools::ConfigOp<
							::core::primitive::u128,
						>,
						min_create_bond: runtime_types::pallet_nomination_pools::ConfigOp<
							::core::primitive::u128,
						>,
						max_pools: runtime_types::pallet_nomination_pools::ConfigOp<
							::core::primitive::u32,
						>,
						max_members: runtime_types::pallet_nomination_pools::ConfigOp<
							::core::primitive::u32,
						>,
						max_members_per_pool: runtime_types::pallet_nomination_pools::ConfigOp<
							::core::primitive::u32,
						>,
						global_max_commission: runtime_types::pallet_nomination_pools::ConfigOp<
							runtime_types::sp_arithmetic::per_things::Perbill,
						>,
					},
					#[codec(index = 12)]
					#[doc = "See [`Pallet::update_roles`]."]
					update_roles {
						pool_id: ::core::primitive::u32,
						new_root: runtime_types::pallet_nomination_pools::ConfigOp<
							::subxt::utils::AccountId32,
						>,
						new_nominator: runtime_types::pallet_nomination_pools::ConfigOp<
							::subxt::utils::AccountId32,
						>,
						new_bouncer: runtime_types::pallet_nomination_pools::ConfigOp<
							::subxt::utils::AccountId32,
						>,
					},
					#[codec(index = 13)]
					#[doc = "See [`Pallet::chill`]."]
					chill { pool_id: ::core::primitive::u32 },
					#[codec(index = 14)]
					#[doc = "See [`Pallet::bond_extra_other`]."]
					bond_extra_other {
						member: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						extra: runtime_types::pallet_nomination_pools::BondExtra<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 15)]
					#[doc = "See [`Pallet::set_claim_permission`]."]
					set_claim_permission {
						permission: runtime_types::pallet_nomination_pools::ClaimPermission,
					},
					#[codec(index = 16)]
					#[doc = "See [`Pallet::claim_payout_other`]."]
					claim_payout_other { other: ::subxt::utils::AccountId32 },
					#[codec(index = 17)]
					#[doc = "See [`Pallet::set_commission`]."]
					set_commission {
						pool_id: ::core::primitive::u32,
						new_commission: ::core::option::Option<(
							runtime_types::sp_arithmetic::per_things::Perbill,
							::subxt::utils::AccountId32,
						)>,
					},
					#[codec(index = 18)]
					#[doc = "See [`Pallet::set_commission_max`]."]
					set_commission_max {
						pool_id: ::core::primitive::u32,
						max_commission: runtime_types::sp_arithmetic::per_things::Perbill,
					},
					#[codec(index = 19)]
					#[doc = "See [`Pallet::set_commission_change_rate`]."]
					set_commission_change_rate {
						pool_id: ::core::primitive::u32,
						change_rate: runtime_types::pallet_nomination_pools::CommissionChangeRate<
							::core::primitive::u32,
						>,
					},
					#[codec(index = 20)]
					#[doc = "See [`Pallet::claim_commission`]."]
					claim_commission { pool_id: ::core::primitive::u32 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum DefensiveError {
					#[codec(index = 0)]
					NotEnoughSpaceInUnbondPool,
					#[codec(index = 1)]
					PoolNotFound,
					#[codec(index = 2)]
					RewardPoolNotFound,
					#[codec(index = 3)]
					SubPoolsNotFound,
					#[codec(index = 4)]
					BondedStashKilledPrematurely,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "A (bonded) pool id does not exist."]
					PoolNotFound,
					#[codec(index = 1)]
					#[doc = "An account is not a member."]
					PoolMemberNotFound,
					#[codec(index = 2)]
					#[doc = "A reward pool does not exist. In all cases this is a system logic error."]
					RewardPoolNotFound,
					#[codec(index = 3)]
					#[doc = "A sub pool does not exist."]
					SubPoolsNotFound,
					#[codec(index = 4)]
					#[doc = "An account is already delegating in another pool. An account may only belong to one"]
					#[doc = "pool at a time."]
					AccountBelongsToOtherPool,
					#[codec(index = 5)]
					#[doc = "The member is fully unbonded (and thus cannot access the bonded and reward pool"]
					#[doc = "anymore to, for example, collect rewards)."]
					FullyUnbonding,
					#[codec(index = 6)]
					#[doc = "The member cannot unbond further chunks due to reaching the limit."]
					MaxUnbondingLimit,
					#[codec(index = 7)]
					#[doc = "None of the funds can be withdrawn yet because the bonding duration has not passed."]
					CannotWithdrawAny,
					#[codec(index = 8)]
					#[doc = "The amount does not meet the minimum bond to either join or create a pool."]
					#[doc = ""]
					#[doc = "The depositor can never unbond to a value less than"]
					#[doc = "`Pallet::depositor_min_bond`. The caller does not have nominating"]
					#[doc = "permissions for the pool. Members can never unbond to a value below `MinJoinBond`."]
					MinimumBondNotMet,
					#[codec(index = 9)]
					#[doc = "The transaction could not be executed due to overflow risk for the pool."]
					OverflowRisk,
					#[codec(index = 10)]
					#[doc = "A pool must be in [`PoolState::Destroying`] in order for the depositor to unbond or for"]
					#[doc = "other members to be permissionlessly unbonded."]
					NotDestroying,
					#[codec(index = 11)]
					#[doc = "The caller does not have nominating permissions for the pool."]
					NotNominator,
					#[codec(index = 12)]
					#[doc = "Either a) the caller cannot make a valid kick or b) the pool is not destroying."]
					NotKickerOrDestroying,
					#[codec(index = 13)]
					#[doc = "The pool is not open to join"]
					NotOpen,
					#[codec(index = 14)]
					#[doc = "The system is maxed out on pools."]
					MaxPools,
					#[codec(index = 15)]
					#[doc = "Too many members in the pool or system."]
					MaxPoolMembers,
					#[codec(index = 16)]
					#[doc = "The pools state cannot be changed."]
					CanNotChangeState,
					#[codec(index = 17)]
					#[doc = "The caller does not have adequate permissions."]
					DoesNotHavePermission,
					#[codec(index = 18)]
					#[doc = "Metadata exceeds [`Config::MaxMetadataLen`]"]
					MetadataExceedsMaxLen,
					#[codec(index = 19)]
					#[doc = "Some error occurred that should never happen. This should be reported to the"]
					#[doc = "maintainers."]
					Defensive(runtime_types::pallet_nomination_pools::pallet::DefensiveError),
					#[codec(index = 20)]
					#[doc = "Partial unbonding now allowed permissionlessly."]
					PartialUnbondNotAllowedPermissionlessly,
					#[codec(index = 21)]
					#[doc = "The pool's max commission cannot be set higher than the existing value."]
					MaxCommissionRestricted,
					#[codec(index = 22)]
					#[doc = "The supplied commission exceeds the max allowed commission."]
					CommissionExceedsMaximum,
					#[codec(index = 23)]
					#[doc = "The supplied commission exceeds global maximum commission."]
					CommissionExceedsGlobalMaximum,
					#[codec(index = 24)]
					#[doc = "Not enough blocks have surpassed since the last commission update."]
					CommissionChangeThrottled,
					#[codec(index = 25)]
					#[doc = "The submitted changes to commission change rate are not allowed."]
					CommissionChangeRateNotAllowed,
					#[codec(index = 26)]
					#[doc = "There is no pending commission to claim."]
					NoPendingCommission,
					#[codec(index = 27)]
					#[doc = "No commission current has been set."]
					NoCommissionCurrentSet,
					#[codec(index = 28)]
					#[doc = "Pool id currently in use."]
					PoolIdInUse,
					#[codec(index = 29)]
					#[doc = "Pool id provided is not correct/usable."]
					InvalidPoolId,
					#[codec(index = 30)]
					#[doc = "Bonding extra is restricted to the exact pending reward amount."]
					BondExtraRestricted,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Events of this pallet."]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A pool has been created."]
					Created {
						depositor: ::subxt::utils::AccountId32,
						pool_id: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "A member has became bonded in a pool."]
					Bonded {
						member: ::subxt::utils::AccountId32,
						pool_id: ::core::primitive::u32,
						bonded: ::core::primitive::u128,
						joined: ::core::primitive::bool,
					},
					#[codec(index = 2)]
					#[doc = "A payout has been made to a member."]
					PaidOut {
						member: ::subxt::utils::AccountId32,
						pool_id: ::core::primitive::u32,
						payout: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "A member has unbonded from their pool."]
					#[doc = ""]
					#[doc = "- `balance` is the corresponding balance of the number of points that has been"]
					#[doc = "  requested to be unbonded (the argument of the `unbond` transaction) from the bonded"]
					#[doc = "  pool."]
					#[doc = "- `points` is the number of points that are issued as a result of `balance` being"]
					#[doc = "dissolved into the corresponding unbonding pool."]
					#[doc = "- `era` is the era in which the balance will be unbonded."]
					#[doc = "In the absence of slashing, these values will match. In the presence of slashing, the"]
					#[doc = "number of points that are issued in the unbonding pool will be less than the amount"]
					#[doc = "requested to be unbonded."]
					Unbonded {
						member: ::subxt::utils::AccountId32,
						pool_id: ::core::primitive::u32,
						balance: ::core::primitive::u128,
						points: ::core::primitive::u128,
						era: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					#[doc = "A member has withdrawn from their pool."]
					#[doc = ""]
					#[doc = "The given number of `points` have been dissolved in return of `balance`."]
					#[doc = ""]
					#[doc = "Similar to `Unbonded` event, in the absence of slashing, the ratio of point to balance"]
					#[doc = "will be 1."]
					Withdrawn {
						member: ::subxt::utils::AccountId32,
						pool_id: ::core::primitive::u32,
						balance: ::core::primitive::u128,
						points: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					#[doc = "A pool has been destroyed."]
					Destroyed { pool_id: ::core::primitive::u32 },
					#[codec(index = 6)]
					#[doc = "The state of a pool has changed"]
					StateChanged {
						pool_id: ::core::primitive::u32,
						new_state: runtime_types::pallet_nomination_pools::PoolState,
					},
					#[codec(index = 7)]
					#[doc = "A member has been removed from a pool."]
					#[doc = ""]
					#[doc = "The removal can be voluntary (withdrawn all unbonded funds) or involuntary (kicked)."]
					MemberRemoved {
						pool_id: ::core::primitive::u32,
						member: ::subxt::utils::AccountId32,
					},
					#[codec(index = 8)]
					#[doc = "The roles of a pool have been updated to the given new roles. Note that the depositor"]
					#[doc = "can never change."]
					RolesUpdated {
						root: ::core::option::Option<::subxt::utils::AccountId32>,
						bouncer: ::core::option::Option<::subxt::utils::AccountId32>,
						nominator: ::core::option::Option<::subxt::utils::AccountId32>,
					},
					#[codec(index = 9)]
					#[doc = "The active balance of pool `pool_id` has been slashed to `balance`."]
					PoolSlashed {
						pool_id: ::core::primitive::u32,
						balance: ::core::primitive::u128,
					},
					#[codec(index = 10)]
					#[doc = "The unbond pool at `era` of pool `pool_id` has been slashed to `balance`."]
					UnbondingPoolSlashed {
						pool_id: ::core::primitive::u32,
						era: ::core::primitive::u32,
						balance: ::core::primitive::u128,
					},
					#[codec(index = 11)]
					#[doc = "A pool's commission setting has been changed."]
					PoolCommissionUpdated {
						pool_id: ::core::primitive::u32,
						current: ::core::option::Option<(
							runtime_types::sp_arithmetic::per_things::Perbill,
							::subxt::utils::AccountId32,
						)>,
					},
					#[codec(index = 12)]
					#[doc = "A pool's maximum commission setting has been changed."]
					PoolMaxCommissionUpdated {
						pool_id: ::core::primitive::u32,
						max_commission: runtime_types::sp_arithmetic::per_things::Perbill,
					},
					#[codec(index = 13)]
					#[doc = "A pool's commission `change_rate` has been changed."]
					PoolCommissionChangeRateUpdated {
						pool_id: ::core::primitive::u32,
						change_rate: runtime_types::pallet_nomination_pools::CommissionChangeRate<
							::core::primitive::u32,
						>,
					},
					#[codec(index = 14)]
					#[doc = "Pool commission has been claimed."]
					PoolCommissionClaimed {
						pool_id: ::core::primitive::u32,
						commission: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum BondExtra<_0> {
				#[codec(index = 0)]
				FreeBalance(_0),
				#[codec(index = 1)]
				Rewards,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BondedPoolInner {
				pub commission: runtime_types::pallet_nomination_pools::Commission,
				pub member_counter: ::core::primitive::u32,
				pub points: ::core::primitive::u128,
				pub roles:
					runtime_types::pallet_nomination_pools::PoolRoles<::subxt::utils::AccountId32>,
				pub state: runtime_types::pallet_nomination_pools::PoolState,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum ClaimPermission {
				#[codec(index = 0)]
				Permissioned,
				#[codec(index = 1)]
				PermissionlessCompound,
				#[codec(index = 2)]
				PermissionlessWithdraw,
				#[codec(index = 3)]
				PermissionlessAll,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Commission {
				pub current: ::core::option::Option<(
					runtime_types::sp_arithmetic::per_things::Perbill,
					::subxt::utils::AccountId32,
				)>,
				pub max: ::core::option::Option<runtime_types::sp_arithmetic::per_things::Perbill>,
				pub change_rate: ::core::option::Option<
					runtime_types::pallet_nomination_pools::CommissionChangeRate<
						::core::primitive::u32,
					>,
				>,
				pub throttle_from: ::core::option::Option<::core::primitive::u32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CommissionChangeRate<_0> {
				pub max_increase: runtime_types::sp_arithmetic::per_things::Perbill,
				pub min_delay: _0,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum ConfigOp<_0> {
				#[codec(index = 0)]
				Noop,
				#[codec(index = 1)]
				Set(_0),
				#[codec(index = 2)]
				Remove,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct PoolMember {
				pub pool_id: ::core::primitive::u32,
				pub points: ::core::primitive::u128,
				pub last_recorded_reward_counter:
					runtime_types::sp_arithmetic::fixed_point::FixedU128,
				pub unbonding_eras:
					runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
						::core::primitive::u32,
						::core::primitive::u128,
					>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct PoolRoles<_0> {
				pub depositor: _0,
				pub root: ::core::option::Option<_0>,
				pub nominator: ::core::option::Option<_0>,
				pub bouncer: ::core::option::Option<_0>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum PoolState {
				#[codec(index = 0)]
				Open,
				#[codec(index = 1)]
				Blocked,
				#[codec(index = 2)]
				Destroying,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RewardPool {
				pub last_recorded_reward_counter:
					runtime_types::sp_arithmetic::fixed_point::FixedU128,
				pub last_recorded_total_payouts: ::core::primitive::u128,
				pub total_rewards_claimed: ::core::primitive::u128,
				pub total_commission_pending: ::core::primitive::u128,
				pub total_commission_claimed: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SubPools {
				pub no_era: runtime_types::pallet_nomination_pools::UnbondPool,
				pub with_era:
					runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
						::core::primitive::u32,
						runtime_types::pallet_nomination_pools::UnbondPool,
					>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UnbondPool {
				pub points: ::core::primitive::u128,
				pub balance: ::core::primitive::u128,
			}
		}
		pub mod pallet_offences {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Events type."]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "There is an offence reported of the given `kind` happened at the `session_index` and"]
					#[doc = "(kind-specific) time slot. This event is not deposited for duplicate slashes."]
					#[doc = "\\[kind, timeslot\\]."]
					Offence {
						kind: [::core::primitive::u8; 16usize],
						timeslot: ::std::vec::Vec<::core::primitive::u8>,
					},
				}
			}
		}
		pub mod pallet_preimage {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::note_preimage`]."]
					note_preimage {
						bytes: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::unnote_preimage`]."]
					unnote_preimage { hash: ::subxt::utils::H256 },
					#[codec(index = 2)]
					#[doc = "See [`Pallet::request_preimage`]."]
					request_preimage { hash: ::subxt::utils::H256 },
					#[codec(index = 3)]
					#[doc = "See [`Pallet::unrequest_preimage`]."]
					unrequest_preimage { hash: ::subxt::utils::H256 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Preimage is too large to store on-chain."]
					TooBig,
					#[codec(index = 1)]
					#[doc = "Preimage has already been noted on-chain."]
					AlreadyNoted,
					#[codec(index = 2)]
					#[doc = "The user is not authorized to perform this action."]
					NotAuthorized,
					#[codec(index = 3)]
					#[doc = "The preimage cannot be removed since it has not yet been noted."]
					NotNoted,
					#[codec(index = 4)]
					#[doc = "A preimage may not be removed when there are outstanding requests."]
					Requested,
					#[codec(index = 5)]
					#[doc = "The preimage request cannot be removed since no outstanding requests exist."]
					NotRequested,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A preimage has been noted."]
					Noted { hash: ::subxt::utils::H256 },
					#[codec(index = 1)]
					#[doc = "A preimage has been requested."]
					Requested { hash: ::subxt::utils::H256 },
					#[codec(index = 2)]
					#[doc = "A preimage has ben cleared."]
					Cleared { hash: ::subxt::utils::H256 },
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RequestStatus<_0, _1> {
				#[codec(index = 0)]
				Unrequested {
					deposit: (_0, _1),
					len: ::core::primitive::u32,
				},
				#[codec(index = 1)]
				Requested {
					deposit: ::core::option::Option<(_0, _1)>,
					count: ::core::primitive::u32,
					len: ::core::option::Option<::core::primitive::u32>,
				},
			}
		}
		pub mod pallet_scheduler {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::schedule`]."]
					schedule {
						when: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::cancel`]."]
					cancel {
						when: ::core::primitive::u32,
						index: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::schedule_named`]."]
					schedule_named {
						id: [::core::primitive::u8; 32usize],
						when: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::cancel_named`]."]
					cancel_named {
						id: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 4)]
					#[doc = "See [`Pallet::schedule_after`]."]
					schedule_after {
						after: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					},
					#[codec(index = 5)]
					#[doc = "See [`Pallet::schedule_named_after`]."]
					schedule_named_after {
						id: [::core::primitive::u8; 32usize],
						after: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Failed to schedule a call"]
					FailedToSchedule,
					#[codec(index = 1)]
					#[doc = "Cannot find the scheduled call."]
					NotFound,
					#[codec(index = 2)]
					#[doc = "Given target block number is in the past."]
					TargetBlockNumberInPast,
					#[codec(index = 3)]
					#[doc = "Reschedule failed because it does not change scheduled time."]
					RescheduleNoChange,
					#[codec(index = 4)]
					#[doc = "Attempt to use a non-named function on a named task."]
					Named,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Events type."]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Scheduled some task."]
					Scheduled {
						when: ::core::primitive::u32,
						index: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "Canceled some task."]
					Canceled {
						when: ::core::primitive::u32,
						index: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Dispatched some task."]
					Dispatched {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 3)]
					#[doc = "The call for the provided hash was not found so the task has been aborted."]
					CallUnavailable {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec(index = 4)]
					#[doc = "The given task was unable to be renewed since the agenda is full at that block."]
					PeriodicFailed {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec(index = 5)]
					#[doc = "The given task can never be executed since it is overweight."]
					PermanentlyOverweight {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Scheduled<_0, _1, _2, _3, _4> {
				pub maybe_id: ::core::option::Option<_0>,
				pub priority: ::core::primitive::u8,
				pub call: _1,
				pub maybe_periodic: ::core::option::Option<(_2, _2)>,
				pub origin: _3,
				#[codec(skip)]
				pub __subxt_unused_type_params: ::core::marker::PhantomData<_4>,
			}
		}
		pub mod pallet_session {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::set_keys`]."]
					set_keys {
						keys: runtime_types::da_runtime::primitives::SessionKeys,
						proof: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::purge_keys`]."]
					purge_keys,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Error for the session pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Invalid ownership proof."]
					InvalidProof,
					#[codec(index = 1)]
					#[doc = "No associated validator ID for account."]
					NoAssociatedValidatorId,
					#[codec(index = 2)]
					#[doc = "Registered duplicate key."]
					DuplicatedKey,
					#[codec(index = 3)]
					#[doc = "No keys are associated with this account."]
					NoKeys,
					#[codec(index = 4)]
					#[doc = "Key setting account is not live, so it's impossible to associate keys."]
					NoAccount,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New session has happened. Note that the argument is the session index, not the"]
					#[doc = "block number as the type might suggest."]
					NewSession {
						session_index: ::core::primitive::u32,
					},
				}
			}
		}
		pub mod pallet_staking {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
					pub enum Call {
						#[codec(index = 0)]
						#[doc = "See [`Pallet::bond`]."]
						bond {
							#[codec(compact)]
							value: ::core::primitive::u128,
							payee: runtime_types::pallet_staking::RewardDestination<
								::subxt::utils::AccountId32,
							>,
						},
						#[codec(index = 1)]
						#[doc = "See [`Pallet::bond_extra`]."]
						bond_extra {
							#[codec(compact)]
							max_additional: ::core::primitive::u128,
						},
						#[codec(index = 2)]
						#[doc = "See [`Pallet::unbond`]."]
						unbond {
							#[codec(compact)]
							value: ::core::primitive::u128,
						},
						#[codec(index = 3)]
						#[doc = "See [`Pallet::withdraw_unbonded`]."]
						withdraw_unbonded {
							num_slashing_spans: ::core::primitive::u32,
						},
						#[codec(index = 4)]
						#[doc = "See [`Pallet::validate`]."]
						validate {
							prefs: runtime_types::pallet_staking::ValidatorPrefs,
						},
						#[codec(index = 5)]
						#[doc = "See [`Pallet::nominate`]."]
						nominate {
							targets: ::std::vec::Vec<
								::subxt::utils::MultiAddress<
									::subxt::utils::AccountId32,
									::core::primitive::u32,
								>,
							>,
						},
						#[codec(index = 6)]
						#[doc = "See [`Pallet::chill`]."]
						chill,
						#[codec(index = 7)]
						#[doc = "See [`Pallet::set_payee`]."]
						set_payee {
							payee: runtime_types::pallet_staking::RewardDestination<
								::subxt::utils::AccountId32,
							>,
						},
						#[codec(index = 8)]
						#[doc = "See [`Pallet::set_controller`]."]
						set_controller,
						#[codec(index = 9)]
						#[doc = "See [`Pallet::set_validator_count`]."]
						set_validator_count {
							#[codec(compact)]
							new: ::core::primitive::u32,
						},
						#[codec(index = 10)]
						#[doc = "See [`Pallet::increase_validator_count`]."]
						increase_validator_count {
							#[codec(compact)]
							additional: ::core::primitive::u32,
						},
						#[codec(index = 11)]
						#[doc = "See [`Pallet::scale_validator_count`]."]
						scale_validator_count {
							factor: runtime_types::sp_arithmetic::per_things::Percent,
						},
						#[codec(index = 12)]
						#[doc = "See [`Pallet::force_no_eras`]."]
						force_no_eras,
						#[codec(index = 13)]
						#[doc = "See [`Pallet::force_new_era`]."]
						force_new_era,
						#[codec(index = 14)]
						#[doc = "See [`Pallet::set_invulnerables`]."]
						set_invulnerables {
							invulnerables: ::std::vec::Vec<::subxt::utils::AccountId32>,
						},
						#[codec(index = 15)]
						#[doc = "See [`Pallet::force_unstake`]."]
						force_unstake {
							stash: ::subxt::utils::AccountId32,
							num_slashing_spans: ::core::primitive::u32,
						},
						#[codec(index = 16)]
						#[doc = "See [`Pallet::force_new_era_always`]."]
						force_new_era_always,
						#[codec(index = 17)]
						#[doc = "See [`Pallet::cancel_deferred_slash`]."]
						cancel_deferred_slash {
							era: ::core::primitive::u32,
							slash_indices: ::std::vec::Vec<::core::primitive::u32>,
						},
						#[codec(index = 18)]
						#[doc = "See [`Pallet::payout_stakers`]."]
						payout_stakers {
							validator_stash: ::subxt::utils::AccountId32,
							era: ::core::primitive::u32,
						},
						#[codec(index = 19)]
						#[doc = "See [`Pallet::rebond`]."]
						rebond {
							#[codec(compact)]
							value: ::core::primitive::u128,
						},
						#[codec(index = 20)]
						#[doc = "See [`Pallet::reap_stash`]."]
						reap_stash {
							stash: ::subxt::utils::AccountId32,
							num_slashing_spans: ::core::primitive::u32,
						},
						#[codec(index = 21)]
						#[doc = "See [`Pallet::kick`]."]
						kick {
							who: ::std::vec::Vec<
								::subxt::utils::MultiAddress<
									::subxt::utils::AccountId32,
									::core::primitive::u32,
								>,
							>,
						},
						#[codec(index = 22)]
						#[doc = "See [`Pallet::set_staking_configs`]."]
						set_staking_configs {
							min_nominator_bond:
								runtime_types::pallet_staking::pallet::pallet::ConfigOp<
									::core::primitive::u128,
								>,
							min_validator_bond:
								runtime_types::pallet_staking::pallet::pallet::ConfigOp<
									::core::primitive::u128,
								>,
							max_nominator_count:
								runtime_types::pallet_staking::pallet::pallet::ConfigOp<
									::core::primitive::u32,
								>,
							max_validator_count:
								runtime_types::pallet_staking::pallet::pallet::ConfigOp<
									::core::primitive::u32,
								>,
							chill_threshold:
								runtime_types::pallet_staking::pallet::pallet::ConfigOp<
									runtime_types::sp_arithmetic::per_things::Percent,
								>,
							min_commission: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
								runtime_types::sp_arithmetic::per_things::Perbill,
							>,
						},
						#[codec(index = 23)]
						#[doc = "See [`Pallet::chill_other`]."]
						chill_other {
							controller: ::subxt::utils::AccountId32,
						},
						#[codec(index = 24)]
						#[doc = "See [`Pallet::force_apply_min_commission`]."]
						force_apply_min_commission {
							validator_stash: ::subxt::utils::AccountId32,
						},
						#[codec(index = 25)]
						#[doc = "See [`Pallet::set_min_commission`]."]
						set_min_commission {
							new: runtime_types::sp_arithmetic::per_things::Perbill,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum ConfigOp<_0> {
						#[codec(index = 0)]
						Noop,
						#[codec(index = 1)]
						Set(_0),
						#[codec(index = 2)]
						Remove,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					#[doc = "The `Error` enum of this pallet."]
					pub enum Error {
						#[codec(index = 0)]
						#[doc = "Not a controller account."]
						NotController,
						#[codec(index = 1)]
						#[doc = "Not a stash account."]
						NotStash,
						#[codec(index = 2)]
						#[doc = "Stash is already bonded."]
						AlreadyBonded,
						#[codec(index = 3)]
						#[doc = "Controller is already paired."]
						AlreadyPaired,
						#[codec(index = 4)]
						#[doc = "Targets cannot be empty."]
						EmptyTargets,
						#[codec(index = 5)]
						#[doc = "Duplicate index."]
						DuplicateIndex,
						#[codec(index = 6)]
						#[doc = "Slash record index out of bounds."]
						InvalidSlashIndex,
						#[codec(index = 7)]
						#[doc = "Cannot have a validator or nominator role, with value less than the minimum defined by"]
						#[doc = "governance (see `MinValidatorBond` and `MinNominatorBond`). If unbonding is the"]
						#[doc = "intention, `chill` first to remove one's role as validator/nominator."]
						InsufficientBond,
						#[codec(index = 8)]
						#[doc = "Can not schedule more unlock chunks."]
						NoMoreChunks,
						#[codec(index = 9)]
						#[doc = "Can not rebond without unlocking chunks."]
						NoUnlockChunk,
						#[codec(index = 10)]
						#[doc = "Attempting to target a stash that still has funds."]
						FundedTarget,
						#[codec(index = 11)]
						#[doc = "Invalid era to reward."]
						InvalidEraToReward,
						#[codec(index = 12)]
						#[doc = "Invalid number of nominations."]
						InvalidNumberOfNominations,
						#[codec(index = 13)]
						#[doc = "Items are not sorted and unique."]
						NotSortedAndUnique,
						#[codec(index = 14)]
						#[doc = "Rewards for this era have already been claimed for this validator."]
						AlreadyClaimed,
						#[codec(index = 15)]
						#[doc = "Incorrect previous history depth input provided."]
						IncorrectHistoryDepth,
						#[codec(index = 16)]
						#[doc = "Incorrect number of slashing spans provided."]
						IncorrectSlashingSpans,
						#[codec(index = 17)]
						#[doc = "Internal state has become somehow corrupted and the operation cannot continue."]
						BadState,
						#[codec(index = 18)]
						#[doc = "Too many nomination targets supplied."]
						TooManyTargets,
						#[codec(index = 19)]
						#[doc = "A nomination target was supplied that was blocked or otherwise not a validator."]
						BadTarget,
						#[codec(index = 20)]
						#[doc = "The user has enough bond and thus cannot be chilled forcefully by an external person."]
						CannotChillOther,
						#[codec(index = 21)]
						#[doc = "There are too many nominators in the system. Governance needs to adjust the staking"]
						#[doc = "settings to keep things safe for the runtime."]
						TooManyNominators,
						#[codec(index = 22)]
						#[doc = "There are too many validator candidates in the system. Governance needs to adjust the"]
						#[doc = "staking settings to keep things safe for the runtime."]
						TooManyValidators,
						#[codec(index = 23)]
						#[doc = "Commission is too low. Must be at least `MinCommission`."]
						CommissionTooLow,
						#[codec(index = 24)]
						#[doc = "Some bound is not met."]
						BoundNotMet,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					#[doc = "The `Event` enum of this pallet"]
					pub enum Event {
						#[codec(index = 0)]
						#[doc = "The era payout has been set; the first balance is the validator-payout; the second is"]
						#[doc = "the remainder from the maximum amount of reward."]
						EraPaid {
							era_index: ::core::primitive::u32,
							validator_payout: ::core::primitive::u128,
							remainder: ::core::primitive::u128,
						},
						#[codec(index = 1)]
						#[doc = "The nominator has been rewarded by this amount."]
						Rewarded {
							stash: ::subxt::utils::AccountId32,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 2)]
						#[doc = "A staker (validator or nominator) has been slashed by the given amount."]
						Slashed {
							staker: ::subxt::utils::AccountId32,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 3)]
						#[doc = "A slash for the given validator, for the given percentage of their stake, at the given"]
						#[doc = "era as been reported."]
						SlashReported {
							validator: ::subxt::utils::AccountId32,
							fraction: runtime_types::sp_arithmetic::per_things::Perbill,
							slash_era: ::core::primitive::u32,
						},
						#[codec(index = 4)]
						#[doc = "An old slashing report from a prior era was discarded because it could"]
						#[doc = "not be processed."]
						OldSlashingReportDiscarded {
							session_index: ::core::primitive::u32,
						},
						#[codec(index = 5)]
						#[doc = "A new set of stakers was elected."]
						StakersElected,
						#[codec(index = 6)]
						#[doc = "An account has bonded this amount. \\[stash, amount\\]"]
						#[doc = ""]
						#[doc = "NOTE: This event is only emitted when funds are bonded via a dispatchable. Notably,"]
						#[doc = "it will not be emitted for staking rewards when they are added to stake."]
						Bonded {
							stash: ::subxt::utils::AccountId32,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 7)]
						#[doc = "An account has unbonded this amount."]
						Unbonded {
							stash: ::subxt::utils::AccountId32,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 8)]
						#[doc = "An account has called `withdraw_unbonded` and removed unbonding chunks worth `Balance`"]
						#[doc = "from the unlocking queue."]
						Withdrawn {
							stash: ::subxt::utils::AccountId32,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 9)]
						#[doc = "A nominator has been kicked from a validator."]
						Kicked {
							nominator: ::subxt::utils::AccountId32,
							stash: ::subxt::utils::AccountId32,
						},
						#[codec(index = 10)]
						#[doc = "The election failed. No new era is planned."]
						StakingElectionFailed,
						#[codec(index = 11)]
						#[doc = "An account has stopped participating as either a validator or nominator."]
						Chilled { stash: ::subxt::utils::AccountId32 },
						#[codec(index = 12)]
						#[doc = "The stakers' rewards are getting paid."]
						PayoutStarted {
							era_index: ::core::primitive::u32,
							validator_stash: ::subxt::utils::AccountId32,
						},
						#[codec(index = 13)]
						#[doc = "A validator has set their preferences."]
						ValidatorPrefsSet {
							stash: ::subxt::utils::AccountId32,
							prefs: runtime_types::pallet_staking::ValidatorPrefs,
						},
						#[codec(index = 14)]
						#[doc = "A new force era mode was set."]
						ForceEra {
							mode: runtime_types::pallet_staking::Forcing,
						},
					}
				}
			}
			pub mod slashing {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SlashingSpans {
					pub span_index: ::core::primitive::u32,
					pub last_start: ::core::primitive::u32,
					pub last_nonzero_slash: ::core::primitive::u32,
					pub prior: ::std::vec::Vec<::core::primitive::u32>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SpanRecord<_0> {
					pub slashed: _0,
					pub paid_out: _0,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ActiveEraInfo {
				pub index: ::core::primitive::u32,
				pub start: ::core::option::Option<::core::primitive::u64>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct EraRewardPoints<_0> {
				pub total: ::core::primitive::u32,
				pub individual: ::subxt::utils::KeyedVec<_0, ::core::primitive::u32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Exposure<_0, _1> {
				#[codec(compact)]
				pub total: _1,
				#[codec(compact)]
				pub own: _1,
				pub others:
					::std::vec::Vec<runtime_types::pallet_staking::IndividualExposure<_0, _1>>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Forcing {
				#[codec(index = 0)]
				NotForcing,
				#[codec(index = 1)]
				ForceNew,
				#[codec(index = 2)]
				ForceNone,
				#[codec(index = 3)]
				ForceAlways,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct IndividualExposure<_0, _1> {
				pub who: _0,
				#[codec(compact)]
				pub value: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Nominations {
				pub targets: runtime_types::bounded_collections::bounded_vec::BoundedVec<
					::subxt::utils::AccountId32,
				>,
				pub submitted_in: ::core::primitive::u32,
				pub suppressed: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RewardDestination<_0> {
				#[codec(index = 0)]
				Staked,
				#[codec(index = 1)]
				Stash,
				#[codec(index = 2)]
				Controller,
				#[codec(index = 3)]
				Account(_0),
				#[codec(index = 4)]
				None,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct StakingLedger {
				pub stash: ::subxt::utils::AccountId32,
				#[codec(compact)]
				pub total: ::core::primitive::u128,
				#[codec(compact)]
				pub active: ::core::primitive::u128,
				pub unlocking: runtime_types::bounded_collections::bounded_vec::BoundedVec<
					runtime_types::pallet_staking::UnlockChunk<::core::primitive::u128>,
				>,
				pub claimed_rewards: runtime_types::bounded_collections::bounded_vec::BoundedVec<
					::core::primitive::u32,
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UnappliedSlash<_0, _1> {
				pub validator: _0,
				pub own: _1,
				pub others: ::std::vec::Vec<(_0, _1)>,
				pub reporters: ::std::vec::Vec<_0>,
				pub payout: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct UnlockChunk<_0> {
				#[codec(compact)]
				pub value: _0,
				#[codec(compact)]
				pub era: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ValidatorPrefs {
				#[codec(compact)]
				pub commission: runtime_types::sp_arithmetic::per_things::Perbill,
				pub blocked: ::core::primitive::bool,
			}
		}
		pub mod pallet_sudo {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::sudo`]."]
					sudo {
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::sudo_unchecked_weight`]."]
					sudo_unchecked_weight {
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::set_key`]."]
					set_key {
						new: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::sudo_as`]."]
					sudo_as {
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Error for the Sudo pallet"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Sender must be the Sudo account"]
					RequireSudo,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A sudo just took place. \\[result\\]"]
					Sudid {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 1)]
					#[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
					KeyChanged {
						old_sudoer: ::core::option::Option<::subxt::utils::AccountId32>,
					},
					#[codec(index = 2)]
					#[doc = "A sudo just took place. \\[result\\]"]
					SudoAsDone {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
				}
			}
		}
		pub mod pallet_timestamp {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::set`]."]
					set {
						#[codec(compact)]
						now: ::core::primitive::u64,
					},
				}
			}
		}
		pub mod pallet_tips {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::report_awesome`]."]
					report_awesome {
						reason: ::std::vec::Vec<::core::primitive::u8>,
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::retract_tip`]."]
					retract_tip { hash: ::subxt::utils::H256 },
					#[codec(index = 2)]
					#[doc = "See [`Pallet::tip_new`]."]
					tip_new {
						reason: ::std::vec::Vec<::core::primitive::u8>,
						who: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						tip_value: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::tip`]."]
					tip {
						hash: ::subxt::utils::H256,
						#[codec(compact)]
						tip_value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "See [`Pallet::close_tip`]."]
					close_tip { hash: ::subxt::utils::H256 },
					#[codec(index = 5)]
					#[doc = "See [`Pallet::slash_tip`]."]
					slash_tip { hash: ::subxt::utils::H256 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The reason given is just too big."]
					ReasonTooBig,
					#[codec(index = 1)]
					#[doc = "The tip was already found/started."]
					AlreadyKnown,
					#[codec(index = 2)]
					#[doc = "The tip hash is unknown."]
					UnknownTip,
					#[codec(index = 3)]
					#[doc = "The account attempting to retract the tip is not the finder of the tip."]
					NotFinder,
					#[codec(index = 4)]
					#[doc = "The tip cannot be claimed/closed because there are not enough tippers yet."]
					StillOpen,
					#[codec(index = 5)]
					#[doc = "The tip cannot be claimed/closed because it's still in the countdown period."]
					Premature,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A new tip suggestion has been opened."]
					NewTip { tip_hash: ::subxt::utils::H256 },
					#[codec(index = 1)]
					#[doc = "A tip suggestion has reached threshold and is closing."]
					TipClosing { tip_hash: ::subxt::utils::H256 },
					#[codec(index = 2)]
					#[doc = "A tip suggestion has been closed."]
					TipClosed {
						tip_hash: ::subxt::utils::H256,
						who: ::subxt::utils::AccountId32,
						payout: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "A tip suggestion has been retracted."]
					TipRetracted { tip_hash: ::subxt::utils::H256 },
					#[codec(index = 4)]
					#[doc = "A tip suggestion has been slashed."]
					TipSlashed {
						tip_hash: ::subxt::utils::H256,
						finder: ::subxt::utils::AccountId32,
						deposit: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct OpenTip<_0, _1, _2, _3> {
				pub reason: _3,
				pub who: _0,
				pub finder: _0,
				pub deposit: _1,
				pub closes: ::core::option::Option<_2>,
				pub tips: ::std::vec::Vec<(_0, _1)>,
				pub finders_fee: ::core::primitive::bool,
			}
		}
		pub mod pallet_transaction_payment {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
					#[doc = "has been paid by `who`."]
					TransactionFeePaid {
						who: ::subxt::utils::AccountId32,
						actual_fee: ::core::primitive::u128,
						tip: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Releases {
				#[codec(index = 0)]
				V1Ancient,
				#[codec(index = 1)]
				V2,
			}
		}
		pub mod pallet_treasury {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::propose_spend`]."]
					propose_spend {
						#[codec(compact)]
						value: ::core::primitive::u128,
						beneficiary: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::reject_proposal`]."]
					reject_proposal {
						#[codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::approve_proposal`]."]
					approve_proposal {
						#[codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::spend`]."]
					spend {
						#[codec(compact)]
						amount: ::core::primitive::u128,
						beneficiary: ::subxt::utils::MultiAddress<
							::subxt::utils::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 4)]
					#[doc = "See [`Pallet::remove_approval`]."]
					remove_approval {
						#[codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Error for the treasury pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Proposer's balance is too low."]
					InsufficientProposersBalance,
					#[codec(index = 1)]
					#[doc = "No proposal or bounty at that index."]
					InvalidIndex,
					#[codec(index = 2)]
					#[doc = "Too many approvals in the queue."]
					TooManyApprovals,
					#[codec(index = 3)]
					#[doc = "The spend origin is valid but the amount it is allowed to spend is lower than the"]
					#[doc = "amount to be spent."]
					InsufficientPermission,
					#[codec(index = 4)]
					#[doc = "Proposal has not been approved."]
					ProposalNotApproved,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New proposal."]
					Proposed {
						proposal_index: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "We have ended a spend period and will now allocate funds."]
					Spending {
						budget_remaining: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Some funds have been allocated."]
					Awarded {
						proposal_index: ::core::primitive::u32,
						award: ::core::primitive::u128,
						account: ::subxt::utils::AccountId32,
					},
					#[codec(index = 3)]
					#[doc = "A proposal was rejected; funds were slashed."]
					Rejected {
						proposal_index: ::core::primitive::u32,
						slashed: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "Some of our funds have been burnt."]
					Burnt {
						burnt_funds: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					#[doc = "Spending has finished; this is the amount that rolls over until next spend."]
					Rollover {
						rollover_balance: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					#[doc = "Some funds have been deposited."]
					Deposit { value: ::core::primitive::u128 },
					#[codec(index = 7)]
					#[doc = "A new spend proposal has been approved."]
					SpendApproved {
						proposal_index: ::core::primitive::u32,
						amount: ::core::primitive::u128,
						beneficiary: ::subxt::utils::AccountId32,
					},
					#[codec(index = 8)]
					#[doc = "The inactive funds of the pallet have been updated."]
					UpdatedInactive {
						reactivated: ::core::primitive::u128,
						deactivated: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Proposal<_0, _1> {
				pub proposer: _0,
				pub value: _1,
				pub beneficiary: _0,
				pub bond: _1,
			}
		}
		pub mod pallet_utility {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "See [`Pallet::batch`]."]
					batch {
						calls: ::std::vec::Vec<runtime_types::da_runtime::RuntimeCall>,
					},
					#[codec(index = 1)]
					#[doc = "See [`Pallet::as_derivative`]."]
					as_derivative {
						index: ::core::primitive::u16,
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					},
					#[codec(index = 2)]
					#[doc = "See [`Pallet::batch_all`]."]
					batch_all {
						calls: ::std::vec::Vec<runtime_types::da_runtime::RuntimeCall>,
					},
					#[codec(index = 3)]
					#[doc = "See [`Pallet::dispatch_as`]."]
					dispatch_as {
						as_origin: ::std::boxed::Box<runtime_types::da_runtime::OriginCaller>,
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
					},
					#[codec(index = 4)]
					#[doc = "See [`Pallet::force_batch`]."]
					force_batch {
						calls: ::std::vec::Vec<runtime_types::da_runtime::RuntimeCall>,
					},
					#[codec(index = 5)]
					#[doc = "See [`Pallet::with_weight`]."]
					with_weight {
						call: ::std::boxed::Box<runtime_types::da_runtime::RuntimeCall>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Too many calls batched."]
					TooManyCalls,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Batch of dispatches did not complete fully. Index of first failing dispatch given, as"]
					#[doc = "well as the error."]
					BatchInterrupted {
						index: ::core::primitive::u32,
						error: runtime_types::sp_runtime::DispatchError,
					},
					#[codec(index = 1)]
					#[doc = "Batch of dispatches completed fully with no error."]
					BatchCompleted,
					#[codec(index = 2)]
					#[doc = "Batch of dispatches completed but has errors."]
					BatchCompletedWithErrors,
					#[codec(index = 3)]
					#[doc = "A single item within a Batch of dispatches has completed with no error."]
					ItemCompleted,
					#[codec(index = 4)]
					#[doc = "A single item within a Batch of dispatches has completed with error."]
					ItemFailed {
						error: runtime_types::sp_runtime::DispatchError,
					},
					#[codec(index = 5)]
					#[doc = "A call was dispatched."]
					DispatchedAs {
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
				}
			}
		}
		pub mod primitive_types {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct U256(pub [::core::primitive::u64; 4usize]);
		}
		pub mod sp_arithmetic {
			use super::runtime_types;
			pub mod fixed_point {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct FixedU128(pub ::core::primitive::u128);
			}
			pub mod per_things {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PerU16(pub ::core::primitive::u16);
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Perbill(pub ::core::primitive::u32);
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Percent(pub ::core::primitive::u8);
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Permill(pub ::core::primitive::u32);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum ArithmeticError {
				#[codec(index = 0)]
				Underflow,
				#[codec(index = 1)]
				Overflow,
				#[codec(index = 2)]
				DivisionByZero,
			}
		}
		pub mod sp_authority_discovery {
			use super::runtime_types;
			pub mod app {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Public(pub runtime_types::sp_core::sr25519::Public);
			}
		}
		pub mod sp_consensus_babe {
			use super::runtime_types;
			pub mod app {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Public(pub runtime_types::sp_core::sr25519::Public);
			}
			pub mod digests {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum NextConfigDescriptor {
					#[codec(index = 1)]
					V1 {
						c: (::core::primitive::u64, ::core::primitive::u64),
						allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum PreDigest {
					#[codec(index = 1)]
					Primary(runtime_types::sp_consensus_babe::digests::PrimaryPreDigest),
					#[codec(index = 2)]
					SecondaryPlain(
						runtime_types::sp_consensus_babe::digests::SecondaryPlainPreDigest,
					),
					#[codec(index = 3)]
					SecondaryVRF(runtime_types::sp_consensus_babe::digests::SecondaryVRFPreDigest),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PrimaryPreDigest {
					pub authority_index: ::core::primitive::u32,
					pub slot: runtime_types::sp_consensus_slots::Slot,
					pub vrf_signature: runtime_types::sp_core::sr25519::vrf::VrfSignature,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SecondaryPlainPreDigest {
					pub authority_index: ::core::primitive::u32,
					pub slot: runtime_types::sp_consensus_slots::Slot,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SecondaryVRFPreDigest {
					pub authority_index: ::core::primitive::u32,
					pub slot: runtime_types::sp_consensus_slots::Slot,
					pub vrf_signature: runtime_types::sp_core::sr25519::vrf::VrfSignature,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum AllowedSlots {
				#[codec(index = 0)]
				PrimarySlots,
				#[codec(index = 1)]
				PrimaryAndSecondaryPlainSlots,
				#[codec(index = 2)]
				PrimaryAndSecondaryVRFSlots,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BabeEpochConfiguration {
				pub c: (::core::primitive::u64, ::core::primitive::u64),
				pub allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
			}
		}
		pub mod sp_consensus_grandpa {
			use super::runtime_types;
			pub mod app {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Public(pub runtime_types::sp_core::ed25519::Public);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Equivocation<_0, _1> {
				#[codec(index = 0)]
				Prevote(
					runtime_types::finality_grandpa::Equivocation<
						runtime_types::sp_consensus_grandpa::app::Public,
						runtime_types::finality_grandpa::Prevote<_0, _1>,
						runtime_types::sp_consensus_grandpa::app::Signature,
					>,
				),
				#[codec(index = 1)]
				Precommit(
					runtime_types::finality_grandpa::Equivocation<
						runtime_types::sp_consensus_grandpa::app::Public,
						runtime_types::finality_grandpa::Precommit<_0, _1>,
						runtime_types::sp_consensus_grandpa::app::Signature,
					>,
				),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct EquivocationProof<_0, _1> {
				pub set_id: ::core::primitive::u64,
				pub equivocation: runtime_types::sp_consensus_grandpa::Equivocation<_0, _1>,
			}
		}
		pub mod sp_consensus_slots {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct EquivocationProof<_0, _1> {
				pub offender: _1,
				pub slot: runtime_types::sp_consensus_slots::Slot,
				pub first_header: _0,
				pub second_header: _0,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Slot(pub ::core::primitive::u64);
		}
		pub mod sp_core {
			use super::runtime_types;
			pub mod crypto {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
			}
			pub mod ecdsa {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Signature(pub [::core::primitive::u8; 65usize]);
			}
			pub mod ed25519 {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Public(pub [::core::primitive::u8; 32usize]);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			pub mod sr25519 {
				use super::runtime_types;
				pub mod vrf {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct VrfSignature {
						pub output: [::core::primitive::u8; 32usize],
						pub proof: [::core::primitive::u8; 64usize],
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Public(pub [::core::primitive::u8; 32usize]);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Void {}
		}
		pub mod sp_npos_elections {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ElectionScore {
				pub minimal_stake: ::core::primitive::u128,
				pub sum_stake: ::core::primitive::u128,
				pub sum_stake_squared: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Support<_0> {
				pub total: ::core::primitive::u128,
				pub voters: ::std::vec::Vec<(_0, ::core::primitive::u128)>,
			}
		}
		pub mod sp_runtime {
			use super::runtime_types;
			pub mod generic {
				use super::runtime_types;
				pub mod digest {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Digest {
						pub logs:
							::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum DigestItem {
						#[codec(index = 6)]
						PreRuntime(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 4)]
						Consensus(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 5)]
						Seal(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 0)]
						Other(::std::vec::Vec<::core::primitive::u8>),
						#[codec(index = 8)]
						RuntimeEnvironmentUpdated,
					}
				}
				pub mod era {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Clone,
						Debug,
						Eq,
						PartialEq,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Era {
						#[codec(index = 0)]
						Immortal,
						#[codec(index = 1)]
						Mortal1(::core::primitive::u8),
						#[codec(index = 2)]
						Mortal2(::core::primitive::u8),
						#[codec(index = 3)]
						Mortal3(::core::primitive::u8),
						#[codec(index = 4)]
						Mortal4(::core::primitive::u8),
						#[codec(index = 5)]
						Mortal5(::core::primitive::u8),
						#[codec(index = 6)]
						Mortal6(::core::primitive::u8),
						#[codec(index = 7)]
						Mortal7(::core::primitive::u8),
						#[codec(index = 8)]
						Mortal8(::core::primitive::u8),
						#[codec(index = 9)]
						Mortal9(::core::primitive::u8),
						#[codec(index = 10)]
						Mortal10(::core::primitive::u8),
						#[codec(index = 11)]
						Mortal11(::core::primitive::u8),
						#[codec(index = 12)]
						Mortal12(::core::primitive::u8),
						#[codec(index = 13)]
						Mortal13(::core::primitive::u8),
						#[codec(index = 14)]
						Mortal14(::core::primitive::u8),
						#[codec(index = 15)]
						Mortal15(::core::primitive::u8),
						#[codec(index = 16)]
						Mortal16(::core::primitive::u8),
						#[codec(index = 17)]
						Mortal17(::core::primitive::u8),
						#[codec(index = 18)]
						Mortal18(::core::primitive::u8),
						#[codec(index = 19)]
						Mortal19(::core::primitive::u8),
						#[codec(index = 20)]
						Mortal20(::core::primitive::u8),
						#[codec(index = 21)]
						Mortal21(::core::primitive::u8),
						#[codec(index = 22)]
						Mortal22(::core::primitive::u8),
						#[codec(index = 23)]
						Mortal23(::core::primitive::u8),
						#[codec(index = 24)]
						Mortal24(::core::primitive::u8),
						#[codec(index = 25)]
						Mortal25(::core::primitive::u8),
						#[codec(index = 26)]
						Mortal26(::core::primitive::u8),
						#[codec(index = 27)]
						Mortal27(::core::primitive::u8),
						#[codec(index = 28)]
						Mortal28(::core::primitive::u8),
						#[codec(index = 29)]
						Mortal29(::core::primitive::u8),
						#[codec(index = 30)]
						Mortal30(::core::primitive::u8),
						#[codec(index = 31)]
						Mortal31(::core::primitive::u8),
						#[codec(index = 32)]
						Mortal32(::core::primitive::u8),
						#[codec(index = 33)]
						Mortal33(::core::primitive::u8),
						#[codec(index = 34)]
						Mortal34(::core::primitive::u8),
						#[codec(index = 35)]
						Mortal35(::core::primitive::u8),
						#[codec(index = 36)]
						Mortal36(::core::primitive::u8),
						#[codec(index = 37)]
						Mortal37(::core::primitive::u8),
						#[codec(index = 38)]
						Mortal38(::core::primitive::u8),
						#[codec(index = 39)]
						Mortal39(::core::primitive::u8),
						#[codec(index = 40)]
						Mortal40(::core::primitive::u8),
						#[codec(index = 41)]
						Mortal41(::core::primitive::u8),
						#[codec(index = 42)]
						Mortal42(::core::primitive::u8),
						#[codec(index = 43)]
						Mortal43(::core::primitive::u8),
						#[codec(index = 44)]
						Mortal44(::core::primitive::u8),
						#[codec(index = 45)]
						Mortal45(::core::primitive::u8),
						#[codec(index = 46)]
						Mortal46(::core::primitive::u8),
						#[codec(index = 47)]
						Mortal47(::core::primitive::u8),
						#[codec(index = 48)]
						Mortal48(::core::primitive::u8),
						#[codec(index = 49)]
						Mortal49(::core::primitive::u8),
						#[codec(index = 50)]
						Mortal50(::core::primitive::u8),
						#[codec(index = 51)]
						Mortal51(::core::primitive::u8),
						#[codec(index = 52)]
						Mortal52(::core::primitive::u8),
						#[codec(index = 53)]
						Mortal53(::core::primitive::u8),
						#[codec(index = 54)]
						Mortal54(::core::primitive::u8),
						#[codec(index = 55)]
						Mortal55(::core::primitive::u8),
						#[codec(index = 56)]
						Mortal56(::core::primitive::u8),
						#[codec(index = 57)]
						Mortal57(::core::primitive::u8),
						#[codec(index = 58)]
						Mortal58(::core::primitive::u8),
						#[codec(index = 59)]
						Mortal59(::core::primitive::u8),
						#[codec(index = 60)]
						Mortal60(::core::primitive::u8),
						#[codec(index = 61)]
						Mortal61(::core::primitive::u8),
						#[codec(index = 62)]
						Mortal62(::core::primitive::u8),
						#[codec(index = 63)]
						Mortal63(::core::primitive::u8),
						#[codec(index = 64)]
						Mortal64(::core::primitive::u8),
						#[codec(index = 65)]
						Mortal65(::core::primitive::u8),
						#[codec(index = 66)]
						Mortal66(::core::primitive::u8),
						#[codec(index = 67)]
						Mortal67(::core::primitive::u8),
						#[codec(index = 68)]
						Mortal68(::core::primitive::u8),
						#[codec(index = 69)]
						Mortal69(::core::primitive::u8),
						#[codec(index = 70)]
						Mortal70(::core::primitive::u8),
						#[codec(index = 71)]
						Mortal71(::core::primitive::u8),
						#[codec(index = 72)]
						Mortal72(::core::primitive::u8),
						#[codec(index = 73)]
						Mortal73(::core::primitive::u8),
						#[codec(index = 74)]
						Mortal74(::core::primitive::u8),
						#[codec(index = 75)]
						Mortal75(::core::primitive::u8),
						#[codec(index = 76)]
						Mortal76(::core::primitive::u8),
						#[codec(index = 77)]
						Mortal77(::core::primitive::u8),
						#[codec(index = 78)]
						Mortal78(::core::primitive::u8),
						#[codec(index = 79)]
						Mortal79(::core::primitive::u8),
						#[codec(index = 80)]
						Mortal80(::core::primitive::u8),
						#[codec(index = 81)]
						Mortal81(::core::primitive::u8),
						#[codec(index = 82)]
						Mortal82(::core::primitive::u8),
						#[codec(index = 83)]
						Mortal83(::core::primitive::u8),
						#[codec(index = 84)]
						Mortal84(::core::primitive::u8),
						#[codec(index = 85)]
						Mortal85(::core::primitive::u8),
						#[codec(index = 86)]
						Mortal86(::core::primitive::u8),
						#[codec(index = 87)]
						Mortal87(::core::primitive::u8),
						#[codec(index = 88)]
						Mortal88(::core::primitive::u8),
						#[codec(index = 89)]
						Mortal89(::core::primitive::u8),
						#[codec(index = 90)]
						Mortal90(::core::primitive::u8),
						#[codec(index = 91)]
						Mortal91(::core::primitive::u8),
						#[codec(index = 92)]
						Mortal92(::core::primitive::u8),
						#[codec(index = 93)]
						Mortal93(::core::primitive::u8),
						#[codec(index = 94)]
						Mortal94(::core::primitive::u8),
						#[codec(index = 95)]
						Mortal95(::core::primitive::u8),
						#[codec(index = 96)]
						Mortal96(::core::primitive::u8),
						#[codec(index = 97)]
						Mortal97(::core::primitive::u8),
						#[codec(index = 98)]
						Mortal98(::core::primitive::u8),
						#[codec(index = 99)]
						Mortal99(::core::primitive::u8),
						#[codec(index = 100)]
						Mortal100(::core::primitive::u8),
						#[codec(index = 101)]
						Mortal101(::core::primitive::u8),
						#[codec(index = 102)]
						Mortal102(::core::primitive::u8),
						#[codec(index = 103)]
						Mortal103(::core::primitive::u8),
						#[codec(index = 104)]
						Mortal104(::core::primitive::u8),
						#[codec(index = 105)]
						Mortal105(::core::primitive::u8),
						#[codec(index = 106)]
						Mortal106(::core::primitive::u8),
						#[codec(index = 107)]
						Mortal107(::core::primitive::u8),
						#[codec(index = 108)]
						Mortal108(::core::primitive::u8),
						#[codec(index = 109)]
						Mortal109(::core::primitive::u8),
						#[codec(index = 110)]
						Mortal110(::core::primitive::u8),
						#[codec(index = 111)]
						Mortal111(::core::primitive::u8),
						#[codec(index = 112)]
						Mortal112(::core::primitive::u8),
						#[codec(index = 113)]
						Mortal113(::core::primitive::u8),
						#[codec(index = 114)]
						Mortal114(::core::primitive::u8),
						#[codec(index = 115)]
						Mortal115(::core::primitive::u8),
						#[codec(index = 116)]
						Mortal116(::core::primitive::u8),
						#[codec(index = 117)]
						Mortal117(::core::primitive::u8),
						#[codec(index = 118)]
						Mortal118(::core::primitive::u8),
						#[codec(index = 119)]
						Mortal119(::core::primitive::u8),
						#[codec(index = 120)]
						Mortal120(::core::primitive::u8),
						#[codec(index = 121)]
						Mortal121(::core::primitive::u8),
						#[codec(index = 122)]
						Mortal122(::core::primitive::u8),
						#[codec(index = 123)]
						Mortal123(::core::primitive::u8),
						#[codec(index = 124)]
						Mortal124(::core::primitive::u8),
						#[codec(index = 125)]
						Mortal125(::core::primitive::u8),
						#[codec(index = 126)]
						Mortal126(::core::primitive::u8),
						#[codec(index = 127)]
						Mortal127(::core::primitive::u8),
						#[codec(index = 128)]
						Mortal128(::core::primitive::u8),
						#[codec(index = 129)]
						Mortal129(::core::primitive::u8),
						#[codec(index = 130)]
						Mortal130(::core::primitive::u8),
						#[codec(index = 131)]
						Mortal131(::core::primitive::u8),
						#[codec(index = 132)]
						Mortal132(::core::primitive::u8),
						#[codec(index = 133)]
						Mortal133(::core::primitive::u8),
						#[codec(index = 134)]
						Mortal134(::core::primitive::u8),
						#[codec(index = 135)]
						Mortal135(::core::primitive::u8),
						#[codec(index = 136)]
						Mortal136(::core::primitive::u8),
						#[codec(index = 137)]
						Mortal137(::core::primitive::u8),
						#[codec(index = 138)]
						Mortal138(::core::primitive::u8),
						#[codec(index = 139)]
						Mortal139(::core::primitive::u8),
						#[codec(index = 140)]
						Mortal140(::core::primitive::u8),
						#[codec(index = 141)]
						Mortal141(::core::primitive::u8),
						#[codec(index = 142)]
						Mortal142(::core::primitive::u8),
						#[codec(index = 143)]
						Mortal143(::core::primitive::u8),
						#[codec(index = 144)]
						Mortal144(::core::primitive::u8),
						#[codec(index = 145)]
						Mortal145(::core::primitive::u8),
						#[codec(index = 146)]
						Mortal146(::core::primitive::u8),
						#[codec(index = 147)]
						Mortal147(::core::primitive::u8),
						#[codec(index = 148)]
						Mortal148(::core::primitive::u8),
						#[codec(index = 149)]
						Mortal149(::core::primitive::u8),
						#[codec(index = 150)]
						Mortal150(::core::primitive::u8),
						#[codec(index = 151)]
						Mortal151(::core::primitive::u8),
						#[codec(index = 152)]
						Mortal152(::core::primitive::u8),
						#[codec(index = 153)]
						Mortal153(::core::primitive::u8),
						#[codec(index = 154)]
						Mortal154(::core::primitive::u8),
						#[codec(index = 155)]
						Mortal155(::core::primitive::u8),
						#[codec(index = 156)]
						Mortal156(::core::primitive::u8),
						#[codec(index = 157)]
						Mortal157(::core::primitive::u8),
						#[codec(index = 158)]
						Mortal158(::core::primitive::u8),
						#[codec(index = 159)]
						Mortal159(::core::primitive::u8),
						#[codec(index = 160)]
						Mortal160(::core::primitive::u8),
						#[codec(index = 161)]
						Mortal161(::core::primitive::u8),
						#[codec(index = 162)]
						Mortal162(::core::primitive::u8),
						#[codec(index = 163)]
						Mortal163(::core::primitive::u8),
						#[codec(index = 164)]
						Mortal164(::core::primitive::u8),
						#[codec(index = 165)]
						Mortal165(::core::primitive::u8),
						#[codec(index = 166)]
						Mortal166(::core::primitive::u8),
						#[codec(index = 167)]
						Mortal167(::core::primitive::u8),
						#[codec(index = 168)]
						Mortal168(::core::primitive::u8),
						#[codec(index = 169)]
						Mortal169(::core::primitive::u8),
						#[codec(index = 170)]
						Mortal170(::core::primitive::u8),
						#[codec(index = 171)]
						Mortal171(::core::primitive::u8),
						#[codec(index = 172)]
						Mortal172(::core::primitive::u8),
						#[codec(index = 173)]
						Mortal173(::core::primitive::u8),
						#[codec(index = 174)]
						Mortal174(::core::primitive::u8),
						#[codec(index = 175)]
						Mortal175(::core::primitive::u8),
						#[codec(index = 176)]
						Mortal176(::core::primitive::u8),
						#[codec(index = 177)]
						Mortal177(::core::primitive::u8),
						#[codec(index = 178)]
						Mortal178(::core::primitive::u8),
						#[codec(index = 179)]
						Mortal179(::core::primitive::u8),
						#[codec(index = 180)]
						Mortal180(::core::primitive::u8),
						#[codec(index = 181)]
						Mortal181(::core::primitive::u8),
						#[codec(index = 182)]
						Mortal182(::core::primitive::u8),
						#[codec(index = 183)]
						Mortal183(::core::primitive::u8),
						#[codec(index = 184)]
						Mortal184(::core::primitive::u8),
						#[codec(index = 185)]
						Mortal185(::core::primitive::u8),
						#[codec(index = 186)]
						Mortal186(::core::primitive::u8),
						#[codec(index = 187)]
						Mortal187(::core::primitive::u8),
						#[codec(index = 188)]
						Mortal188(::core::primitive::u8),
						#[codec(index = 189)]
						Mortal189(::core::primitive::u8),
						#[codec(index = 190)]
						Mortal190(::core::primitive::u8),
						#[codec(index = 191)]
						Mortal191(::core::primitive::u8),
						#[codec(index = 192)]
						Mortal192(::core::primitive::u8),
						#[codec(index = 193)]
						Mortal193(::core::primitive::u8),
						#[codec(index = 194)]
						Mortal194(::core::primitive::u8),
						#[codec(index = 195)]
						Mortal195(::core::primitive::u8),
						#[codec(index = 196)]
						Mortal196(::core::primitive::u8),
						#[codec(index = 197)]
						Mortal197(::core::primitive::u8),
						#[codec(index = 198)]
						Mortal198(::core::primitive::u8),
						#[codec(index = 199)]
						Mortal199(::core::primitive::u8),
						#[codec(index = 200)]
						Mortal200(::core::primitive::u8),
						#[codec(index = 201)]
						Mortal201(::core::primitive::u8),
						#[codec(index = 202)]
						Mortal202(::core::primitive::u8),
						#[codec(index = 203)]
						Mortal203(::core::primitive::u8),
						#[codec(index = 204)]
						Mortal204(::core::primitive::u8),
						#[codec(index = 205)]
						Mortal205(::core::primitive::u8),
						#[codec(index = 206)]
						Mortal206(::core::primitive::u8),
						#[codec(index = 207)]
						Mortal207(::core::primitive::u8),
						#[codec(index = 208)]
						Mortal208(::core::primitive::u8),
						#[codec(index = 209)]
						Mortal209(::core::primitive::u8),
						#[codec(index = 210)]
						Mortal210(::core::primitive::u8),
						#[codec(index = 211)]
						Mortal211(::core::primitive::u8),
						#[codec(index = 212)]
						Mortal212(::core::primitive::u8),
						#[codec(index = 213)]
						Mortal213(::core::primitive::u8),
						#[codec(index = 214)]
						Mortal214(::core::primitive::u8),
						#[codec(index = 215)]
						Mortal215(::core::primitive::u8),
						#[codec(index = 216)]
						Mortal216(::core::primitive::u8),
						#[codec(index = 217)]
						Mortal217(::core::primitive::u8),
						#[codec(index = 218)]
						Mortal218(::core::primitive::u8),
						#[codec(index = 219)]
						Mortal219(::core::primitive::u8),
						#[codec(index = 220)]
						Mortal220(::core::primitive::u8),
						#[codec(index = 221)]
						Mortal221(::core::primitive::u8),
						#[codec(index = 222)]
						Mortal222(::core::primitive::u8),
						#[codec(index = 223)]
						Mortal223(::core::primitive::u8),
						#[codec(index = 224)]
						Mortal224(::core::primitive::u8),
						#[codec(index = 225)]
						Mortal225(::core::primitive::u8),
						#[codec(index = 226)]
						Mortal226(::core::primitive::u8),
						#[codec(index = 227)]
						Mortal227(::core::primitive::u8),
						#[codec(index = 228)]
						Mortal228(::core::primitive::u8),
						#[codec(index = 229)]
						Mortal229(::core::primitive::u8),
						#[codec(index = 230)]
						Mortal230(::core::primitive::u8),
						#[codec(index = 231)]
						Mortal231(::core::primitive::u8),
						#[codec(index = 232)]
						Mortal232(::core::primitive::u8),
						#[codec(index = 233)]
						Mortal233(::core::primitive::u8),
						#[codec(index = 234)]
						Mortal234(::core::primitive::u8),
						#[codec(index = 235)]
						Mortal235(::core::primitive::u8),
						#[codec(index = 236)]
						Mortal236(::core::primitive::u8),
						#[codec(index = 237)]
						Mortal237(::core::primitive::u8),
						#[codec(index = 238)]
						Mortal238(::core::primitive::u8),
						#[codec(index = 239)]
						Mortal239(::core::primitive::u8),
						#[codec(index = 240)]
						Mortal240(::core::primitive::u8),
						#[codec(index = 241)]
						Mortal241(::core::primitive::u8),
						#[codec(index = 242)]
						Mortal242(::core::primitive::u8),
						#[codec(index = 243)]
						Mortal243(::core::primitive::u8),
						#[codec(index = 244)]
						Mortal244(::core::primitive::u8),
						#[codec(index = 245)]
						Mortal245(::core::primitive::u8),
						#[codec(index = 246)]
						Mortal246(::core::primitive::u8),
						#[codec(index = 247)]
						Mortal247(::core::primitive::u8),
						#[codec(index = 248)]
						Mortal248(::core::primitive::u8),
						#[codec(index = 249)]
						Mortal249(::core::primitive::u8),
						#[codec(index = 250)]
						Mortal250(::core::primitive::u8),
						#[codec(index = 251)]
						Mortal251(::core::primitive::u8),
						#[codec(index = 252)]
						Mortal252(::core::primitive::u8),
						#[codec(index = 253)]
						Mortal253(::core::primitive::u8),
						#[codec(index = 254)]
						Mortal254(::core::primitive::u8),
						#[codec(index = 255)]
						Mortal255(::core::primitive::u8),
					}
				}
			}
			pub mod traits {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BlakeTwo256;
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum DispatchError {
				#[codec(index = 0)]
				Other,
				#[codec(index = 1)]
				CannotLookup,
				#[codec(index = 2)]
				BadOrigin,
				#[codec(index = 3)]
				Module(runtime_types::sp_runtime::ModuleError),
				#[codec(index = 4)]
				ConsumerRemaining,
				#[codec(index = 5)]
				NoProviders,
				#[codec(index = 6)]
				TooManyConsumers,
				#[codec(index = 7)]
				Token(runtime_types::sp_runtime::TokenError),
				#[codec(index = 8)]
				Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
				#[codec(index = 9)]
				Transactional(runtime_types::sp_runtime::TransactionalError),
				#[codec(index = 10)]
				Exhausted,
				#[codec(index = 11)]
				Corruption,
				#[codec(index = 12)]
				Unavailable,
				#[codec(index = 13)]
				RootNotAllowed,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ModuleError {
				pub index: ::core::primitive::u8,
				pub error: [::core::primitive::u8; 4usize],
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum MultiSignature {
				#[codec(index = 0)]
				Ed25519(runtime_types::sp_core::ed25519::Signature),
				#[codec(index = 1)]
				Sr25519(runtime_types::sp_core::sr25519::Signature),
				#[codec(index = 2)]
				Ecdsa(runtime_types::sp_core::ecdsa::Signature),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum TokenError {
				#[codec(index = 0)]
				FundsUnavailable,
				#[codec(index = 1)]
				OnlyProvider,
				#[codec(index = 2)]
				BelowMinimum,
				#[codec(index = 3)]
				CannotCreate,
				#[codec(index = 4)]
				UnknownAsset,
				#[codec(index = 5)]
				Frozen,
				#[codec(index = 6)]
				Unsupported,
				#[codec(index = 7)]
				CannotCreateHold,
				#[codec(index = 8)]
				NotExpendable,
				#[codec(index = 9)]
				Blocked,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum TransactionalError {
				#[codec(index = 0)]
				LimitReached,
				#[codec(index = 1)]
				NoLayer,
			}
		}
		pub mod sp_session {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MembershipProof {
				pub session: ::core::primitive::u32,
				pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				pub validator_count: ::core::primitive::u32,
			}
		}
		pub mod sp_staking {
			use super::runtime_types;
			pub mod offence {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct OffenceDetails<_0, _1> {
					pub offender: _1,
					pub reporters: ::std::vec::Vec<_0>,
				}
			}
		}
		pub mod sp_version {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RuntimeVersion {
				pub spec_name: ::std::string::String,
				pub impl_name: ::std::string::String,
				pub authoring_version: ::core::primitive::u32,
				pub spec_version: ::core::primitive::u32,
				pub impl_version: ::core::primitive::u32,
				pub apis:
					::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
				pub transaction_version: ::core::primitive::u32,
				pub state_version: ::core::primitive::u8,
			}
		}
		pub mod sp_weights {
			use super::runtime_types;
			pub mod weight_v2 {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Clone,
					Debug,
					Eq,
					PartialEq,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Weight {
					#[codec(compact)]
					pub ref_time: ::core::primitive::u64,
					#[codec(compact)]
					pub proof_size: ::core::primitive::u64,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Clone,
				Debug,
				Eq,
				PartialEq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RuntimeDbWeight {
				pub read: ::core::primitive::u64,
				pub write: ::core::primitive::u64,
			}
		}
	}
}
