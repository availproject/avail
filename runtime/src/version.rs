use sp_runtime::create_runtime_str;
use sp_version::RuntimeVersion;

use crate::apis;

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	// The identifier for the different Substrate runtimes.
	spec_name: create_runtime_str!("avail"),
	// The name of the implementation of the spec. This is of little
	// consequence for the node and serves only to differentiate code of
	// different implementation teams.
	impl_name: create_runtime_str!("avail"),
	// The version of the authorship interface. An authoring node will not
	// attempt to author blocks unless this is equal to its native runtime.
	authoring_version: 12,
	// Per convention: if the runtime behavior changes, increment spec_version
	// and set impl_version to 0. This paramenter is typically incremented when
	// there's an update to the transaction_version.
	spec_version: 35,
	// The version of the implementation of the specification. Nodes can ignore this. It is only
	// used to indicate that the code is different. As long as the authoring_version and the
	// spec_version are the same, the code itself might have changed, but the native and Wasm
	// binaries do the same thing. In general, only non-logic-breaking optimizations would result
	// in a change of the impl_version.
	impl_version: 0,
	// The version of the interface for handling transactions. This parameter can
	// be useful to synchronize firmware updates for hardware wallets or other signing
	// devices to verify that runtime transactions are valid. The parameter allows
	// hardware wallets to know which transactions they can safely sign. This number
	// must be bumped if there is a change in the index of the pallets in the construct_runtime!
	// macro or if there are any changes to dispatchable functions, such as the number of
	// parameters or parameter types. If this number is updated, then the spec_version must also
	// be updated.
	transaction_version: 1,
	apis: apis::runtime_api_versions(),
	state_version: 1,
};
