use crate::models::{
    Address, AddressInfo, EvaluateTxResult, Genesis, HTTPResponse, HttpErrorInner, ProtocolParams,
    TxSubmitResult, UTxO,
};
use async_trait::async_trait;
use reqwest::Response;
use serde::de::DeserializeOwned;
use std::{fs, path::Path};
use url::Url;

use crate::error::{Error, Result};

pub mod error;
pub mod models;
#[cfg(test)]
pub mod tests;

pub const MAINNET_URL: &str = "https://cardano-mainnet.blockfrost.io/api/v0";
pub const PREPROD_NETWORK_URL: &str = "https://cardano-preprod.blockfrost.io/api/v0/";

pub fn load_key_from_file(key_path: &str) -> Result<String> {
    let path = Path::new(key_path);
    let text = fs::read_to_string(&path).map_err(Error::FileRead)?;
    let config: toml::Value = toml::from_str(&text).map_err(Error::Toml)?;
    let field = "project_id";
    let project_id = config[field]
        .as_str()
        .ok_or_else(|| Error::Config(field.to_string()))?
        .to_string();
    Ok(project_id)
}

pub struct BlockFrostHttp {
    parent_url: String,
    api_key: String, // A.K.A. `project_id`
}

#[async_trait]
pub trait BlockFrostHttpTrait {
    async fn genesis(&self) -> Result<Genesis>;

    async fn protocol_params(&self, epoch: u32) -> Result<ProtocolParams>;

    async fn address_info(&self, address: &str) -> Result<AddressInfo>;

    async fn utxos(&self, address: &str, maybe_count: Option<usize>) -> Result<Vec<UTxO>>;

    async fn datum(&self, datum_hash: &str) -> Result<serde_json::Value>;

    async fn assoc_addresses(&self, stake_address: &str) -> Result<Vec<Address>>;

    async fn account_associated_addresses_total(&self, base_addr: &str) -> Result<Vec<Address>>;

    async fn execution_units(&self, bytes: &[u8]) -> Result<EvaluateTxResult>;

    async fn submit_tx(&self, bytes: &[u8]) -> Result<TxSubmitResult>;
}

#[async_trait]
impl BlockFrostHttpTrait for BlockFrostHttp {
    async fn genesis(&self) -> Result<Genesis> {
        let ext = "./genesis";
        self.get_endpoint(ext).await
    }

    async fn protocol_params(&self, epoch: u32) -> Result<ProtocolParams> {
        let ext = format!("./epochs/{}/parameters", epoch);
        self.get_endpoint(&ext).await
    }

    async fn address_info(&self, address: &str) -> Result<AddressInfo> {
        let ext = format!("./addresses/{}", address);
        self.get_endpoint(&ext).await
    }

    async fn utxos(&self, address: &str, maybe_count: Option<usize>) -> Result<Vec<UTxO>> {
        let ext = format!("./addresses/{}/utxos", address);

        let params = if let Some(count) = maybe_count {
            let count_str = count.to_string();
            vec![
                ("order".to_string(), "desc".to_string()),
                ("count".to_string(), count_str),
            ]
        } else {
            // TODO: Paginate response for more than 100
            vec![("order".to_string(), "desc".to_string())]
        };
        self.get_endpoint_with_params(&ext, &params).await
    }

    async fn datum(&self, datum_hash: &str) -> Result<serde_json::Value> {
        let ext = format!("./scripts/datum/{}", datum_hash);
        self.get_endpoint(&ext).await
    }

    async fn assoc_addresses(&self, stake_address: &str) -> Result<Vec<Address>> {
        let ext = format!("./accounts/{}/addresses", stake_address);
        self.get_endpoint(&ext).await
    }

    async fn account_associated_addresses_total(&self, base_addr: &str) -> Result<Vec<Address>> {
        let ext = format!("./accounts/{}/addresses/total", base_addr);
        self.get_endpoint(&ext).await
    }

    async fn execution_units(&self, bytes: &[u8]) -> Result<EvaluateTxResult> {
        let ext = "./utils/txs/evaluate".to_string();
        let url = Url::parse(&self.parent_url)?.join(&ext)?;
        let client = reqwest::Client::new();
        let project_id = &self.api_key;
        let encoded = hex::encode(bytes);
        let res = client
            .post(url)
            .header("Content-Type", "application/cbor")
            .header("project_id", project_id)
            .body(encoded)
            .send()
            .await
            .unwrap();
        try_deserializing(res).await
    }

    async fn submit_tx(&self, bytes: &[u8]) -> Result<TxSubmitResult> {
        let ext = "./tx/submit".to_string();
        let url = Url::parse(&self.parent_url)?.join(&ext)?;
        let client = reqwest::Client::new();
        let project_id = &self.api_key;
        let res = client
            .post(url)
            .header("Content-Type", "application/cbor")
            .header("project_id", project_id)
            .body(bytes.to_owned()) // For some dumb-ass reason this is binary
            .send()
            .await
            .unwrap();
        try_deserializing(res).await
    }
}

impl BlockFrostHttp {
    pub fn new(url: &str, key: &str) -> Self {
        let parent_url = url.to_string();
        let api_key = key.to_string();
        BlockFrostHttp {
            parent_url,
            api_key,
        }
    }

    async fn get_endpoint<T: DeserializeOwned>(&self, ext: &str) -> Result<T> {
        self.get_endpoint_with_params(ext, &[]).await
    }

    async fn get_endpoint_with_params<T: DeserializeOwned>(
        &self,
        ext: &str,
        params: &[(String, String)],
    ) -> Result<T> {
        let mut url = Url::parse(&self.parent_url)?.join(ext)?;
        url.query_pairs_mut().extend_pairs(params);
        let client = reqwest::Client::new();
        let project_id = &self.api_key;
        let res = client
            .get(url)
            .header("project_id", project_id)
            .send()
            .await?;

        try_deserializing(res).await
    }
}

async fn try_deserializing<T: DeserializeOwned>(res: Response) -> Result<T> {
    let full = res.bytes().await.map_err(|e| Error::Reqwest(e))?;
    // let json: serde_json::Value = serde_json::from_slice(&full).unwrap();
    // println!("json: {:?}", json);
    let response = if let Ok(inner) = serde_json::from_slice(&full) {
        HTTPResponse::HttpOk(inner)
    } else {
        let err: HttpErrorInner = serde_json::from_slice(&full).map_err(|e| Error::SerdeJson(e))?;
        HTTPResponse::HttpError(err)
    };
    match response {
        HTTPResponse::HttpOk(inner) => Ok(inner),
        HTTPResponse::HttpError(HttpErrorInner {
            status_code,
            error,
            message,
        }) => Err(Error::HttpError {
            status_code,
            error,
            message,
        }),
    }
}
