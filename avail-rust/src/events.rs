/* use crate::avail;
use crate::config::AvailConfig;
use subxt::blocks::ExtrinsicEvents;

pub mod balances {
	use super::*;
	use avail::balances::events as BalancesEvents;

	pub struct TransferEvent {
		pub from: String,
		pub to: String,
		pub amount: String,
	}

	impl TransferEvent {
		pub fn new(events: &ExtrinsicEvents<AvailConfig>) -> Result<Self, String> {
			let Ok(event) = events.find_first::<BalancesEvents::Transfer>() else {
				return Err(String::from("Failed to find Transfer event."));
			};

			let Some(event) = event else {
				return Err(String::from("Event was None for some reason."));
			};

			let event = Self {
				from: event.from.to_string(),
				to: event.to.to_string(),
				amount: event.amount.to_string(),
			};

			Ok(event)
		}
	}
}

pub mod system {
	use super::*;
	use avail::system::events as SystemEvents;

	pub struct KilledAccount {
		pub account: String,
	}

	impl KilledAccount {
		pub fn new(events: &ExtrinsicEvents<AvailConfig>) -> Result<Self, String> {
			let Ok(event) = events.find_first::<SystemEvents::KilledAccount>() else {
				return Err(String::from("Failed to find KilledAccount event."));
			};

			let Some(event) = event else {
				return Err(String::from("Event was None for some reason."));
			};

			let event = Self {
				account: event.account.to_string(),
			};

			Ok(event)
		}
	}
}
 */
