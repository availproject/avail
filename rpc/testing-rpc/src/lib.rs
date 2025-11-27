// use avail_base::testing_env::*;
// // use avail_core::app_extrinsic::AppExtrinsic;
// use jsonrpsee::{
// 	core::{async_trait, RpcResult},
// 	proc_macros::rpc,
// };
// use std::marker::Sync;

// /// # TODO
// #[rpc(client, server)]
// pub trait TestingApi {
// 	#[method(name = "testing_toggleExtensionFailure")]
// 	async fn toggle_extension_failure(&self, value: bool) -> RpcResult<()>;
// 	#[method(name = "testing_toggleGridFailure")]
// 	async fn toggle_grid_failure(&self, value: bool) -> RpcResult<()>;
// 	#[method(name = "testing_toggleCommitmentFailure")]
// 	async fn toggle_commitment_failure(&self, value: bool) -> RpcResult<()>;
// 	#[method(name = "testing_populateGrid")]
// 	async fn populate_grid(&self, value: Option<Vec<AppExtrinsic>>) -> RpcResult<()>;
// }

// pub struct TestingEnv;

// #[async_trait]
// impl TestingApiServer for TestingEnv {
// 	async fn toggle_extension_failure(&self, value: bool) -> RpcResult<()> {
// 		unsafe {
// 			ENABLE_TEST_EXTENSION_FAILURE = value;
// 		}

// 		Ok(())
// 	}
// 	async fn toggle_grid_failure(&self, value: bool) -> RpcResult<()> {
// 		unsafe {
// 			ENABLE_TEST_GRID_FAILURE = value;
// 		}

// 		Ok(())
// 	}
// 	async fn toggle_commitment_failure(&self, value: bool) -> RpcResult<()> {
// 		unsafe {
// 			ENABLE_TEST_COMMITMENT_FAILURE = value;
// 		}

// 		Ok(())
// 	}
// 	async fn populate_grid(&self, value: Option<Vec<AppExtrinsic>>) -> RpcResult<()> {
// 		unsafe {
// 			TEST_POPULATE_GRID = value;
// 		}

// 		Ok(())
// 	}
// }
