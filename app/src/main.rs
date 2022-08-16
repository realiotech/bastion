// use actix_web::body::BoxBody;
// use actix_web::http::header::ContentType;
// use actix_web::http::StatusCode;
// use actix_web::{
//     get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError,
// };
// use bindings::erc20::ERC20;
// use bindings::land_nft::LandNFT;
// use bindings::marketplace::Marketplace;
// use ethers::abi::Uint;
// use ethers::prelude::k256::ecdsa::SigningKey;
// use ethers::prelude::k256::SecretKey;
// use ethers::prelude::{ContractError, Http};
// use ethers::providers::test_provider::TestProvider;
// use ethers::providers::{self, PendingTransaction};
// use ethers::signers::LocalWallet;
// use ethers::types::{H256, U256};
// use ethers::utils::hex;
// use ethers::utils::Anvil;
// use ethers::{prelude::*, providers::Provider, types::Address};
// use eyre::Result;
// use serde::{Deserialize, Serialize};
// use std::convert::TryFrom;
// use std::fmt::Display;
// use std::ops::Add;
// use std::str::FromStr;
// use std::sync::{Arc, Mutex};
// #[warn(unused_must_use)]
// #[derive(Serialize, Deserialize)]
// pub struct Contract {
//     pub address: Address,
//     pub token: Address,
//     pub provider: String,
// }

// #[derive(Deserialize, Clone)]
// pub struct Region {
//     pub region: Vec<U256>,
//     pub price: U256,
// }

// #[derive(Deserialize, Clone)]
// pub struct WalletAccount {
//     pub key: String,
// }

// impl Responder for Contract {
//     type Body = BoxBody;

//     fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
//         let res_body = serde_json::to_string(&self).unwrap();

//         // create HttpResponse and set content type
//         HttpResponse::Ok()
//             .content_type(ContentType::json())
//             .body(res_body)
//     }
// }

// #[get("/total_supply")]
// async fn total_supply(data: web::Data<Contract>) -> impl Responder {
//     let address = data.address;
//     let provider = Provider::try_from(&data.provider).unwrap();
//     let provider = Arc::new(provider);
//     let land_contract = LandNFT::new(address, provider);

//     let total_supply = land_contract.total_supply().call().await;

//     HttpResponse::Ok()
//         .content_type(ContentType::json())
//         .body(total_supply.unwrap().to_string())
// }

// #[get("/balanceOf/{user}")]
// async fn get_balance_of(user: web::Path<String>, data: web::Data<Contract>) -> impl Responder {
//     let address = String::from(&*user);

//     println!("address {}", address);

//     let address = address.parse::<Address>();

//     let contract = data.address;
//     let provider = Provider::try_from(data.provider.clone()).unwrap();
//     let provider = Arc::new(provider);
//     let land_contract = LandNFT::new(contract, provider);

//     let balance_of = land_contract.balance_of(address.unwrap()).call().await;
//     HttpResponse::Ok()
//         .content_type(ContentType::json())
//         .body(balance_of.unwrap().to_string())
// }

// #[get("/ownerOf/{token_id}")]
// async fn owner_of(token_id: web::Path<u32>, data: web::Data<Contract>) -> impl Responder {
//     let token_id = *token_id;
//     let contract = data.address;
//     let provider = Provider::try_from(data.provider.clone()).unwrap();
//     let provider = Arc::new(provider);

//     let land_contract = LandNFT::new(contract, provider);

//     let owner_of = land_contract
//         .method::<_, Address>("ownerOf", token_id)
//         .unwrap()
//         .call()
//         .await;

//     HttpResponse::Ok()
//         .content_type(ContentType::json())
//         .body(owner_of.unwrap().to_string())
// }

// #[get("/data")]
// async fn get_data(data: web::Data<Contract>) -> impl Responder {
//     let contract = data.address;
//     let provider = Provider::try_from(data.provider.clone()).unwrap();
//     let provider = Arc::new(provider);
//     let land_contract = LandNFT::new(contract, provider);

//     let response = land_contract.owner().call().await;

//     let result = serde_json::to_string(&response.unwrap()).unwrap();
//     HttpResponse::Ok()
//         .content_type(ContentType::json())
//         .body(result)
// }

// #[post("/mint")]
// async fn mint(field: web::Json<Region>, data: web::Data<Contract>) -> impl Responder {
//     let address = data.address;
//     let token = data.token;
//     let provider = Arc::new({
//         // connect to the network
//         let provider = Provider::try_from(&data.provider).unwrap();
//         let chain_id = provider.get_chainid().await;

//         // this wallet's private key
//         let wallet = "<Key>"
//             .parse::<LocalWallet>()
//             .unwrap()
//             .with_chain_id(chain_id.expect("msg").as_u64());

//         SignerMiddleware::new(provider, wallet)
//     });

//     let token = ERC20::new(token, provider.clone());
//     let land_contract = LandNFT::new(address, provider.clone());
//     let token_method = token.approve(land_contract.address(), field.price);

//     let tx = token_method.send().await;
//     let method = land_contract.mint(field.region.clone(), field.price);

//     let send_method = method.send().await;

//     let response = HttpResponse::Created()
//         .content_type(ContentType::json())
//         .body(tx.unwrap().to_string());
//     response
// }
#![crate_name = "app"]

use app::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // let app_state = web::Data::new(Contract {
    //     address: "0x7382507777ec4b2bc80Ea2b06F43f8A410fbbaa0"
    //         .parse::<Address>()
    //         .unwrap(),
    //     provider: String::from("https://rinkeby.infura.io/v3/0a7b42115f6a48c0b2aa5be4aacfd789"),
    //     token: "0x32E0b53B799cC14c455011fE3458306f89aee848"
    //         .parse::<Address>()
    //         .unwrap(),
    // });

    // HttpServer::new(move || {
    //     App::new()
    //         .app_data(app_state.clone())
    //         .service(get_data)
    //         .service(get_balance_of)
    //         .service(owner_of)
    //         .service(total_supply)
    //         .service(mint)
    // })
    // .bind(("127.0.0.1", 8000))?
    // .run()
    // .await
    run()?.await
}
