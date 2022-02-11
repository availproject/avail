/// Get application Id trait
pub trait GetAppId<T: Default> {
	fn app_id(&self) -> T { T::default() }
}

impl<A, B, C, D, E, F, G, H: GetAppId<u32>> GetAppId<u32> for (A, B, C, D, E, F, G, H) {
	fn app_id(&self) -> u32 { self.7.app_id() }
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::asdr::AppId;

	struct CustomAppId {}

	impl GetAppId<AppId> for CustomAppId {
		fn app_id(&self) -> AppId { 7 }
	}

	struct DefaultGetAppId {}
	impl<T: Default> GetAppId<T> for DefaultGetAppId {}

	#[test]
	fn app_id_trait_on_tuples() {
		let custom_app_id = (0, 1, 2, 3, 4, 5, 6, CustomAppId {});
		let default_app_id = (0, 1, 2, 3, 4, 5, 6, DefaultGetAppId {});

		assert_eq!(custom_app_id.app_id(), 7);
		assert_eq!(default_app_id.app_id(), u32::default());
	}
}
