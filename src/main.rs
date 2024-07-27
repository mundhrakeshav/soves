mod anvil_pool;
use std::{str::FromStr, time};

use alloy::{
    network::TransactionBuilder,
    node_bindings::Anvil,
    primitives::{Address, Bytes, U256},
    providers::{ext::AnvilApi, Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
};
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    let mut v: Vec<JoinHandle<()>> = Vec::new();

    for x in 1..=7 {
        println!("Spawning worker {}", x.clone() as u32);
        let y = tokio::spawn(async move {
            boot(x as u32).await;
        });
        v.push(y);
    }

    for handle in v {
        handle.await.unwrap();
    }
}

async fn boot(x: u32) {
    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH.
    //
    // print start time

    let anvil = Anvil::new().block_time(1);
    let start = time::Instant::now();

    let anvil = anvil.fork("192.168.29.83:8545").try_spawn().unwrap();

    let duration = start.elapsed();
    println!("Time elapsed 1 : {} {:?}", x, duration);

    let provider = ProviderBuilder::new().on_http(anvil.endpoint_url());
    let from_address = Address::from_str("0xADCf0A13Aeb1400DD7f5A340dD51c62754e605b9").unwrap();
    let to_address = Address::from_str("0xecD4bd3121F9FD604ffaC631bF6d41ec12f1fafb").unwrap();
    let input_data = Bytes::from_str("0x80500d20000000000000000000000000794a61358d6845594f94dc1db02a252b5b4814ad00000000000000000000000000000000000000000000000001871e0058b96720000000000000000000000000adcf0a13aeb1400dd7f5a340dd51c62754e605b9").unwrap();
    let value: U256 = U256::from_str("0").unwrap();

    let duration = start.elapsed();
    println!("Time elapsed 2 : {} {:?}", x, duration);
    let txn_req = TransactionRequest::default()
        .with_to(to_address)
        .with_from(from_address)
        .with_input(input_data)
        .with_value(value)
        .with_gas_limit(30000000);
    let duration = start.elapsed();
    println!("Time elapsed 3 : {} {:?}", x, duration);
    let balance = U256::from(20).pow(U256::from(18));
    // Perform account setup operations concurrently
    let (balance_result, impersonate_result) = tokio::join!(
        provider.anvil_set_balance(from_address, balance),
        provider.anvil_impersonate_account(from_address)
    );

    // Check for errors in concurrent operations
    balance_result.unwrap();
    impersonate_result.unwrap();

    let duration = start.elapsed();
    println!("Time elapsed 4 : {} {:?}", x, duration);
    let reciept = provider.send_transaction(txn_req).await.unwrap();
    let duration = start.elapsed();
    println!("Time elapsed 5 : {} {:?}", x, duration);

    let reciept = reciept.get_receipt().await.unwrap();
    println!("Transaction Hash: {:?}", reciept.transaction_hash);
    // let logs: &Vec<Log> = reciept.inner.as_receipt().unwrap().logs.as_ref();

    // calcualte time since start
    let duration = start.elapsed();
    println!("Time elapsed 6 : {} {:?}", x, duration);
}
