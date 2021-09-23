use actix_web::{HttpRequest, Result};
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};

pub async fn translate(
    client: &ClientWithMiddleware,
    req: &HttpRequest,
    translation: &str,
    text: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    Ok(client
        .post(req.url_for("translations", &[translation])?)
        .form(&Request { text })
        .send()
        .await?
        .error_for_status()?
        .json::<Response>()
        .await?
        .contents
        .translated)
}

#[derive(Debug, Serialize)]
struct Request<'a> {
    text: &'a str,
}

#[derive(Debug, Deserialize)]
struct Response {
    contents: ResponseContents,
}

#[derive(Debug, Deserialize)]
struct ResponseContents {
    translated: String,
}