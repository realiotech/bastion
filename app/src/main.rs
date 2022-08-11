use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{
    get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError,
};
use bindings::land_nft::LandNFT;
use bindings::marketplace::Marketplace;
use ethers::abi::Uint;
use ethers::types::U256;
use ethers::{prelude::Middleware, providers::Provider, types::Address};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::Display;
use std::ops::Add;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize)]
pub struct Contract {
    pub address: Address,
}

impl Responder for Contract {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();

        // create HttpResponse and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(res_body)
    }
}

#[get("/balanceOf")]
async fn get_balance_of(user: web::Path<String>, data: web::Data<Contract>) -> impl Responder {
    let address = String::from(&*user);

    let address = address.parse::<Address>();

    let contract = data.address;
    let provider =
        Provider::try_from("https://eth-rinkeby.alchemyapi.io/v2/Lc7oIGYeL_QvInzI0Wiu_pOZZDEKBrdf")
            .unwrap();
    let provider = Arc::new(provider);
    let land_contract = LandNFT::new(contract, provider);

    let balance_of = land_contract
        .method::<_, U256>("balanceOf", address.unwrap())
        .unwrap()
        .call()
        .await;

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(balance_of.unwrap().to_string())
}

#[get("/data")]
async fn get_data(data: web::Data<Contract>) -> impl Responder {
    let contract = data.address;
    let provider =
        Provider::try_from("https://eth-rinkeby.alchemyapi.io/v2/Lc7oIGYeL_QvInzI0Wiu_pOZZDEKBrdf")
            .unwrap();
    let provider = Arc::new(provider);
    let land_contract = LandNFT::new(contract, provider);

    let response = land_contract
        .method::<_, Address>("owner", ())
        .unwrap()
        .call()
        .await;

    let result = serde_json::to_string(&response.unwrap()).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(result)
}
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(Contract {
        address: "0x7382507777ec4b2bc80Ea2b06F43f8A410fbbaa0"
            .parse::<Address>()
            .unwrap(),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(get_data)
            .service(get_balance_of)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
