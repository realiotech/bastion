//! tests/integration.rs

use dotenv::dotenv;
use ethers::providers::Middleware;
use ethers::signers::{LocalWallet, Signer};
use ethers::{
    prelude::{k256::ecdsa::SigningKey, SignerMiddleware},
    providers::{Http, Provider},
    signers::Wallet,
};
use reqwest::Client;
use std::env;
use std::time::Duration;
use std::{net::TcpListener, sync::Arc};

async fn spawn_app() -> String {
    dotenv().ok();
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = app::run(listener, enable_provider().await).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

async fn enable_provider() -> Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    Arc::new({
        let provider = Provider::<Http>::try_from(env::var("TEST_RPC_URL").expect("error"))
            .expect("Unable to Create Provider")
            .interval(Duration::from_millis(10u64));
        let chain_id = provider.get_chainid().await;

        // this wallet's private key
        let wallet = env::var("PRIVATE_KEY")
            .expect("error")
            .parse::<LocalWallet>()
            .expect("Unable to derive wallet")
            .with_chain_id(chain_id.expect("msg").as_u64());

        SignerMiddleware::new(provider, wallet)
    })
}

#[tokio::test]

async fn health_check() {
    let address = spawn_app().await;

    let client = Client::new();
    let response = client
        .get(&format!("{}/health_check", &address))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
#[tokio::test]
async fn get_balance() {
    let address = spawn_app().await;

    let client = Client::new();

    let body = r#"{
        "address": "0x27a1876A09581E02E583E002E42EC1322abE9655"
    }"#;

    let response = client
        .get(&format!("{}/balance_of", &address))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(body)
        .send()
        .await
        .expect("failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(1), response.content_length());
    assert_eq!(200, response.status().as_u16());

    // we minted 4 nft from owner address
    let response_result = response.text().await.unwrap();
    assert!(response_result == "4");
}

#[tokio::test]

async fn mint_works() {
    let address = spawn_app().await;

    let client = Client::new();

    let address_user = r#"{
        "address": "0x27a1876A09581E02E583E002E42EC1322abE9655"
    }"#;

    // get total mint
    let balance_before = client
        .get(&format!("{}/balance_of", &address))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(address_user)
        .send()
        .await
        .expect("failed to execute request");

    let result_before = balance_before.text().await.unwrap();

    let body = r#"{
        "price": 20000000000000000000,
        "region": [14]
    }"#;
    // mint a new land
    let response = client
        .post(&format!("{}/mint", &address))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());

    // get new supply
    let new_supply = client
        .get(&format!("{}/balance_of", &address))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(address_user)
        .send()
        .await
        .expect("failed to execute request");

    let new_supply_result = new_supply.text().await.unwrap();

    assert!(new_supply_result.parse::<u32>().unwrap() == result_before.parse::<u32>().unwrap() + 1);
    // assert!(new_supply.text().await.unwrap()) == u32(total_result + 1));
}
