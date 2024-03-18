use sp_std::{cell::RefCell, rc::Rc};

pub type RcMetrics = Rc<RefCell<Metrics>>;

/// Information about `submitted_data_root` and `submitted_data_proof` methods.
#[derive(Default, Debug)]
pub struct Metrics {
	/// Number of extrinsics containing one or more submitted data.
	pub data_submit_extrinsics: u32,
	/// Total number of submitted data.
	pub data_submit_leaves: u32,
	/// Total number of bridge leaves.
	pub bridge_leaves: u32,
	/// Total number of analysed extrinsic.
	pub total_extrinsics: u32,
}

impl Metrics {
	/// Creates a shared metric with internal mutability.
	pub fn new_shared() -> RcMetrics {
		Rc::new(RefCell::new(Self::default()))
	}
}
