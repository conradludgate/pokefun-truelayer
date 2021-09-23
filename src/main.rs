use std::borrow::Cow;

use actix_web::{App, Error, HttpServer, Result, dev::{self, ServiceFactory}, web};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_tracing::TracingMiddleware;

mod api;
mod config;
mod pokemon;
mod translations;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the app config
    let config = config::parse()?;

    // Initialise stdout logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Create a new reqwest client with logging
    let client = reqwest::Client::builder().build()?;
    let client = ClientBuilder::new(client).with(TracingMiddleware).build();

    // Create a http server and await the future
    Ok(
        HttpServer::new(move || new_service(client.clone(), &APP_CONFIG))
            .bind(("0.0.0.0", config.port))?
            .run()
            .await?,
    )
}

/// Default [`AppConfig`] with the production api endpoints configured
pub static APP_CONFIG: AppConfig = AppConfig {
    pokemon_url: Cow::Borrowed("https://pokeapi.co"),
    translations_url: Cow::Borrowed("https://api.funtranslations.com"),
};

#[derive(Clone)]
pub struct AppConfig {
    pokemon_url: Cow<'static, str>,
    translations_url: Cow<'static, str>,
}

/// Create a new actix_web App Service.
/// Configuring it in a function allows for easy access to the app service for testing
pub fn new_service(
    client: ClientWithMiddleware,
    api_config: &AppConfig,
) -> App<
    impl ServiceFactory<
        dev::ServiceRequest,
        Config = (),
        Response = dev::ServiceResponse<dev::AnyBody>,
        Error = Error,
        InitError = (),
    >,
    dev::AnyBody,
> {
    App::new()
        .app_data(web::Data::new(client))
        .external_resource(
            "pokemon_species",
            api_config.pokemon_url.to_string() + "/api/v2/pokemon-species/{pokemon_name}/",
        )
        .external_resource(
            "translations",
            api_config.translations_url.to_string() + "/translate/{translation}",
        )
        .service(api::get_pokemon)
        .service(api::get_pokemon_translated)
}

#[cfg(test)]
mod tests;
