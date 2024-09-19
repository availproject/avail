use sdk_core::types::{avail, H256};

#[derive(Clone, Copy)]
pub struct Extra {
	nonce: Option<Nonce>,
	mortality: Option<Mortality>,
	tip: Option<avail::Tip>,
	app_id: Option<avail::AppId>,
}
impl Extra {
	pub fn new() -> Self {
		Self {
			nonce: None,
			mortality: None,
			tip: None,
			app_id: None,
		}
	}

	pub fn nonce(mut self, value: Nonce) -> Self {
		self.nonce = Some(value);
		self
	}

	pub fn mortality(mut self, value: Mortality) -> Self {
		self.mortality = Some(value);
		self
	}

	pub fn tip(mut self, value: avail::Tip) -> Self {
		self.tip = Some(value);
		self
	}

	pub fn app_id(mut self, value: avail::AppId) -> Self {
		self.app_id = Some(value);
		self
	}

	pub fn deconstruct(
		self,
	) -> (
		Option<Nonce>,
		Option<Mortality>,
		Option<avail::Tip>,
		Option<avail::AppId>,
	) {
		(self.nonce, self.mortality, self.tip, self.app_id)
	}
}

#[derive(Clone, Copy)]
pub enum Nonce {
	BestBlock,
	FinalizedBlock,
	BestBlockAndTxPool,
	Custom(u32),
}

#[derive(Clone, Copy)]
pub enum Mortality {
	Period(avail::Period),
	Custom((avail::Period, avail::BlockNumber, H256)),
}
