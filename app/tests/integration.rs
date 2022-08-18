//! tests/integration.rs

use app::run;
use reqwest::Client;
use std::net::TcpListener;

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = app::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
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
