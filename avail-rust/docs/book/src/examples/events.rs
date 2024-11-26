use avail_rust::{avail, error::ClientError, utils, SDK};

pub async fn run() -> Result<(), ClientError> {
	let sdk = SDK::new(SDK::local_endpoint()).await?;

	let account = SDK::alice()?;

	let dest = utils::account_id_from_str("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty")?;
	let tx = sdk.tx.balances.transfer_keep_alive(dest, SDK::one_avail());
	let res = tx.execute_wait_for_inclusion(&account, None).await?;

	for event in res.events.iter() {
		let Ok(event) = event else {
			return Ok(());
		};

		println!(
			"Pallet name: {}, Event Name: {}",
			event.pallet_name(),
			event.variant_name()
		);
	}

	let event = res.events.find_first::<avail::balances::events::Transfer>();
	let Some(event) = event.ok().flatten() else {
		return Ok(());
	};

	println!(
		"Transfer from: {}, to: {}, amount: {}",
		event.from, event.to, event.amount
	);

	Ok(())
}
