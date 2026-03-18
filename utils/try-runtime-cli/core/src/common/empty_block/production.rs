use std::{ops::DerefMut, str::FromStr, sync::Arc};

use array_bytes::Hexify;
use parity_scale_codec::{Decode, Encode};
use sc_cli::Result;
use sc_executor::{HostFunctions, WasmExecutor};
use sp_core::H256;
use sp_inherents::InherentData;
use sp_runtime::{
    traits::{Block as BlockT, HashingFor, Header, NumberFor, One},
    DeserializeOwned, Digest, ExtrinsicInclusionMode,
};
use sp_state_machine::TestExternalities;
use sp_std::fmt::Debug;
use tokio::sync::Mutex;

use super::inherents::{pre_apply::pre_apply_inherents, providers::InherentProvider};
use crate::{
    common::{
        empty_block::inherents::providers::ProviderVariant, misc_logging::LogLevelGuard,
        state::state_machine_call,
    },
    full_extensions,
};

pub async fn mine_block<Block, HostFns: HostFunctions>(
    ext_mutex: Arc<Mutex<TestExternalities<HashingFor<Block>>>>,
    executor: &WasmExecutor<HostFns>,
    previous_block_building_info: Option<(InherentData, Digest)>,
    parent_header: Block::Header,
    provider_variant: ProviderVariant,
    try_state: frame_try_runtime::TryStateSelect,
) -> Result<(
    (InherentData, Digest),
    Block::Header,
    Option<ExtrinsicInclusionMode>,
)>
where
    Block: BlockT<Hash = H256> + DeserializeOwned,
    Block::Header: DeserializeOwned,
    <Block::Hash as FromStr>::Err: Debug,
    NumberFor<Block>: FromStr,
    <NumberFor<Block> as FromStr>::Err: Debug,
{
    // We are saving state before we overwrite it while producing new block.
    let mut ext_guard = ext_mutex.lock().await;
    let ext = ext_guard.deref_mut();
    let backend = ext.as_backend();
    drop(ext_guard);

    log::info!(
        "Producing new empty block at height {:?}",
        *parent_header.number() + One::one()
    );

    // Prevent it from printing all logs twice:
    let muffle = LogLevelGuard::only_errors();
    let (next_block, new_block_building_info, mode) = produce_next_block::<Block, HostFns>(
        ext_mutex.clone(),
        executor,
        parent_header.clone(),
        provider_variant,
        previous_block_building_info,
    )
    .await?;
    drop(muffle);

    log::info!(
        "Produced a new block ({})",
        next_block.header().hash().as_bytes().hexify_prefixed()
    );

    let mut ext_guard = ext_mutex.lock().await;
    let ext = ext_guard.deref_mut();

    // And now we restore previous state.
    ext.backend = backend;

    pre_apply_inherents::<Block>(ext);
    let state_root_check = true;
    let signature_check = true;
    let payload = (
        next_block.clone(),
        state_root_check,
        signature_check,
        try_state.clone(),
    )
        .encode();
    //call::<(), Block, _>(ext, executor, "TryRuntime_execute_block", &payload).await?;

    if try_state == frame_try_runtime::TryStateSelect::None {
        call::<(), Block, _>(ext, executor, "Core_execute_block", &next_block.encode()).await?;
    } else {
        call::<(), Block, _>(ext, executor, "TryRuntime_execute_block", &payload).await?;
    }

    log::info!("Executed the new block");

    Ok((new_block_building_info, next_block.header().clone(), mode))
}

/// Produces next block containing only inherents.
pub async fn produce_next_block<Block, HostFns: HostFunctions>(
    ext_mutex: Arc<Mutex<TestExternalities<HashingFor<Block>>>>,
    executor: &WasmExecutor<HostFns>,
    parent_header: Block::Header,
    chain: ProviderVariant,
    previous_block_building_info: Option<(InherentData, Digest)>,
) -> Result<(
    Block,
    (InherentData, Digest),
    Option<ExtrinsicInclusionMode>,
)>
where
    Block: BlockT<Hash = H256> + DeserializeOwned,
    Block::Header: DeserializeOwned,
    <Block::Hash as FromStr>::Err: Debug,
    NumberFor<Block>: FromStr,
    <NumberFor<Block> as FromStr>::Err: Debug,
{
    let rp_offset = {
        let ext_guard = ext_mutex.lock().await;
        relay_parent_offset::<Block, HostFns>(&ext_guard, executor)
    };

    let (inherent_data_provider, pre_digest) =
        <ProviderVariant as InherentProvider<Block>>::get_inherent_providers_and_pre_digest(
            &chain,
            previous_block_building_info,
            parent_header.clone(),
            ext_mutex.clone(),
            rp_offset,
        )?;

    let mut ext_guard = ext_mutex.lock().await;
    let ext = ext_guard.deref_mut();

    pre_apply_inherents::<Block>(ext);
    drop(ext_guard);

    let inherent_data = inherent_data_provider
        .create_inherent_data()
        .await
        .map_err(|s| sc_cli::Error::Input(s.to_string()))?;
    let digest = Digest { logs: pre_digest };

    let header = Block::Header::new(
        *parent_header.number() + One::one(),
        Default::default(),
        Default::default(),
        parent_header.hash(),
        digest.clone(),
    );

    let mut ext_guard = ext_mutex.lock().await;
    let ext = ext_guard.deref_mut();
    // Only RA API version 5 supports returning a mode, so need to check.
    let mode = if core_version::<Block, HostFns>(ext, executor)? >= 5 {
        let mode = call::<ExtrinsicInclusionMode, Block, _>(
            ext,
            executor,
            "Core_initialize_block",
            &header.encode(),
        )
        .await?;
        Some(mode)
    } else {
        call::<(), Block, _>(ext, executor, "Core_initialize_block", &header.encode()).await?;
        None
    };

    let extrinsics = dry_call::<Vec<Block::Extrinsic>, Block, _>(
        ext,
        executor,
        "BlockBuilder_inherent_extrinsics",
        &inherent_data.encode(),
    )?;

    for xt in &extrinsics {
        call::<(), Block, _>(ext, executor, "BlockBuilder_apply_extrinsic", &xt.encode()).await?;
    }

    let header = dry_call::<Block::Header, Block, _>(
        ext,
        executor,
        "BlockBuilder_finalize_block",
        &[0u8; 0],
    )?;

    call::<(), Block, _>(ext, executor, "BlockBuilder_finalize_block", &[0u8; 0]).await?;
    ext.commit_all().unwrap();
    drop(ext_guard);

    Ok((
        Block::new(header, extrinsics),
        (inherent_data, digest),
        mode,
    ))
}

/// Call `method` with `data` and actually save storage changes to `externalities`.
async fn call<T: Decode, Block: BlockT, HostFns: HostFunctions>(
    externalities: &mut TestExternalities<HashingFor<Block>>,
    executor: &WasmExecutor<HostFns>,
    method: &'static str,
    data: &[u8],
) -> Result<T> {
    let (mut changes, result) = state_machine_call::<Block, HostFns>(
        externalities,
        executor,
        method,
        data,
        full_extensions(executor.clone()),
    )?;

    let storage_changes =
        changes.drain_storage_changes(&externalities.backend, externalities.state_version)?;

    externalities.backend.apply_transaction(
        storage_changes.transaction_storage_root,
        storage_changes.transaction,
    );

    T::decode(&mut &*result).map_err(|e| sc_cli::Error::Input(format!("{:?}", e)))
}

pub fn core_version<Block: BlockT, HostFns: HostFunctions>(
    externalities: &TestExternalities<HashingFor<Block>>,
    executor: &WasmExecutor<HostFns>,
) -> Result<u32> {
    dry_call::<u32, Block, HostFns>(externalities, executor, "Core_version", &[])
}

/// Query the relay parent offset from the runtime.
pub fn relay_parent_offset<Block: BlockT, HostFns: HostFunctions>(
    externalities: &TestExternalities<HashingFor<Block>>,
    executor: &WasmExecutor<HostFns>,
) -> u32 {
    dry_call::<u32, Block, HostFns>(
        externalities,
        executor,
        "RelayParentOffsetApi_relay_parent_offset",
        &[],
    )
    .unwrap_or(0)
}

/// Call `method` with `data` and return the result. `externalities` will not change.
fn dry_call<T: Decode, Block: BlockT, HostFns: HostFunctions>(
    externalities: &TestExternalities<HashingFor<Block>>,
    executor: &WasmExecutor<HostFns>,
    method: &'static str,
    data: &[u8],
) -> Result<T> {
    let (_, result) = state_machine_call::<Block, HostFns>(
        externalities,
        executor,
        method,
        data,
        full_extensions(executor.clone()),
    )?;

    Ok(<T>::decode(&mut &*result)?)
}
