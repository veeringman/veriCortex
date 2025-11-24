// src/components/wallet.rs
use dioxus::prelude::*;
use crate::utils::wallet;
use serde_json::json;

pub fn WalletPanel(cx: Scope) -> Element {
    let addr = use_state(&cx, || String::new());
    let signature = use_state(&cx, || String::new());
    let status = use_state(&cx, || String::from("disconnected"));

    let connect = {
        let addr = addr.clone();
        let status = status.clone();
        async move {
            match wallet::request_accounts().await {
                Ok(accounts) => {
                    if !accounts.is_empty() {
                        addr.set(accounts[0].clone());
                        status.set("connected".to_string());
                    }
                }
                Err(e) => {
                    status.set(format!("error: {}", e));
                }
            }
        }
    };

    let do_connect = move |_| {
        let fut = connect.clone();
        cx.spawn(fut);
    };

    let sign = {
        let addr = addr.clone();
        let signature = signature.clone();
        let status = status.clone();
        async move {
            let a = addr.get().clone();
            if a.is_empty() {
                status.set("connect wallet first".to_string());
                return;
            }
            // Build a canonical message to sign: for demo use a JSON string
            let proof_example = json!({
                "model_id": "demo-model",
                "input_hash": "0xdeadbeef",
                "output_hash": "0xbeefdead",
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            let message = serde_json::to_string(&proof_example).unwrap();

            match wallet::sign_message(&message).await {
                Ok(sig) => {
                    signature.set(sig);
                    status.set("signed".to_string());
                }
                Err(e) => {
                    status.set(format!("sign error: {}", e));
                }
            }
        }
    };

    let do_sign = move |_| {
        let fut = sign.clone();
        cx.spawn(fut);
    };

    cx.render(rsx!(
        div { style: "padding: 1rem; background: #fff; border-radius: 8px; box-shadow: 0 2px 6px rgba(0,0,0,0.06);",
            h3 { "Wallet" }
            p { "Status: {status}" }
            button { onclick: do_connect, "Connect Wallet" }
            p { "Address: {addr}" }
            button { onclick: do_sign, "Sign Proof JSON (demo)" }
            p { "Signature:" }
            pre { "{signature}" }
        }
    ))
}
