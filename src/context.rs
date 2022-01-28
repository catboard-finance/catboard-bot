use serde::Deserialize;
use web_sys::Url;

use std::collections::HashMap;

use crate::{
    catboard::publisher::{fetch_pyth_prices_and_record, fetch_pyth_product_and_record},
    cloudflare::worker::WorkersKv,
    discord::{interaction::Interaction, verification::verify_signature},
    error::Error,
    http::{HttpError, HttpRequest, HttpResponse},
    solana::web3::Cluster,
};

#[derive(Deserialize)]
pub(crate) struct Context {
    pub(crate) env: HashMap<String, String>,
    pub(crate) request: HttpRequest,
}

impl Context {
    fn env(&self, key: &str) -> Result<&String, Error> {
        self.env
            .get(key)
            .ok_or_else(|| Error::EnvironmentVariableNotFound(key.to_string()))
    }

    fn perform_verification(&self) -> Result<(), Error> {
        let public_key = self.env("PUBLIC_KEY")?;
        let signature = self.request.header("x-signature-ed25519")?;
        let timestamp = self.request.header("x-signature-timestamp")?;

        verify_signature(public_key, signature, timestamp, &self.request.body)
            .map_err(Error::VerificationFailed)
    }

    #[allow(dead_code)]
    fn perform_api_verification(&self, _request_api_key: &str) -> Result<(), Error> {
        // Will temporary use PUBLIC_KEY as API_KEY
        // TODO: verify with signature
        // let api_key = self.env("PUBLIC_KEY").unwrap();

        // verify_api_key(request_api_key, api_key).map_err(Error::VerificationFailed)
        Ok(())
    }

    async fn handle_payload(&self, kv: &WorkersKv) -> Result<String, Error> {
        let payload = &self.request.body;
        let interaction =
            serde_json::from_str::<Interaction>(payload).map_err(Error::JsonFailed)?;
        let response = interaction.perform(kv).await;

        serde_json::to_string(&response.unwrap()).map_err(Error::JsonFailed)
    }

    async fn handle_api_payload(&self, kv: &WorkersKv, fn_name: &str) -> Result<String, Error> {
        let response = match fn_name {
            "sync_products" => {
                let symbols = self.env("SYMBOLS")?;
                let symbols = symbols.split(",").collect();
                fetch_pyth_product_and_record(kv, &Cluster::Devnet, symbols).await
            }
            "sync_prices" => {
                let symbols = self.env("SYMBOLS")?;
                let symbols = symbols.split(",").collect();
                fetch_pyth_prices_and_record(kv, &Cluster::Devnet, symbols).await
            }
            _ => todo!(),
            // _ => Ok(hello().await).map_err(Error::JsonFailed),
        };

        serde_json::to_string(&response.unwrap()).map_err(Error::JsonFailed)
    }

    pub(crate) async fn handle_signed_http_request(&self, kv: &WorkersKv) -> HttpResponse {
        let verified_result = self.perform_verification().map_err(HttpError::from);
        match verified_result {
            Ok(_) => {
                let result = self.handle_payload(kv).await.map_err(HttpError::from);

                match result {
                    Ok(body) => HttpResponse { status: 200, body },
                    Err(error) => HttpResponse {
                        body: error.to_string(),
                        status: error.status as u16,
                    },
                }
            }
            Err(error) => HttpResponse {
                body: error.to_string(),
                status: error.status as u16,
            },
        }
    }

    #[allow(dead_code)]
    pub(crate) async fn handle_api_http_request(
        &self,
        kv: &WorkersKv,
        fn_name: &str,
    ) -> HttpResponse {
        let request_api_key = self.request.header("x-api-key");

        let verified_result = self
            .perform_api_verification(request_api_key.unwrap())
            .map_err(HttpError::from);

        match verified_result {
            Ok(_) => {
                let result = self
                    .handle_api_payload(kv, fn_name)
                    .await
                    .map_err(HttpError::from);

                match result {
                    Ok(body) => HttpResponse { status: 200, body },
                    Err(error) => HttpResponse {
                        body: error.to_string(),
                        status: error.status as u16,
                    },
                }
            }
            Err(error) => HttpResponse {
                body: error.to_string(),
                status: error.status as u16,
            },
        }
    }

    pub(crate) async fn handle_internal_http_request(
        &self,
        kv: &WorkersKv,
        fn_name: &str,
    ) -> HttpResponse {
        let result = self
            .handle_api_payload(kv, fn_name)
            .await
            .map_err(HttpError::from);

        match result {
            Ok(body) => HttpResponse { status: 200, body },
            Err(error) => HttpResponse {
                body: error.to_string(),
                status: error.status as u16,
            },
        }
    }

    pub(crate) async fn handle_http_request(&self, kv: &WorkersKv) -> HttpResponse {
        let url = Url::new(&self.request.url).unwrap();
        let pathname = url.pathname();
        let pathname_str = pathname.as_str();

        // pathname = /api/sync_prices
        let paths: Vec<&str> = pathname_str.split('/').collect();
        let response = match paths[1] {
            "api" => self.handle_internal_http_request(kv, paths[2]).await,
            _ => self.handle_signed_http_request(kv).await,
        };

        response

        // let request_api_key = self.request.header("x-api-key").map_err(|_|=>Err(());
        // if let None = Some(request_api_key) {
        //     self.handle_signed_http_request(kv).await
        // } else {
        //     self.handle_api_http_request(kv).await
        // }
        // if request_api_key.len() > 0 {
        //     return self.handle_api_http_request(kv).await;
        // } else {
        //     return self.handle_signed_http_request(kv).await;
        // }
        // return self.handle_signed_http_request(kv).await;
    }
}
