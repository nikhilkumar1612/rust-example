use std::fmt::format;
use std::str::FromStr;
use std::{collections::HashMap, sync::Arc};
use abi::Function;
use url::form_urlencoded::parse;
use std::fs::{self};
use serde::Deserialize;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use ethers::{prelude::*, utils::parse_ether};
use ethers::signers::Wallet;
use ethers::abi::Abi;
use std::env;
use crate::apikeys::{ApiKey, ApiKeyWhitelist};
use crate::utils::db::connect_db;
use diesel::prelude::*;
use std::convert::TryFrom;


#[derive(Deserialize)]
struct Config {
    bundler: String,
    contract: String,
}

pub async fn deposit(req: HttpRequest) -> impl Responder {

    let query_string = req.query_string();
    let params: Vec<(String, String)> = parse(query_string.as_bytes())
        .into_owned()
        .collect();
    let mut query_params = std::collections::HashMap::new();
    for (key, value) in params {
        query_params.insert(key, value);
    }
    let chain_id = match query_params.get("chainId") {
        Some(chain_id) => chain_id,
        None => {
            return HttpResponse::BadRequest().json(json!({"error": "Missing chainId"}));
        }
    };

    let data = fs::read_to_string("config.json").expect("Error reading config.json");
    let config: HashMap<String, Config> = serde_json::from_str(&data).expect("Error parsing data into json");

    if !config.contains_key(chain_id) {
        return HttpResponse::BadRequest().json(json!({"error": "chain id not found"}));
    }

    let network = config.get(chain_id).unwrap();
    let private_key = env::var("PVT_KEY").expect("private key not set");
    let wallet: LocalWallet = Wallet::from_str(&private_key).unwrap();
    let provider = Provider::<Http>::try_from(&network.bundler).unwrap();
    let client = SignerMiddleware::new(provider, wallet);
    let client_arc = Arc::new(client);
    let contract_address = network.contract.parse::<Address>().unwrap();
    let abi_json = fs::read_to_string("paymaster.json").unwrap();
    let abi: Abi = serde_json::from_str(&abi_json).unwrap();
    let function = abi.function("depositFunds").unwrap();
    let calldata = function.encode_input(&vec![]).unwrap();
    // let contract = Contract::new(contract_address, abi, client_arc.clone());
    // let tx = contract.method::<_, U256>("depositFunds", ()).unwrap();

    let value = parse_ether("0.01").unwrap();
    let tx = TransactionRequest::new()
                                    .to(contract_address)
                                    .value(value)
                                    .data(calldata);
    println!("{:?}", tx);

    let result = client_arc.send_transaction(tx, None).await;
    println!("{:?}", result);
    HttpResponse::Ok().json(json!(
        {"success": true, "message": "Deposited Successfully"}
    ))
}

pub async fn whitelist(req: HttpRequest, body: web::Bytes) -> impl Responder{
    let json_body: ApiKeyWhitelist = serde_json::from_slice(&body).expect("invalid json body");

    
    let mut connection = connect_db().await;
    // use crate::schema::apikeys::dsl::*;
    let query = format!("SELECT * FROM apikeys WHERE api_key = '{}'", json_body.api_key);
    let api_key_records: Vec<ApiKey> = diesel::sql_query(query).load::<ApiKey>(&mut connection).unwrap();
    if api_key_records.is_empty() {
        return HttpResponse::BadRequest().json(json!(
            {"success": false, "message": "Invalid API key"}
        ));
    }

    let api_key_record = api_key_records.first().unwrap();

    println!("{:?}, {}", api_key_record.whitelisted_addresses, api_key_record.api_key);

    let first_vec: Vec<String> = match &api_key_record.whitelisted_addresses {
        Some(json_str) => serde_json::from_str(&json_str.to_string()).unwrap(),
        None => vec![],
    };

    let second_vec: Vec<String> = serde_json::from_str(&json_body.whitelist_addresses).unwrap();

    let mut concatenated_vec = first_vec;
    concatenated_vec.extend(second_vec);

    let concatenated_json = serde_json::to_string(&concatenated_vec).unwrap();

    let update_query = format!("UPDATE apikeys SET whitelisted_addresses = '{}' WHERE api_key = '{}'", concatenated_json, json_body.api_key);
    let _ = diesel::sql_query(update_query).execute(&mut connection);

    return HttpResponse::Ok().json(json!(
        {"success": true, "message": "Whitelisted Successfully"}
    ));
}

pub async fn save_key(req: HttpRequest, body: web::Bytes) -> impl Responder {
    let json_body: ApiKey = serde_json::from_slice(&body).expect("invalid JSON body");
    
    use crate::schema::apikeys::dsl::*;
    let mut connection = connect_db().await;

    let new_api_key = ApiKey {
        api_key: json_body.api_key,
        transaction_limit: json_body.transaction_limit,
        wallet_address: json_body.wallet_address,
        whitelisted_addresses: json_body.whitelisted_addresses,
    };
    let _ = diesel::insert_into(apikeys).values(new_api_key).execute(&mut connection);
    return HttpResponse::Ok().json(json!(
        {"success": true, "message": "Saved key successfully"}
    ));
}