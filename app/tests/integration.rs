//! tests/integration.rs

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
async fn mint_works() {
    let address = spawn_app().await;

    let client = Client::new();
    // curl -v POST "http://127.0.0.1:8000/mint" -H 'Accept: application/json' -H 'Content-Type: application/json' -d '{
        
    //     "region": [2],
    //     "price": 2000000000000000000
    //     }'
    let body = r# {
        
        //     "region": [2],
        //     "price": 2000000000000000000
        //     }'
        r#
    let response = client
        .get(&format!("{}/mint", &address))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body()
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
