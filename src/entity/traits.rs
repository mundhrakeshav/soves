use alloy::{providers::RootProvider, transports::Transport};


trait Parser {
    fn name() -> &'static str;
    fn parse();
}



pub trait ClientProvider {
    type ClientType: Transport + Clone;

    /// Get a client by its chain ID
    fn get_client(&self, chain_id: u32) -> Option<&RootProvider<Self::ClientType>>;
}

