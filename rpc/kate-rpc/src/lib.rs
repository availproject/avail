use std::{
	result::Result as AvailResult,
	sync::{Arc, RwLock},
};

use codec::{Compact, Decode, Encode, Error as DecodeError, Input};
use da_primitives::asdr::{AppExtrinsic, GetAppId};
use frame_system::limits::BlockLength;
use jsonrpc_core::{Error as RpcError, Result};
use jsonrpc_derive::rpc;
use kate::{BlockDimensions, BlsScalar, PublicParameters};
use kate_rpc_runtime_api::KateParamsGetter;
use lru::LruCache;
use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};
use sc_client_api::{BlockBackend, StorageProvider};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_rpc::number::NumberOrHex;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT, Header, NumberFor},
	AccountId32, MultiAddress, MultiSignature,
};

#[rpc]
pub trait KateApi {
	#[rpc(name = "kate_queryProof")]
	fn query_proof(
		&self,
		block_number: NumberOrHex,
		cells: Vec<kate::com::Cell>,
	) -> Result<Vec<u8>>;

	#[rpc(name = "kate_blockLength")]
	fn query_block_length(&self) -> Result<BlockLength>;

	#[rpc(name = "kate_queryDataProof")]
	fn query_data_proof(&self, block_number: NumberOrHex, index: usize) -> Result<Vec<[u8; 32]>>;
}

pub struct Kate<Client, Block: BlockT> {
	client: Arc<Client>,
	block_ext_cache: RwLock<LruCache<Block::Hash, (Vec<BlsScalar>, BlockDimensions)>>,
}

impl<Client, Block> Kate<Client, Block>
where
	Block: BlockT,
{
	pub fn new(client: Arc<Client>) -> Self {
		Self {
			client,
			block_ext_cache: RwLock::new(LruCache::new(2048)), // 524288 bytes per block, ~1Gb max size
		}
	}
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i64 {
	fn from(e: Error) -> i64 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

macro_rules! internal_err {
	($($arg:tt)*) => {{
		let mut error = RpcError::internal_error();
		error.message = format!($($arg)*);
		error
	}}
}

impl<Client, Block> KateApi for Kate<Client, Block>
where
	Block: BlockT,
	Block::Extrinsic: GetAppId,
	Client: Send + Sync + 'static,
	Client: HeaderBackend<Block>
		+ ProvideRuntimeApi<Block>
		+ BlockBackend<Block>
		+ StorageProvider<Block, sc_client_db::Backend<Block>>,
	Client::Api: KateParamsGetter<Block>,
{
	//TODO allocate static thread pool, just for RPC related work, to free up resources, for the block producing processes.
	fn query_proof(
		&self,
		block_number: NumberOrHex,
		cells: Vec<kate::com::Cell>,
	) -> Result<Vec<u8>> {
		let block_num: u32 = block_number
			.try_into()
			.map_err(|_| RpcError::invalid_params("Invalid block number"))?;

		let block_num = <NumberFor<Block>>::from(block_num);
		let signed_block = self
			.client
			.block(&BlockId::number(block_num))
			.map_err(|e| internal_err!("Invalid block number: {:?}", e))?
			.ok_or_else(|| internal_err!("Missing block number {}", block_num))?;
		let block_hash = signed_block.block.header().hash();
		let block_id = BlockId::hash(block_hash.clone());

		let mut block_ext_cache = self
			.block_ext_cache
			.write()
			.map_err(|_| internal_err!("Block cache lock is poisoned .qed"))?;

		let block_length: BlockLength = self
			.client
			.runtime_api()
			.get_block_length(&block_id)
			.map_err(|e| internal_err!("Block Length cannot be fetched: {:?}", e))?;

		if !block_ext_cache.contains(&block_hash) {
			// build block data extension and cache it
			let xts_by_id: Vec<AppExtrinsic> = signed_block
				.block
				.extrinsics()
				.iter()
				.map(|e| AppExtrinsic {
					app_id: e.app_id(),
					data: e.encode(),
				})
				.collect();

			// Use Babe's VRF
			let seed: [u8; 32] =
				self.client
					.runtime_api()
					.get_babe_vrf(&block_id)
					.map_err(|e| {
						internal_err!("Babe VRF not found for block {}: {:?}", block_num, e)
					})?;

			let (_, block, block_dims) = kate::com::flatten_and_pad_block(
				block_length.rows as usize,
				block_length.cols as usize,
				block_length.chunk_size() as usize,
				&xts_by_id,
				seed,
			)
			.map_err(|e| internal_err!("Flatten and pad block failed: {:?}", e))?;

			let data = kate::com::par_extend_data_matrix(block_dims, &block)
				.map_err(|e| internal_err!("Matrix cannot be extended: {:?}", e))?;
			block_ext_cache.put(block_hash, (data, block_dims));
		}

		let (ext_data, block_dims) = block_ext_cache
			.get(&block_hash)
			.ok_or_else(|| internal_err!("Block hash {} cannot be fetched", block_hash))?;
		let kc_public_params_raw = self
			.client
			.runtime_api()
			.get_public_params(&block_id)
			.map_err(|e| {
				internal_err!(
					"Public params cannot be fetched on block {}: {:?}",
					block_hash,
					e
				)
			})?;
		let kc_public_params =
			unsafe { PublicParameters::from_slice_unchecked(&kc_public_params_raw) };

		let proof = kate::com::build_proof(&kc_public_params, *block_dims, ext_data, &cells)
			.map_err(|e| internal_err!("Proof cannot be generated: {:?}", e))?;

		Ok(proof)
	}

	fn query_block_length(&self) -> Result<BlockLength> {
		let best_hash = self.client.info().best_hash;
		let block_length = self
			.client
			.runtime_api()
			.get_block_length(&BlockId::hash(best_hash))
			.map_err(|e| internal_err!("Length of best block({:?}): {:?}", best_hash, e))?;

		Ok(block_length)
	}

	fn query_data_proof(&self, block_number: NumberOrHex, index: usize) -> Result<Vec<[u8; 32]>> {
		let block_num: u32 = block_number
			.try_into()
			.map_err(|_| RpcError::invalid_params("Invalid block number"))?;

		let block_num = <NumberFor<Block>>::from(block_num);
		let signed_block = self
			.client
			.block(&BlockId::number(block_num))
			.map_err(|e| internal_err!("Invalid block number: {:?}", e))?
			.ok_or_else(|| internal_err!("Missing block number {}", block_num))?;

		let mut leaves: Vec<[u8; 32]> = Vec::new();
		let mut interested_leaf_position: Option<usize> = None;

		for (pos, xt) in signed_block.block.extrinsics().iter().enumerate() {
			let avail_extrinsic = AppExtrinsic {
				app_id: xt.app_id(),
				data: xt.encode(),
			};
			let optional_decoded_xt = <AvailExtrinsic>::decode(&mut &avail_extrinsic.data[..]);

			match optional_decoded_xt {
				Ok(decoded_xt) => leaves.push(Sha256::hash(&decoded_xt.data)),
				Err(_) => continue,
			}
			if pos == index {
				interested_leaf_position = Some(leaves.len() - 1);
			}
		}

		if leaves.len() > 0 {
			if let Some(position) = interested_leaf_position {
				let data_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
				// TODO: Enable assertion
				// assert_eq!(data_tree.root(), signed_block.block.header().extrinsics_root().data_root(), "Wrong data root!");
				let proof = data_tree.proof(&[position]);
				return Ok(proof.proof_hashes().to_vec());
			}
		}
		Err(internal_err!("Proof not possible!"))
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AvailExtrinsic {
	pub app_id: u32,
	pub signature: Option<MultiSignature>,
	pub data: Vec<u8>,
}

pub type AvailSignedExtra = ((), (), (), AvailMortality, Nonce, (), Balance, u32);

#[derive(Decode)]
pub struct Balance(#[codec(compact)] u128);

#[derive(Decode)]
pub struct Nonce(#[codec(compact)] u32);

pub enum AvailMortality {
	Immortal,
	Mortal(u64, u64),
}

impl Decode for AvailMortality {
	fn decode<I: Input>(input: &mut I) -> AvailResult<Self, DecodeError> {
		let first = input.read_byte()?;
		if first == 0 {
			Ok(Self::Immortal)
		} else {
			let encoded = first as u64 + ((input.read_byte()? as u64) << 8);
			let period = 2 << (encoded % (1 << 4));
			let quantize_factor = (period >> 12).max(1);
			let phase = (encoded >> 4) * quantize_factor;
			if period >= 4 && phase < period {
				Ok(Self::Mortal(period, phase))
			} else {
				Err("Invalid period and phase".into())
			}
		}
	}
}

const EXTRINSIC_VERSION: u8 = 4;
impl Decode for AvailExtrinsic {
	fn decode<I: Input>(input: &mut I) -> AvailResult<AvailExtrinsic, DecodeError> {
		// This is a little more complicated than usual since the binary format must be compatible
		// with substrate's generic `Vec<u8>` type. Basically this just means accepting that there
		// will be a prefix of vector length (we don't need
		// to use this).
		let _length_do_not_remove_me_see_above: Compact<u32> = Decode::decode(input)?;

		let version = input.read_byte()?;

		let is_signed = version & 0b1000_0000 != 0;
		let version = version & 0b0111_1111;
		if version != EXTRINSIC_VERSION {
			return Err("Invalid transaction version".into());
		}
		let (app_id, signature) = if is_signed {
			let _address = <MultiAddress<AccountId32, u32>>::decode(input)?;
			let sig = MultiSignature::decode(input)?;
			let extra = <AvailSignedExtra>::decode(input)?;
			let app_id = extra.7;

			(app_id, Some(sig))
		} else {
			return Err("Not signed".into());
		};

		let section: u8 = Decode::decode(input)?;
		let method: u8 = Decode::decode(input)?;

		let data: Vec<u8> = match (section, method) {
			// TODO: Define these pairs as enums or better yet - make a dependency on substrate enums if possible
			(29, 1) => Decode::decode(input)?,
			_ => return Err("Not Avail Extrinsic".into()),
		};

		Ok(Self {
			app_id,
			signature,
			data,
		})
	}
}
