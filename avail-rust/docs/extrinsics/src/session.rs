use avail_rust::error::ClientError;

pub async fn run() -> Result<(), ClientError> {
	println!("session_set_key");
	set_keys::run().await?;

	Ok(())
}

mod set_keys {
	use avail_rust::{
		error::ClientError, transactions::SessionCalls, utils, Keypair, Nonce, Options, SecretUri,
		SDK,
	};
	use core::str::FromStr;

	pub async fn run() -> Result<(), ClientError> {
		let sdk = SDK::new(SDK::local_endpoint()).await?;

		// Input
		let secret_uri = SecretUri::from_str("//Alice//stash").unwrap();
		let account = Keypair::from_uri(&secret_uri).unwrap();
		let keys = sdk.rpc.author.rotate_keys().await?;
		let keys = utils::deconstruct_session_keys(keys)?;

		let options = Some(Options::new().nonce(Nonce::BestBlockAndTxPool));
		let tx = sdk.tx.session.set_keys(keys);
		let result = tx.execute_wait_for_inclusion(&account, options).await?;

		dbg!(&result);
		if let Some(data) = result
			.get_data::<SessionCalls::SetKeys>(&sdk.online_client)
			.await
		{
			dbg!(data);
		}

		Ok(())
	}
}
