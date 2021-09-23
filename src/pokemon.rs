use actix_web::HttpRequest;
use reqwest::StatusCode;
use reqwest_middleware::ClientWithMiddleware;
use serde::Deserialize;

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
