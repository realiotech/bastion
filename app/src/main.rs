use app::run;
use dotenv::dotenv;
use env_logger::Env;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::{prelude::*, providers::Provider, types::Address};
use std::env;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let listener = TcpListener::bind("0.0.0.0:8000").expect("failed to bind port");
    run(listener, enable_provider().await)?.await
}

async fn enable_provider() -> Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    dotenv().ok();
    let provider = Arc::new({
        // connect to the network
        let provider = Provider::try_from(String::from(env::var("RPC_URL").unwrap())).unwrap();
        let chain_id = provider.get_chainid().await;

        // this wallet's private key
        let wallet = String::from(env::var("PRIVATE_KEY").expect("error"))
            .parse::<LocalWallet>()
            .expect("Unable to derive wallet")
            .with_chain_id(chain_id.expect("msg").as_u64());

        SignerMiddleware::new(provider, wallet)
    });
    provider
}
