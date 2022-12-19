use crate::asdr::AppId;

/// Get application Id trait
pub trait GetAppId {
	fn app_id(&self) -> AppId {
		Default::default()
	}
}

impl<A, B, C, D, E, F, G, H: GetAppId> GetAppId for (A, B, C, D, E, F, G, H) {
	fn app_id(&self) -> AppId {
		self.7.app_id()
	}
}

impl<A, B, C, D, E, F, G, H, I: GetAppId> GetAppId for (A, B, C, D, E, F, G, H, I) {
	fn app_id(&self) -> AppId {
		self.8.app_id()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::asdr::AppId;

	struct CustomAppId {}

	impl GetAppId for CustomAppId {
		fn app_id(&self) -> AppId {
			7.into()
		}
	}

	struct DefaultGetAppId {}
	impl GetAppId for DefaultGetAppId {}

	#[test]
	fn app_id_trait_on_tuples() {
		let custom_app_id = (0, 1, 2, 3, 4, 5, 6, CustomAppId {});
		let default_app_id = (0, 1, 2, 3, 4, 5, 6, DefaultGetAppId {});

		assert_eq!(custom_app_id.app_id(), 7.into());
		assert_eq!(default_app_id.app_id(), Default::default());
	}
}
