use actix_http::Request;
use actix_web::{
    dev::{self, Service, ServiceResponse},
    test, Error,
};
use reqwest::{Client, StatusCode};
use reqwest_middleware::ClientBuilder;
use reqwest_tracing::TracingMiddleware;

use crate::{APP_CONFIG, AppConfig, api::PokemonInfo, new_service};

use std::sync::Once;

static TRACING: Once = Once::new();

fn setup_tracing() {
    TRACING.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter("trace")
            .with_test_writer()
            .init();
    });
}

async fn create_test_app(app_config: &AppConfig) -> impl Service<Request, Response = ServiceResponse<dev::AnyBody>, Error = Error> {
    setup_tracing();

    let client = Client::builder()
        .build()
        .expect("client build successfully");
    let client = ClientBuilder::new(client).with(TracingMiddleware).build();

    test::init_service(new_service(client, app_config)).await
}

#[actix_rt::test]
async fn get_pokemon_ok() {
    let app = create_test_app(&APP_CONFIG).await;

    let req = test::TestRequest::with_uri("/pokemon/mewtwo").to_request();

    let resp: ServiceResponse = app.call(req).await.expect("valid response");

    assert_eq!(resp.status(), StatusCode::OK);

    let result: PokemonInfo = test::read_body_json(resp).await;

    assert_eq!(result, PokemonInfo {
        name: "mewtwo".into(),
        description: "It was created by a scientist after years of horrific gene splicing and DNA engineering experiments.".into(),
        is_legendary: true,
        habitat: "rare".into(),
    })
}