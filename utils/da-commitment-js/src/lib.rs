#[cfg(feature = "wasm")]
mod wasm_exports {
	use avail_fri::{
		core::{FriBiniusPCS, B128},
		encoding::BytesEncoder,
		eval_utils::{derive_evaluation_point, derive_seed_from_inputs, eval_claim_to_bytes},
		FriParamsVersion,
	};
	use wasm_bindgen::prelude::*;

	fn parse_fri_params_version(params_version: u32) -> Result<FriParamsVersion, JsValue> {
		match params_version {
			0 => Ok(FriParamsVersion::V0),
			_ => Err(JsValue::from_str("unsupported FRI params version")),
		}
	}

	fn to_fixed_32(label: &str, value: &[u8]) -> Result<[u8; 32], JsValue> {
		value
			.try_into()
			.map_err(|_| JsValue::from_str(&format!("{label} must be 32 bytes")))
	}

	#[wasm_bindgen(getter_with_clone)]
	pub struct FriBlobMetadataJs {
		pub commitment: Vec<u8>,
		pub eval_point_seed: Vec<u8>,
		pub eval_claim: Vec<u8>,
	}

	#[wasm_bindgen(start)]
	pub fn _start() {
		console_error_panic_hook::set_once();
		let _ = wasm_logger::init(wasm_logger::Config::default());
		log::info!("da-commitment wasm initialized");
	}

	#[wasm_bindgen]
	pub fn build_commitments_js(
		data: &[u8],
		params_version: u32,
		babe_randomness: &[u8],
		blob_hash: &[u8],
	) -> Result<FriBlobMetadataJs, JsValue> {
		log::info!(
			"build_commitments_js called (len={}, params_version={})",
			data.len(),
			params_version
		);

		let params_version = parse_fri_params_version(params_version)?;
		let babe_randomness = to_fixed_32("babe_randomness", babe_randomness)?;
		let blob_hash = to_fixed_32("blob_hash", blob_hash)?;

		let encoder = BytesEncoder::<B128>::new();
		let packed = encoder
			.bytes_to_packed_mle(data)
			.map_err(|e| JsValue::from_str(&e.to_string()))?;

		let pcs = FriBiniusPCS::new(params_version.to_config(packed.total_n_vars));
		let ctx = pcs
			.initialize_fri_context::<B128>(packed.packed_mle.log_len())
			.map_err(|e| JsValue::from_str(&e.to_string()))?;
		let commit_output = pcs
			.commit(&packed.packed_mle, &ctx)
			.map_err(|e| JsValue::from_str(&e.to_string()))?;

		let eval_point_seed = derive_seed_from_inputs(&babe_randomness, &blob_hash);
		let eval_point = derive_evaluation_point(eval_point_seed, packed.total_n_vars);
		let eval_claim = pcs
			.calculate_evaluation_claim(&packed.packed_values, &eval_point)
			.map_err(|e| JsValue::from_str(&e.to_string()))?;
		let eval_claim: [u8; 16] = eval_claim_to_bytes(eval_claim)
			.try_into()
			.map_err(|_| JsValue::from_str("invalid evaluation claim length"))?;

		Ok(FriBlobMetadataJs {
			commitment: commit_output.commitment.to_vec(),
			eval_point_seed: eval_point_seed.to_vec(),
			eval_claim: eval_claim.to_vec(),
		})
	}
}
