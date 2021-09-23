use crate::{pokemon, translations::translate};
use actix_web::{error::ErrorInternalServerError, web, HttpRequest, Result};
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonInfo {
    pub name: String,
    pub description: String,
    pub is_legendary: bool,
    pub habitat: String,
}

impl PokemonInfo {
    pub fn translation(&self) -> &'static str {
        if self.habitat == "cave" || self.is_legendary {
            "yoda"
        } else {
            "shakespeare"
        }
    }
}

/// replace all whitespace in the string with normal ascii spaces
pub fn clean_description(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_whitespace() { ' ' } else { c })
        .collect()
}

impl From<pokemon::Species> for PokemonInfo {
    fn from(ps: pokemon::Species) -> Self {
        Self {
            name: ps.name,
            is_legendary: ps.is_legendary,
            habitat: ps.habitat.name,
            // get first english flavour text
            description: ps
                .flavor_text_entries
                .into_iter()
                .filter(|flavor| flavor.language.name == "en")
                .next()
                .map(|flavor| clean_description(&flavor.flavor_text))
                .unwrap_or_default(),
        }
    }
}

pub async fn get_pokemon(
    client: web::Data<ClientWithMiddleware>,
    req: HttpRequest,
    pokemon_name: web::Path<String>,
) -> Result<Option<web::Json<PokemonInfo>>> {
    match pokemon::get_species(&client, &req, &pokemon_name).await {
        Ok(Some(species)) => Ok(Some(web::Json(species.into()))),
        Ok(None) => Ok(None),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}

pub async fn get_pokemon_translated(
    client: web::Data<ClientWithMiddleware>,
    req: HttpRequest,
    pokemon_name: web::Path<String>,
) -> Result<Option<web::Json<PokemonInfo>>> {
    match pokemon::get_species(&client, &req, &pokemon_name).await {
        Ok(Some(species)) => {
            let mut info: PokemonInfo = species.into();

            match translate(&client, &req, info.translation(), &info.description).await {
                Ok(desc) => info.description = desc,
                Err(err) => warn!(%err, "error getting translation"),
            };

            Ok(Some(web::Json(info)))
        }
        Ok(None) => Ok(None),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}
