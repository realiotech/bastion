extern crate dotenv;

use actix_web::dev::{HttpServiceFactory, Server};
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use bindings::erc20::{BalanceOfCall, ERC20};
use bindings::land_nft::LandNFT;
use dotenv::dotenv;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::signers::LocalWallet;
use ethers::types::{H256, U256};
use ethers::utils::hex;
use ethers::utils::Anvil;
use ethers::{prelude::*, providers::Provider, types::Address};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::TcpListener;
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

// #[derive(Deserialize, Serialize)]
pub struct DataState {
    pub token:  ERC20<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub land_contract:  LandNFT<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    // pub rpc_url: String,
    // pub key: String,
    pub provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}



#[derive(Deserialize, Clone)]
pub struct LandBank {
    pub bank: Address,
}

#[derive(Deserialize, Clone)]
pub struct DevFund {
    pub dev: Address,
}

// async fn enable_provider() -> Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
//     let provider = Arc::new({
//         // connect to the network
//         let provider =
//             Provider::try_from(String::from(env::var("RPC_URL").expect("error"))).unwrap();
//         let chain_id = provider.get_chainid().await;

//         // this wallet's private key
//         let wallet = String::from(env::var("PRIVATE_KEY").expect("error"))
//             .parse::<LocalWallet>()
//             .expect("Unable to derive wallet")
//             .with_chain_id(chain_id.expect("msg").as_u64());

//         SignerMiddleware::new(provider, wallet)
//     });
//     provider
// }

// async fn land_contract() -> LandNFT<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
//     let land_address = String::from(env::var("LAND_CONTRACT").expect("error"))
//         .parse::<Address>()
//         .unwrap();
//     // let land_address =
//     let land_contract = LandNFT::new(land_address, enable_provider().await);

//     land_contract
// }

// async fn token() -> ERC20<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
//     let token_address = String::from(env::var("TOKEN_ADDRESS").expect("error"))
//         .parse::<Address>()
//         .unwrap();

//     let token = ERC20::new(token_address, enable_provider().await);

//     token
// }

// async fn generate_contracts() -> 
// let land_address = String::from(env::var("LAND_CONTRACT").expect("error"))
// .parse::<Address>()
// .unwrap();
// let land_contract = LandNFT::new(land_address, provider);
// let token_address = String::from(env::var("TOKEN_ADDRESS").expect("error"))
// .parse::<Address>()
// .unwrap();
// let token = ERC20::new(token_address, provider);

#[get("/balance_of")]
async fn get_user_balance(user: web::Json<User>, data: web::Data<DataState>) -> impl Responder {
    let address = user.address;

    let balance_of = data.land_contract.balance_of(address);

    let result = balance_of.call().await;

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(result.unwrap().to_string())
}

#[post("/set_bank")]
async fn set_land_bank(land_bank: web::Json<LandBank>, data: web::Data<DataState>) -> impl Responder {
    let new_landBank = land_bank.bank;
    let setter = data.land_contract.set_dev_fund(new_landBank);

    let tx = setter.send().await.unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(tx.to_string())
}

#[post("/set_dev")]
async fn set_dev_fund(bank: web::Json<LandBank>, data: web::Data<DataState>) -> impl Responder {
    let new_landBank = bank.bank;
    let setter = data.land_contract.set_land_bank(new_landBank);

    let tx = setter.send().await.unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(tx.to_string())
}

#[post("/mint")]
async fn mint(field: web::Json<Region>, data: web::Data<DataState>) -> impl Responder
{
    let price_u256 = U256::from(field.price);
    let region_u256 = field.region.iter().map(|x| U256::from(*x)).collect();

    let approve = data.token
        .approve(data.land_contract.address(), price_u256);

    approve.send().await.unwrap();

    thread::sleep(time::Duration::from_secs(20));

    let buy_land = data.land_contract.mint(region_u256, price_u256);

    let tx = buy_land.send().await.unwrap();

    // let result = tx.unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(tx.to_string())
}

#[get("/total_supply")]
async fn get_total_supply(data: web::Data<DataState>) -> impl Responder {
    let total_supply = data.land_contract.total_supply().call().await;

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(total_supply.unwrap().to_string())
}

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(
    listener: TcpListener,
    provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
) -> Result<Server, std::io::Error> {
    dotenv().ok();
    let land_address = String::from(env::var("LAND_CONTRACT").expect("error"))
        .parse::<Address>()
        .unwrap();
    let land_contract = LandNFT::new(land_address, provider.clone());
    let token_address = String::from(env::var("TOKEN_ADDRESS").expect("error"))
        .parse::<Address>()
        .unwrap();
    let token = ERC20::new(token_address, provider.clone());
    let app_state = web::Data::new(DataState {
        token: token,
        land_contract: land_contract,
        provider: provider.clone(),
    });
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
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
