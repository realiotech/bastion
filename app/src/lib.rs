use actix_web::dev::{HttpServiceFactory, Server};
use actix_web::http::header::ContentType;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
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

async fn mint() -> impl Responder {
    let anvil = Anvil::new()
        .fork("https://mainnet.infura.io/v3/0a7b42115f6a48c0b2aa5be4aacfd789")
        .spawn();

    let wallet_address = "0x27a1876A09581E02E583E002E42EC1322abE9655".parse::<Address>();

    let provider = Arc::new({
        // connect to the network
        let provider = Provider::try_from(anvil.endpoint()).unwrap();
        let chain_id = provider.get_chainid().await;

        // this wallet's private key
        let wallet = "<Key>"
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(chain_id.expect("msg").as_u64());

        SignerMiddleware::new(provider, wallet)
    });

    // deploy new landbank
    let land_contract = LandNFT::deploy(
        provider,
        (
            wallet_address.unwrap().to_string(),
            wallet_address.unwrap().to_string(),
            20000,
        ),
    );

    let contract = land_contract.expect("failed").send().await;

    let result = contract.expect("errir").address();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(result.to_string())
}

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

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/total_supply", web::get().to(get_total_supply))
            .route("mint", web::get().to(mint))
    })
    .bind("127.0.0.1:8000")?
    .run();

    Ok(server)
}
