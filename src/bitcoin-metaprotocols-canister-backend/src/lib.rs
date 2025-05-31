use candid::{candid_method, CandidType};
use ic_cdk::api::management_canister::http_request::{
    CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroAddressInscriptionsResponse {
    data: Vec<MaestroAddressInscription>,
    last_updated: LastUpdated,
    next_cursor: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroAddressInscription {
    inscription_id: String,
    satoshis: String,
    utxo_sat_offset: i64,
    utxo_txid: String,
    utxo_vout: i32,
    utxo_block_height: i64,
    utxo_confirmations: i64,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroInscriptionInfoResponse {
    data: MaestroInscriptionInfo,
    last_updated: LastUpdated,
    next_cursor: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroInscriptionInfo {
    collection_symbol: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct AddressInscription {
    inscription_id: String,
    satoshis: String,
    utxo_sat_offset: i64,
    utxo_txid: String,
    utxo_vout: i32,
    utxo_block_height: i64,
    utxo_confirmations: i64,
    collection_symbol: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct AddressInscriptions {
    data: Vec<AddressInscription>,
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
async fn get_address_inscriptions(
    api_key: String,
    address: String,
    count: String,
) -> Result<AddressInscriptions, String> {
    let address_inscriptions_maestro_url = format!(
        "https://xbt-mainnet.gomaestro-api.org/v0/addresses/{}/inscriptions?count={}",
        address, count
    );

    let address_inscriptions_maestro_request = CanisterHttpRequestArgument {
        url: address_inscriptions_maestro_url,
        method: HttpMethod::GET,
        headers: vec![HttpHeader {
            name: "api-key".to_string(),
            value: api_key.clone(),
        }],
        body: None,
        max_response_bytes: Some(5 * 1000), // 5000 KB
        transform: None,
    };

    let cycles = 1_000_000_000u128;

    match ic_cdk::api::management_canister::http_request::http_request(
        address_inscriptions_maestro_request,
        cycles,
    )
    .await
    {
        Ok((response,)) => {
            let raw_body = String::from_utf8_lossy(&response.body);
            ic_cdk::println!("HTTP response body: {}", raw_body);

            let address_inscriptions_maestro_response: MaestroAddressInscriptionsResponse =
                serde_json::from_slice(&response.body)
                    .map_err(|e| format!("Failed to parse: {} (body: {})", e, raw_body))?;

            let mut final_result: Vec<AddressInscription> = Vec::new();

            for inscription in address_inscriptions_maestro_response.data {
                let inscription_info_url = format!(
                    "https://xbt-mainnet.gomaestro-api.org/v0/assets/inscriptions/{}",
                    inscription.inscription_id
                );

                let inscription_info_request = CanisterHttpRequestArgument {
                    url: inscription_info_url,
                    method: HttpMethod::GET,
                    headers: vec![HttpHeader {
                        name: "api-key".to_string(),
                        value: api_key.clone(),
                    }],
                    body: None,
                    max_response_bytes: Some(5 * 1000),
                    transform: None,
                };

                let collection_symbol =
                    match ic_cdk::api::management_canister::http_request::http_request(
                        inscription_info_request,
                        cycles,
                    )
                    .await
                    {
                        Ok((inscription_info_response,)) => {
                            match serde_json::from_slice::<MaestroInscriptionInfoResponse>(
                                &inscription_info_response.body,
                            ) {
                                Ok(info_response) => info_response.data.collection_symbol,
                                Err(e) => {
                                    ic_cdk::println!(
                                        "Failed to parse MaestroInscriptionInfoResponse: {} (body: {})",
                                        e,
                                        String::from_utf8_lossy(&inscription_info_response.body)
                                    );
                                    None
                                }
                            }
                        }
                        Err((code, message)) => {
                            ic_cdk::println!(
                                "Failed to fetch inscription info for {}: {} - {}",
                                inscription.inscription_id,
                                code as u8,
                                message
                            );
                            None
                        }
                    };

                final_result.push(AddressInscription {
                    inscription_id: inscription.inscription_id,
                    satoshis: inscription.satoshis,
                    utxo_sat_offset: inscription.utxo_sat_offset,
                    utxo_txid: inscription.utxo_txid,
                    utxo_vout: inscription.utxo_vout,
                    utxo_block_height: inscription.utxo_block_height,
                    utxo_confirmations: inscription.utxo_confirmations,
                    collection_symbol,
                });
            }

            Ok(AddressInscriptions {
                data: final_result,
                last_updated: address_inscriptions_maestro_response.last_updated,
                next_cursor: address_inscriptions_maestro_response.next_cursor,
            })
        }
        Err((code, message)) => Err(format!("HTTP error {}: {}", code as u8, message)),
    }
}

ic_cdk::export_candid!();

fn main() {}
