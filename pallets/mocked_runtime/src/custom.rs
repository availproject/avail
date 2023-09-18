pub const TEST_KEY: &[u8] = b":test:key:";

#[frame_support::pallet]
pub mod custom {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		// module hooks.
		// one with block number arg and one without
		fn on_initialize(n: T::BlockNumber) -> Weight {
			println!("on_initialize({})", n);
			Weight::from_parts(175, 0)
		}

		fn on_idle(n: T::BlockNumber, remaining_weight: Weight) -> Weight {
			println!("on_idle{}, {})", n, remaining_weight);
			Weight::from_parts(175, 0)
		}

		fn on_finalize(n: T::BlockNumber) {
			println!("on_finalize({})", n);
		}

		fn on_runtime_upgrade() -> Weight {
			sp_io::storage::set(super::TEST_KEY, "module".as_bytes());
			Weight::from_parts(200, 0)
		}

		fn offchain_worker(n: T::BlockNumber) {
			assert_eq!(T::BlockNumber::from(1u32), n);
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(100)]
		#[pallet::call_index(0)]
		pub fn some_function(origin: OriginFor<T>) -> DispatchResult {
			// NOTE: does not make any different.
			frame_system::ensure_signed(origin)?;
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight((200, DispatchClass::Operational))]
		pub fn some_root_operation(origin: OriginFor<T>) -> DispatchResult {
			frame_system::ensure_root(origin)?;
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn some_unsigned_message(origin: OriginFor<T>) -> DispatchResult {
			frame_system::ensure_none(origin)?;
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn allowed_unsigned(origin: OriginFor<T>) -> DispatchResult {
			frame_system::ensure_root(origin)?;
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(0)]
		pub fn unallowed_unsigned(origin: OriginFor<T>) -> DispatchResult {
			frame_system::ensure_root(origin)?;
			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(0)]
		pub fn inherent_call(origin: OriginFor<T>) -> DispatchResult {
			frame_system::ensure_none(origin)?;
			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(0)]
		pub fn calculate_storage_root(_origin: OriginFor<T>) -> DispatchResult {
			let root = sp_io::storage::root(sp_runtime::StateVersion::V1);
			sp_io::storage::set("storage_root".as_bytes(), &root);
			Ok(())
		}
	}

	#[pallet::inherent]
	impl<T: Config> ProvideInherent for Pallet<T> {
		type Call = Call<T>;
		type Error = sp_inherents::MakeFatalError<()>;

		const INHERENT_IDENTIFIER: [u8; 8] = *b"test1234";

		fn create_inherent(_data: &InherentData) -> Option<Self::Call> { None }

		fn is_inherent(call: &Self::Call) -> bool { *call == Call::<T>::inherent_call {} }
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		// Inherent call is accepted for being dispatched
		fn pre_dispatch(call: &Self::Call) -> Result<(), TransactionValidityError> {
			match call {
				Call::allowed_unsigned { .. } => Ok(()),
				Call::inherent_call { .. } => Ok(()),
				_ => Err(UnknownTransaction::NoUnsignedValidator.into()),
			}
		}

		// Inherent call is not validated as unsigned
		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			match call {
				Call::allowed_unsigned { .. } => Ok(Default::default()),
				_ => UnknownTransaction::NoUnsignedValidator.into(),
			}
		}
	}
}
