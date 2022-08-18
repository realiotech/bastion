extern crate dotenv;
use actix_web::middleware::Logger;
// use dotenv::dotenv;

use actix_web::dev::{HttpServiceFactory, Server};
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use bindings::erc20::{BalanceOfCall, ERC20};
use bindings::land_nft::LandNFT;
use dotenv::dotenv;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::k256::sha2::digest::typenum::private::PrivateAnd;
use ethers::providers::test_provider::TestProvider;
use ethers::providers::{self, PendingTransaction};
use ethers::signers::LocalWallet;
use ethers::types::{H256, U256};
use ethers::utils::hex;
use ethers::utils::Anvil;
use ethers::{prelude::*, providers::Provider, types::Address};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::TcpListener;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::{thread, time};

#[derive(Deserialize, Clone)]
pub struct Region {
    pub region: Vec<u128>,
    pub price: u128,
}
#[derive(Deserialize, Clone)]
pub struct User {
    pub address: Address,
}

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub token: String,
    pub land_contract: String,
    pub rpc_url: String,
    pub key: String,
}
#[derive(Deserialize, Serialize)]
struct AppState {
    datas: Mutex<Data>,
}

#[derive(Deserialize, Clone)]
pub struct LandBank {
    pub bank: Address,
}

#[derive(Deserialize, Clone)]
pub struct DevFund {
    pub dev: Address,
}

async fn enable_provider() -> Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    dotenv().ok();
    let provider = Arc::new({
        // connect to the network
        let provider =
            Provider::try_from("https://rinkeby.infura.io/v3/0a7b42115f6a48c0b2aa5be4aacfd789")
                .unwrap();
        let chain_id = provider.get_chainid().await;

        // this wallet's private key
        let wallet = env::var("PRIVATE_KEY")
            .expect("error")
            .parse::<LocalWallet>()
            .expect("Unable to derive wallet")
            .with_chain_id(chain_id.expect("msg").as_u64());

        SignerMiddleware::new(provider, wallet)
    });
    provider
}

async fn land_contract() -> LandNFT<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    let land_address = "0xA536F3E29ACaA2faaf8Ae0F2BAB1F002B7eBB887"
        .parse::<Address>()
        .unwrap();
    let land_contract = LandNFT::new(land_address, enable_provider().await);

    land_contract
}

async fn token() -> ERC20<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    let token_address = "0x32E0b53B799cC14c455011fE3458306f89aee848"
        .parse::<Address>()
        .unwrap();

    let token = ERC20::new(token_address, enable_provider().await);

    token
}

#[get("/balance_of")]
async fn get_user_balance(user: web::Json<User>) -> impl Responder {
    let address = user.address;

    let balanceOf = land_contract().await.balance_of(address);

    let result = balanceOf.call().await;

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(result.unwrap().to_string())
}

#[post("/set_bank")]
async fn set_land_bank(data: web::Json<DevFund>) -> impl Responder {
    let new_landBank = data.dev;
    let setter = land_contract().await.set_dev_fund(new_landBank);

    let tx = setter.send().await.unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(tx.to_string())
}

#[post("/set_dev")]
async fn set_dev_fund(bank: web::Json<LandBank>) -> impl Responder {
    let new_landBank = bank.bank;
    let setter = land_contract().await.set_land_bank(new_landBank);

    let tx = setter.send().await.unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(tx.to_string())
}

#[post("/mint")]
async fn mint(field: web::Json<Region>) -> impl Responder {
    let rio_address = "0x32e0b53b799cc14c455011fe3458306f89aee848"
        .parse::<Address>()
        .unwrap();

    let rio = ERC20::new(rio_address, enable_provider().await);

    let wallet_address = "0x27a1876A09581E02E583E002E42EC1322abE9655"
        .parse::<Address>()
        .unwrap();
    let balance = rio.balance_of(wallet_address).call().await.unwrap();
    let price = land_contract().await.price().call().await.unwrap();
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

    let approve = token()
        .await
        .approve(land_contract().await.address(), price_u256);

    approve.send().await.unwrap();

    thread::sleep(time::Duration::from_secs(20));

    let buy_land = land_contract().await.mint(region_u256, price_u256);

    let tx = buy_land.send().await.unwrap();

    // let result = tx.unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(tx.to_string())
}

#[get("/total_supply")]
async fn get_total_supply() -> impl Responder {
    let total_supply = land_contract().await.total_supply().call().await;

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(total_supply.unwrap().to_string())
}

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            // .app_data(app_state.clone())
            .service(get_total_supply)
            .service(health_check)
            .service(get_user_balance)
            .service(set_land_bank)
            .service(set_dev_fund)
            .service(mint)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
