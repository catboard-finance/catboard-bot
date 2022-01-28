use cfg_if::cfg_if;
use cloudflare::worker::WorkersKv;
use context::Context;

use http::HttpResponse;
use wasm_bindgen::prelude::*;

use js_sys::Promise;
use wasm_bindgen_futures::future_to_promise;

use crate::cloudflare::worker::WorkersKvJs;

mod catboard;
mod cloudflare;
mod context;
mod discord;
mod error;
mod http;
mod pyth;
mod solana;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub async fn wasm_main(context: JsValue, kv: WorkersKvJs) -> Promise {
    future_to_promise(async move {
        let value = JsValue::from_serde(
            &(match context.into_serde::<Context>() {
                Ok(ctx) => {
                    let kv = WorkersKv { kv };
                    ctx.handle_http_request(&kv).await
                }

                Err(error) => HttpResponse {
                    status: 400,
                    body: error.to_string(),
                },
            }),
        )
        .unwrap();

        Ok(value.into())
    })
}
