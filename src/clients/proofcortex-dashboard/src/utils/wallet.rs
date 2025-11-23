// src/utils/wallet.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::{Promise, JsString};
use anyhow::{anyhow, Result};

#[wasm_bindgen(module = "/js/wallet_bridge.js")]
extern "C" {
    // These functions return JS Promise objects
    #[wasm_bindgen(js_name = requestAccounts)]
    fn js_request_accounts() -> Promise;

    #[wasm_bindgen(js_name = getAccounts)]
    fn js_get_accounts() -> Promise;

    #[wasm_bindgen(js_name = signMessage)]
    fn js_sign_message(msg: &str) -> Promise;

    #[wasm_bindgen(js_name = signTypedData)]
    fn js_sign_typed_data(account: &str, typed_json: &str) -> Promise;

    #[wasm_bindgen(js_name = getChainId)]
    fn js_get_chain_id() -> Promise;
}

async fn promise_to_string(p: Promise) -> Result<String> {
    let v = JsFuture::from(p).await.map_err(|e| anyhow!("JS promise rejected: {:?}", e))?;
    // If result is object with signature property, handle both string or object:
    if v.is_string() {
        let s: String = v.as_string().unwrap_or_default();
        return Ok(s);
    }
    // If object { signature: "...", account: "0x..." }
    if v.is_object() {
        let sig = js_sys::Reflect::get(&v, &JsValue::from_str("signature"))
            .ok()
            .and_then(|s| s.as_string());
        if let Some(s) = sig {
            return Ok(s);
        }
        // fallback: stringify value
        return Ok(js_sys::JSON::stringify(&v).map_err(|e| anyhow!("json stringify err: {:?}", e))?.as_string().unwrap_or_default());
    }
    Err(anyhow!("unexpected JS return type"))
}

pub async fn request_accounts() -> Result<Vec<String>> {
    let p = js_request_accounts();
    let v = JsFuture::from(p).await.map_err(|e| anyhow!("JS promise rejected: {:?}", e))?;
    // Expect array of accounts
    if v.is_object() {
        let arr = js_sys::Array::from(&v);
        let mut out = Vec::new();
        for i in 0..arr.length() {
            if let Some(s) = arr.get(i).as_string() {
                out.push(s);
            }
        }
        return Ok(out);
    }
    Err(anyhow!("unexpected accounts return"))
}

pub async fn get_accounts() -> Result<Vec<String>> {
    let p = js_get_accounts();
    let v = JsFuture::from(p).await.map_err(|e| anyhow!("JS promise rejected: {:?}", e))?;
    if v.is_object() {
        let arr = js_sys::Array::from(&v);
        let mut out = Vec::new();
        for i in 0..arr.length() {
            if let Some(s) = arr.get(i).as_string() {
                out.push(s);
            }
        }
        return Ok(out);
    }
    Err(anyhow!("unexpected accounts return"))
}

pub async fn sign_message(message: &str) -> Result<String> {
    let p = js_sign_message(message);
    promise_to_string(p).await
}

pub async fn sign_typed_data(account: &str, typed_json: &str) -> Result<String> {
    let p = js_sign_typed_data(account, typed_json);
    promise_to_string(p).await
}

pub async fn get_chain_id() -> Result<String> {
    let p = js_get_chain_id();
    let s = JsFuture::from(p).await.map_err(|e| anyhow!("chainId promise rejected: {:?}", e))?;
    s.as_string().ok_or_else(|| anyhow!("no chain id"))
}
