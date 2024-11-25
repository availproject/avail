use avail_rust::error::ClientError;

pub async fn run() -> Result<(), ClientError> {
	println!("session_set_key");
	set_keys::run().await?;

	Ok(())
}

mod set_keys {
	use avail_rust::{error::ClientError, utils, Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice//stash").unwrap();
		let account = Keypair::from_uri(&secret_uri).unwrap();
		let keys = sdk.rpc.author.rotate_keys().await?;
		let keys = utils::deconstruct_session_keys(keys)?;

		let wait_for = WaitFor::BlockInclusion;
		let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
		let result = sdk
			.tx
			.session
			.set_keys(keys, wait_for, &account, Some(options))
			.await;
		let result = result.map_err(|e| e.reason)?;

		dbg!(result);

		Ok(())
	}
}
