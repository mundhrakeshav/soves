use alloy::{
    primitives::TxHash,
    providers::ext::DebugApi,
    rpc::types::trace::geth::{
        CallConfig, CallFrame, GethDebugBuiltInTracerType, GethDebugTracerType,
        GethDebugTracingOptions, GethDefaultTracingOptions,
    },
};

use crate::entity::{errors::SovesError, traits::ProviderFactory};

pub async fn get_trace_for_hash<T: ProviderFactory>(
    rpc_factory: T,
    tx_hash: TxHash,
    chain_id: u32,
) -> Result<CallFrame, SovesError> {
    let client_opt = rpc_factory.client(chain_id);

    let provider = match client_opt {
        None => return Err(SovesError::ClientNotFound(chain_id)),
        Some(client) => client,
    };

    // Trace with built-in call tracer.
    let trace_options = GethDebugTracingOptions {
        config: GethDefaultTracingOptions {
            ..Default::default()
        },
        tracer: Some(GethDebugTracerType::BuiltInTracer(
            GethDebugBuiltInTracerType::CallTracer,
        )),
        tracer_config: serde_json::to_value(CallConfig {
            only_top_call: Some(false),
            with_log: Some(true),
        })
        .unwrap()
        .into(),
        ..Default::default()
    };

    let trace = provider
        .debug_trace_transaction(tx_hash, trace_options)
        .await
        .map_err(|e| {
            SovesError::ProviderError(format!(
                "failed to get trace for txn hash: {}, chain: {}, err: {}",
                tx_hash,
                chain_id,
                e.to_string()
            ))
        })?
        .try_into_call_frame()
        .map_err(|e| {
            SovesError::ProviderError(format!(
                "failed to parse trace into callframe, err: {}",
                e.to_string()
            ))
        })?;

    Ok(trace)
}
