mod anvil_pool;
use alloy::node_bindings::Anvil;

#[tokio::main]
async fn main() {
    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH.
      let anvil = Anvil::default().spawn();
      let x = anvil.addresses();
      for a in x {
          println!("{}", a.to_checksum(None))
      }
    println!("Anvil running at `{}`", anvil.endpoint());
}
