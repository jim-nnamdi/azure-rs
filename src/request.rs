use anyhow::{Ok, Result};
use async_trait::async_trait;

#[async_trait]
pub trait AzureRequest {
    fn get_token() -> &'static str;
    async fn fetch_get_request(path: String) -> Result<String>;
    async fn make_post_request(path: String, body: &'static str) -> Result<String>;
}

pub struct Azureclient;

#[async_trait]
impl AzureRequest for Azureclient {
    fn get_token() -> &'static str {
        return "";
    }

    async fn fetch_get_request(path: String) -> Result<String> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert(
            "Authorization",
            format!("Bearer {}", Self::get_token()).parse().unwrap(),
        );
        let req = reqwest::Client::new()
            .get(path)
            .headers(headers)
            .send()
            .await;
        if req.is_ok() {
            let data = req.ok().unwrap().text().await?;
            return Ok(data);
        }
        let req_error = "could not make request successfully";
        Ok(req_error.to_string())
    }

    async fn make_post_request(path: String, body: &'static str) -> Result<String> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert(
            "Authorization",
            format!("Bearer {}", Self::get_token()).parse().unwrap(),
        );
        let req = reqwest::Client::new()
            .post(path)
            .headers(headers)
            .body(body)
            .send()
            .await;
        if req.is_ok() {
            let data = req.ok().unwrap().text().await?;
            return Ok(data);
        }
        let req_error = "could not make request successfully";
        Ok(req_error.to_string())
    }
}
