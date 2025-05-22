use candid::{candid_method, CandidType};
use ic_cdk::api::management_canister::http_request::{
    CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct Inscription {
    inscription_id: String,
    satoshis: String,
    utxo_sat_offset: i64,
    utxo_txid: String,
    utxo_vout: i32,
    utxo_block_height: i64,
    utxo_confirmations: i64,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroResponse {
    data: Vec<Inscription>,
    last_updated: LastUpdated,
    next_cursor: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct LastUpdated {
    block_hash: String,
    block_height: i64,
}

#[update]
#[candid_method(update)]
async fn get_inscriptions(
    api_key: String,
    address: String,
    count: String,
) -> Result<MaestroResponse, String> {
    let url = format!(
        "https://xbt-mainnet.gomaestro-api.org/v0/addresses/{}/inscriptions?count={}",
        address, count
    );

    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        headers: vec![HttpHeader {
            name: "api-key".to_string(),
            value: api_key,
        }],
        body: None,
        max_response_bytes: Some(5 * 1000), // 5000 KB
        transform: None,
    };

    let cycles = 1_000_000_000u128;

    match ic_cdk::api::management_canister::http_request::http_request(request, cycles).await {
        Ok((response,)) => {
            let raw_body = String::from_utf8_lossy(&response.body);
            ic_cdk::println!("HTTP response body: {}", raw_body);

            let json_result: Result<MaestroResponse, _> = serde_json::from_slice(&response.body);
            json_result.map_err(|e| format!("Failed to parse: {} (body: {})", e, raw_body))
        }
        Err((code, message)) => Err(format!("HTTP error {}: {}", code as u8, message)),
    }
}

ic_cdk::export_candid!();

fn main() {}
