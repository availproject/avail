use crate::{interface::BlockHash, transaction::Period, BlockNumber};

pub type Tip = u128;
pub type AppId = u32;

#[derive(Clone, Copy)]
pub enum Nonce {
	BestBlockAndTxPool,
	BestBlock,
	FinalizedBlock,
	Custom(u32),
}

#[derive(Clone, Copy)]
pub enum Mortality {
	Period(Period),
	Custom((Period, BlockNumber, BlockHash)),
}

#[derive(Clone, Copy)]
pub struct ExtrinsicExtra {
	nonce: Option<Nonce>,
	mortality: Option<Mortality>,
	tip: Option<Tip>,
	app_id: Option<AppId>,
}

impl ExtrinsicExtra {
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

	pub fn tip(mut self, value: Tip) -> Self {
		self.tip = Some(value);
		self
	}

	pub fn app_id(mut self, value: AppId) -> Self {
		self.app_id = Some(value);
		self
	}

	pub fn deconstruct(self) -> (Option<Nonce>, Option<Mortality>, Option<Tip>, Option<AppId>) {
		(self.nonce, self.mortality, self.tip, self.app_id)
	}
}
