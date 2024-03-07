use sp_runtime::create_runtime_str;
use sp_version::RuntimeVersion;

use crate::apis;

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("avail"),
	impl_name: create_runtime_str!("avail"),
	authoring_version: 11,
	spec_version: 23,
	impl_version: 0,
	transaction_version: 1,
	apis: apis::runtime_api_versions(),
	state_version: 1,
};
