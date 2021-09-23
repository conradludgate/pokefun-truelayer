use actix_web::HttpRequest;
use reqwest::StatusCode;
use reqwest_middleware::ClientWithMiddleware;
use serde::Deserialize;

/// Make a GET request to the pokeapi for the provided pokemon species
///
/// # Errors:
/// Will return [`Err`] if the http connection could not be made,
/// if the API responded with an error status code
/// or if the response body contained invalid JSON.
///
/// Will return [`Ok(None)`] if the API returned a 404 status code
pub async fn get_species(
    client: &ClientWithMiddleware,
    req: &HttpRequest,
    pokemon_name: &str,
) -> Result<Option<Species>, Box<dyn std::error::Error>> {
    let resp = client
        .get(req.url_for("pokemon_species", &[pokemon_name])?)
        .send()
        .await?;

    match resp.status() {
        StatusCode::NOT_FOUND => Ok(None),
        _ => Ok(Some(resp.error_for_status()?.json().await?)),
    }
}

#[derive(Debug, Deserialize)]
pub struct Habitat {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct FlavorText {
    pub flavor_text: String,
    pub language: Language,
}

#[derive(Debug, Deserialize)]
pub struct Language {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Species {
    pub name: String,
    pub is_legendary: bool,
    pub habitat: Habitat,
    pub flavor_text_entries: Vec<FlavorText>,
}
