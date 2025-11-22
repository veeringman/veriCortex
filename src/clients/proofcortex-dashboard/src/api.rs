use serde::{Deserialize, Serialize};
use reqwest::Client;
use anyhow::Result;


const BASE: &str = "http://localhost:8080"; // change to deployed endpoint


#[derive(Deserialize, Debug, Clone)]
pub struct ProofEvent { pub proofId: String, pub modelId: String, pub submitter: String, pub valid: bool }


pub async fn fetch_recent_events() -> Result<Vec<ProofEvent>> {
let client = Client::new();
let url = format!("{}/proofs/events/recent", BASE);
let resp = client.get(&url).send().await?.json::<Vec<ProofEvent>>().await?;
Ok(resp)
}


#[derive(Deserialize, Debug, Clone)]
pub struct ModelInfo { pub model_id: String, pub name: String, pub owner: String, pub description: String, pub trust: Option<f64> }


pub async fn fetch_models() -> Result<Vec<ModelInfo>> {
let client = Client::new();
let url = format!("{}/models", BASE);
let resp = client.get(&url).send().await?.json::<Vec<ModelInfo>>().await?;
Ok(resp)
}
