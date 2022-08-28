//! tests/integration.rs

use dotenv::dotenv;
use ethers::providers::Middleware;
use ethers::signers::{LocalWallet, Signer};
use ethers::{
    prelude::{k256::ecdsa::SigningKey, SignerMiddleware},
    providers::{Http, Provider},
    signers::Wallet,
    utils::Anvil,
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
        let provider = Provider::<Http>::try_from(env::var("RPC_URL").expect("error"))
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
    // curl -v POST "http://127.0.0.1:8000/mint" -H 'Accept: application/json' -H 'Content-Type: application/json' -d '{

    //     "region": [2],
    //     "price": 2000000000000000000
    //     }'
    // let body = r# {

    //         // "region": [2],
    //         // "price": 2000000000000000000
    //         // }'
    //     r#

    let response = client
        .get(&format!("{}/health_check", &address))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        // .body()
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// curl -v POST "http://127.0.0.1:8000/mint" -H 'Accept: application/json' -H 'Content-Type: application/json' -d '{

//     "region": ["0xae0b5f5221983e3e82f0eb91f7345e7848dec067d9dc6a0d2eed5a41245d97f3"],
//     "price": "0xae0b5f5221983e3e82f0eb91f7345e7848dec067d9dc6a0d2eed5a41245d97f3"
//     }'

//     curl -v POST "http://127.0.0.1:8000/mint" -H 'Accept: application/json' -H 'Content-Type: application/json' -d '{

//         "region": ["0xae0b5f5221983e3e82f0eb91f7345e7848dec067d9dc6a0d2eed5a41245d97f3"],
//         "price": "0xae0b5f5221983e3e82f0eb91f7345e7848dec067d9dc6a0d2eed5a41245d97f3"
//         }'

//     curl -v POST "http://127.0.0.1:8000/mint" -H 'Accept: application/json' -H 'Content-Type: application/json' -d '{

//         "region": ["0"],
//         "price": "200"
//         }'

//     curl -v POST "http://127.0.0.1:8000/mint" -H 'Accept: application/json' -H 'Content-Type: application/json' -d '{

//         "region": [2],
//         "price": 2000000000000000000
//         }'

//     curl -v POST "http://127.0.0.1:8000/balance_of" -H 'Accept: application/json' -H 'Content-Type: application/json' -d '{

//         "address": "0x27a1876A09581E02E583E002E42EC1322abE9655"
//         }'

// 0x27a1876A09581E02E583E002E42EC1322abE9655

//          curl -v POST "http://127.0.0.1:8000/total_supply"

//          91000000000000000000000
//          2000000000000000000

//          curl -v POST "http://127.0.0.1:8000/total_supply"
