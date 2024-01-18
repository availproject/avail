/// Macro to set a value (e.g. when using the `parameter_types` macro) to either a production value
/// or to an environment variable or testing value (in case the `goldberg` feature is selected)
/// or one of two testing values depending on feature.
/// Note that the environment variable is evaluated _at compile time_.
///
/// Usage:
/// ```Rust
/// parameter_types! {
/// 	// Note that the env variable version parameter cannot be const.
/// 	pub LaunchPeriod: BlockNumber = prod_or_test!(7 * DAYS, 1, "AVL_LAUNCH_PERIOD");
/// 	pub const VotingPeriod: BlockNumber = prod_or_test!(7 * DAYS, 1 * MINUTES);
/// 	pub const EpochDuration: BlockNumber =
/// 		prod_or_test!(1 * HOURS, "fast-runtime", 1 * MINUTES, "fast-runtime-10m", 10 * MINUTES);
/// }
/// ```
#[macro_export]
macro_rules! prod_or_test {
	($prod:expr, $test:expr) => {
		if cfg!(feature = "goldberg") {
			$test
		} else {
			$prod
		}
	};
	($prod:expr, $test:expr, $env:expr) => {
		if cfg!(feature = "goldberg") {
			core::option_env!($env)
				.map(|s| s.parse().ok())
				.flatten()
				.unwrap_or($test)
		} else {
			$prod
		}
	};
}
