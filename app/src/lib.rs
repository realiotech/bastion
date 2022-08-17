extern crate dotenv;
use actix_web::middleware::Logger;
use dotenv::dotenv;

use actix_web::dev::{HttpServiceFactory, Server};
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use bindings::erc20::{BalanceOfCall, ERC20};
use bindings::land_bank;
use bindings::land_nft::LandNFT;
use ethers::prelude::k256::sha2::digest::typenum::private::PrivateAnd;
use ethers::providers::test_provider::TestProvider;
use ethers::providers::{self, PendingTransaction};
use ethers::signers::LocalWallet;
use ethers::types::{H256, U256};
use ethers::utils::hex;
use ethers::utils::Anvil;
use ethers::{prelude::*, providers::Provider, types::Address};
use serde::{Deserialize, Serialize};
use std::net::TcpListener;
use std::ops::Add;
use std::sync::Arc;
use std::{thread, time};
pub struct Data {
    pub land_contract: Address,
    pub marketplace_contract: Address,
}

#[derive(Deserialize, Clone)]
pub struct Region {
    pub region: Vec<u128>,
    pub price: u128,
}

#[post("/mint")]
async fn mint(field: web::Json<Region>) -> impl Responder {
    let land_address = "0xA536F3E29ACaA2faaf8Ae0F2BAB1F002B7eBB887"
        .parse::<Address>()
        .unwrap();

    let token_address = "0x32E0b53B799cC14c455011fE3458306f89aee848"
        .parse::<Address>()
        .unwrap();

    let provider = Arc::new({
        // connect to the network
        let provider =
            Provider::try_from("https://rinkeby.infura.io/v3/0a7b42115f6a48c0b2aa5be4aacfd789")
                .unwrap();
        let chain_id = provider.get_chainid().await;

        // this wallet's private key
        let wallet = "<Key>"
            .parse::<LocalWallet>()
            .expect("Unable to derive wallet")
            .with_chain_id(chain_id.expect("msg").as_u64());

        SignerMiddleware::new(provider, wallet)
    });

    let token = ERC20::new(token_address, provider.clone());
    let land_contract = LandNFT::new(land_address, provider.clone());

    let rio_address = "0x32e0b53b799cc14c455011fe3458306f89aee848"
        .parse::<Address>()
        .unwrap();

    let rio = ERC20::new(rio_address, provider.clone());

    let wallet_address = "0x27a1876A09581E02E583E002E42EC1322abE9655"
        .parse::<Address>()
        .unwrap();
    let balance = rio.balance_of(wallet_address).call().await.unwrap();
    let price = land_contract.price().call().await.unwrap();
    let price_u256 = U256::from(field.price);
    let region_u256 = field.region.iter().map(|x| U256::from(*x)).collect();

    println!(
        "Wallet Balance is {:?} 
                \n Price is {:?} 
                \n Price of from json {:?}
                \n Region Vec<u128> {:?}
                \n Region Vec<U256> {:?}",
        balance, price, price_u256, field.region, region_u256
    );

    let approve = token.approve(land_contract.address(), price_u256);

    approve.send().await.unwrap();

    thread::sleep(time::Duration::from_secs(20));

    let buy_land = land_contract.mint(region_u256, price_u256);

    let tx = buy_land.send().await.unwrap();

    // let result = tx.unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(tx.to_string())
}

#[get("/total_supply")]
async fn get_total_supply() -> impl Responder {
    let provider = Arc::new({
        // connect to the network
        let provider =
            Provider::try_from("https://rinkeby.infura.io/v3/0a7b42115f6a48c0b2aa5be4aacfd789")
                .unwrap();
        let chain_id = provider.get_chainid().await;

        // this wallet's private key
        let wallet = "<Key>"
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(chain_id.expect("msg").as_u64());

        SignerMiddleware::new(provider, wallet)
    });
    let address = "0x7382507777ec4b2bc80Ea2b06F43f8A410fbbaa0"
        .parse::<Address>()
        .unwrap();
    let land_contract = LandNFT::new(address, provider.clone());

    // let rio_address= "0x32e0b53b799cc14c455011fe3458306f89aee848".parse::()<Address>().unwrap();

    // let balance = BalanceOfCall("0x27a1876A09581E02E583E002E42EC1322abE9655");

    let total_supply = land_contract.total_supply().call().await;

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(total_supply.unwrap().to_string())
}

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// pub fn run() -> Result<Server, std::io::Error> {
//     let server = HttpServer::new(|| {
//         App::new()
//             .service(get_total_supply)
//             .service(mint)
//             .service(health_check)
//     })
//     .bind("127.0.0.1:8000")?
//     .run();

//     Ok(server)
// }

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    dotenv().ok();
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(get_total_supply)
            .service(health_check)
            .service(mint)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
