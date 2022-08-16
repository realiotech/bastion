use actix_web::dev::{HttpServiceFactory, Server};
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use bindings::erc20::ERC20;
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
use std::ops::Add;
use std::sync::Arc;
pub struct Data {
    pub land_contract: Address,
    pub marketplace_contract: Address,
}

#[derive(Deserialize, Clone)]
pub struct Region {
    pub region: Vec<U256>,
    pub price: U256,
}

#[post("/mint")]
async fn mint(field: web::Json<Region>) -> impl Responder {
    let wallet_address = "0x27a1876A09581E02E583E002E42EC1322abE9655".parse::<Address>();

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
        let wallet = "<key>"
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(chain_id.expect("msg").as_u64());

        SignerMiddleware::new(provider, wallet)
    });

    let token = ERC20::new(token_address, provider.clone());
    let land_contract = LandNFT::new(land_address, provider.clone());

    let approve = token.approve(land_contract.address(), field.price);

    approve.send().await;

    let buy_land = land_contract.mint(field.region.clone(), field.price);

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
    let land_contract = LandNFT::new(address, provider);

    let total_supply = land_contract.total_supply().call().await;

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(total_supply.unwrap().to_string())
}

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(get_total_supply)
            .service(mint)
            .service(health_check)
    })
    .bind("127.0.0.1:8000")?
    .run();

    Ok(server)
}
