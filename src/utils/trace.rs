use crate::entity::errors::SovesError;
use alloy::providers::{Provider, RootProvider};
use alloy::transports::Transport;
use alloy::{
    primitives::TxHash,
    providers::ext::DebugApi,
    rpc::types::trace::geth::{
        CallConfig, CallFrame, GethDebugBuiltInTracerType, GethDebugTracerType,
        GethDebugTracingOptions, GethDefaultTracingOptions,
    },
};

pub async fn get_trace_for_hash<T>(
    provider: &RootProvider<T>,
    tx_hash: TxHash,
) -> Result<CallFrame, SovesError>
where
    T: Transport + Clone,
{
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
                "failed to get trace for txn hash: {}, err: {}",
                tx_hash,
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
