#[cfg(feature = "wasm")]
mod wasm_exports {
	use da_commitment::build_da_commitments::*;
	use kate::Seed;
	use wasm_bindgen::prelude::*;

	#[wasm_bindgen(start)]
	pub fn _start() {
		console_error_panic_hook::set_once();
		let _ = wasm_logger::init(wasm_logger::Config::default());
		log::info!("da-commitment wasm initialized");
	}

	#[wasm_bindgen]
	pub fn build_commitments_js(data: &[u8], max_width: usize, max_height: usize) -> Vec<u8> {
		log::info!(
			"build_commitments_js called (len={}, w={}, h={})",
			data.len(),
			max_width,
			max_height
		);
		build_da_commitments(data, max_width, max_height, Seed::default())
	}
}
