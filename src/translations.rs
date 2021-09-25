use actix_web::{HttpRequest, Result};
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};

/// Make a POST request for a fun-translation.
///
/// # Errors:
/// Will return [`Err`] if the http connection could not be made,
/// if the API responded with an error status code
/// or if the response body contained invalid JSON.
pub async fn translate(
    client: &ClientWithMiddleware,
    req: &HttpRequest,
    translation: &str,
    text: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    Ok(client
        .get(req.url_for("translations", &[translation])?)
        .query(&Request { text })
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
