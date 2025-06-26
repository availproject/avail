pub(crate) mod v1 {
	pub(crate) mod da_sampling {
		include!(concat!(env!("OUT_DIR"), "/api.v1.sampling.rs"));
	}
}
