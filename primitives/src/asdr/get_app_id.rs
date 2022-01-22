use impl_trait_for_tuples::impl_for_tuples;
use sp_std::ops::Add;

/// Get application Id trait
pub trait GetAppId<T: Default> {
	fn app_id(&self) -> T { T::default() }
}

/// Tuple implementation for `GetAppId`.
#[impl_for_tuples(1, 9)]
impl<T: Default + Add<Output = T>> GetAppId<T> for Tuple {
	fn app_id(&self) -> T { for_tuples!( #( Tuple.app_id())+* ) }
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
		assert_eq!(CustomAppId {}.app_id(), 7);
		assert_eq!(GetAppId::<AppId>::app_id(&DefaultGetAppId {}), 0);

		assert_eq!((CustomAppId {}).app_id(), 7);
		assert_eq!((CustomAppId {}, DefaultGetAppId {}).app_id(), 7);
		assert_eq!(
			GetAppId::<AppId>::app_id(&(
				DefaultGetAppId {},
				DefaultGetAppId {},
				DefaultGetAppId {}
			)),
			0
		);
	}
}
