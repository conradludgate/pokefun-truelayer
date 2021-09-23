use actix_http::{Method, Request};
use actix_web::{
    dev::{self, Service, ServiceResponse},
    test, Error,
};
use lazy_static::lazy_static;
use mockito::mock;
use reqwest::{Client, StatusCode};
use reqwest_middleware::ClientBuilder;
use reqwest_tracing::TracingMiddleware;

use crate::{api::PokemonInfo, new_service, AppConfig};

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

async fn create_test_app(
    app_config: &AppConfig,
) -> impl Service<Request, Response = ServiceResponse<dev::AnyBody>, Error = Error> {
    setup_tracing();

    let client = Client::builder()
        .build()
        .expect("client build successfully");
    let client = ClientBuilder::new(client).with(TracingMiddleware).build();

    test::init_service(new_service(client, app_config)).await
}

lazy_static! {
    static ref MOCK_CONFIG: AppConfig = AppConfig {
        pokemon_url: mockito::server_url().into(),
        translations_url: mockito::server_url().into(),
    };
    static ref PROD_CONFIG: AppConfig = AppConfig {
        pokemon_url: "https://pokeapi.co".into(),
        translations_url: "https://api.funtranslations.com".into(),
    };
}

#[actix_rt::test]
async fn get_pokemon_ok_mocked() {
    let _m = mock("GET", "/api/v2/pokemon-species/mewtwo/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("replays/mewtwo.json")
        .create();

    let app = create_test_app(&MOCK_CONFIG).await;

    let req = test::TestRequest::with_uri("/pokemon/mewtwo")
        .method(Method::GET)
        .to_request();

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

#[actix_rt::test]
async fn get_pokemon_not_found_mocked() {
    let _m1 = mock("GET", "/api/v2/pokemon-species/mewthree/")
        .with_status(404)
        .create();

    let app = create_test_app(&MOCK_CONFIG).await;

    let req = test::TestRequest::with_uri("/pokemon/mewthree")
        .method(Method::GET)
        .to_request();

    let resp: ServiceResponse = app.call(req).await.expect("valid response");

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
async fn get_pokemon_translated_legendary_mocked() {
    let _m1 = mock("GET", "/api/v2/pokemon-species/mewtwo/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("replays/mewtwo.json")
        .create();

    let _m2 = mock("GET", "/translate/yoda")
        .match_body("text=It+was+created+by+a+scientist+after+years+of+horrific+gene+splicing+and+DNA+engineering+experiments.")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("replays/mewtwo_yoda.json")
        .create();

    let app = create_test_app(&MOCK_CONFIG).await;

    let req = test::TestRequest::with_uri("/pokemon/translated/mewtwo")
        .method(Method::GET)
        .to_request();

    let resp: ServiceResponse = app.call(req).await.expect("valid response");

    assert_eq!(resp.status(), StatusCode::OK);

    let result: PokemonInfo = test::read_body_json(resp).await;

    assert_eq!(result, PokemonInfo {
        name: "mewtwo".into(),
        description: "Created by a scientist after years of horrific gene splicing and dna engineering experiments,  it was.".into(),
        is_legendary: true,
        habitat: "rare".into(),
    })
}

#[actix_rt::test]
async fn get_pokemon_translated_cave_mocked() {
    let _m1 = mock("GET", "/api/v2/pokemon-species/zubat/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("replays/zubat.json")
        .create();

    let _m2 = mock("GET", "/translate/yoda")
        .match_body("text=Forms+colonies+in+perpetually+dark+places.+Uses+ultrasonic+waves+to+identify+and+approach+targets.")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("replays/zubat_yoda.json")
        .create();

    let app = create_test_app(&MOCK_CONFIG).await;

    let req = test::TestRequest::with_uri("/pokemon/translated/zubat")
        .method(Method::GET)
        .to_request();

    let resp: ServiceResponse = app.call(req).await.expect("valid response");

    assert_eq!(resp.status(), StatusCode::OK);

    let result: PokemonInfo = test::read_body_json(resp).await;

    assert_eq!(result, PokemonInfo {
        name: "zubat".into(),
        description: "Forms colonies in perpetually dark places.Ultrasonic waves to identify and approach targets,  uses.".into(),
        is_legendary: false,
        habitat: "cave".into(),
    })
}

#[actix_rt::test]
async fn get_pokemon_translated_other_mocked() {
    let _m1 = mock("GET", "/api/v2/pokemon-species/ditto/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("replays/ditto.json")
        .create();

    let _m2 = mock("GET", "/translate/shakespeare")
        .match_body("text=It+can+freely+recombine+its+own+cellular+structure+to+transform+into+other+life-forms.")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("replays/ditto_shakespeare.json")
        .create();

    let app = create_test_app(&MOCK_CONFIG).await;

    let req = test::TestRequest::with_uri("/pokemon/translated/ditto")
        .method(Method::GET)
        .to_request();

    let resp: ServiceResponse = app.call(req).await.expect("valid response");

    assert_eq!(resp.status(), StatusCode::OK);

    let result: PokemonInfo = test::read_body_json(resp).await;

    assert_eq!(result, PokemonInfo {
        name: "ditto".into(),
        description: "'t can freely recombine its own cellular structure to transform into other life-forms.".into(),
        is_legendary: false,
        habitat: "urban".into(),
    })
}

#[actix_rt::test]
async fn get_pokemon_translated_not_found_mocked() {
    let _m1 = mock("GET", "/api/v2/pokemon-species/mewthree/")
        .with_status(404)
        .create();

    let app = create_test_app(&MOCK_CONFIG).await;

    let req = test::TestRequest::with_uri("/pokemon/translated/mewthree")
        .method(Method::GET)
        .to_request();

    let resp: ServiceResponse = app.call(req).await.expect("valid response");

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
async fn get_pokemon_translated_bad_translation_mocked() {
    let _m1 = mock("GET", "/api/v2/pokemon-species/mewtwo/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("replays/mewtwo.json")
        .create();

    let _m2 = mock("GET", "/translate/yoda")
            .match_body("text=It+was+created+by+a+scientist+after+years+of+horrific+gene+splicing+and+DNA+engineering+experiments.")
            .with_status(500)
            .create();

    let app = create_test_app(&MOCK_CONFIG).await;

    let req = test::TestRequest::with_uri("/pokemon/translated/mewtwo")
        .method(Method::GET)
        .to_request();

    let resp: ServiceResponse = app.call(req).await.expect("valid response");

    let result: PokemonInfo = test::read_body_json(resp).await;

    assert_eq!(result, PokemonInfo {
        name: "mewtwo".into(),
        description: "It was created by a scientist after years of horrific gene splicing and DNA engineering experiments.".into(),
        is_legendary: true,
        habitat: "rare".into(),
    })
}

#[actix_rt::test]
#[ignore]
async fn get_pokemon_ok() {
    let app = create_test_app(&PROD_CONFIG).await;

    let req = test::TestRequest::with_uri("/pokemon/mewtwo")
        .method(Method::GET)
        .to_request();

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

#[actix_rt::test]
#[ignore]
async fn get_pokemon_not_found() {
    let app = create_test_app(&PROD_CONFIG).await;

    let req = test::TestRequest::with_uri("/pokemon/mewthree")
        .method(Method::GET)
        .to_request();

    let resp: ServiceResponse = app.call(req).await.expect("valid response");

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
#[ignore]
async fn get_pokemon_translated_legendary() {
    let app = create_test_app(&PROD_CONFIG).await;

    let req = test::TestRequest::with_uri("/pokemon/translated/mewtwo")
        .method(Method::GET)
        .to_request();

    let resp: ServiceResponse = app.call(req).await.expect("valid response");

    assert_eq!(resp.status(), StatusCode::OK);

    let result: PokemonInfo = test::read_body_json(resp).await;

    assert_eq!(result, PokemonInfo {
        name: "mewtwo".into(),
        description: "Created by a scientist after years of horrific gene splicing and dna engineering experiments,  it was.".into(),
        is_legendary: true,
        habitat: "rare".into(),
    })
}

#[actix_rt::test]
#[ignore]
async fn get_pokemon_translated_cave() {
    let app = create_test_app(&PROD_CONFIG).await;

    let req = test::TestRequest::with_uri("/pokemon/translated/zubat")
        .method(Method::GET)
        .to_request();

    let resp: ServiceResponse = app.call(req).await.expect("valid response");

    assert_eq!(resp.status(), StatusCode::OK);

    let result: PokemonInfo = test::read_body_json(resp).await;

    assert_eq!(result, PokemonInfo {
        name: "zubat".into(),
        description: "Forms colonies in perpetually dark places.Ultrasonic waves to identify and approach targets,  uses.".into(),
        is_legendary: false,
        habitat: "cave".into(),
    })
}

#[actix_rt::test]
#[ignore]
async fn get_pokemon_translated_other() {
    let app = create_test_app(&PROD_CONFIG).await;

    let req = test::TestRequest::with_uri("/pokemon/translated/ditto")
        .method(Method::GET)
        .to_request();

    let resp: ServiceResponse = app.call(req).await.expect("valid response");

    assert_eq!(resp.status(), StatusCode::OK);

    let result: PokemonInfo = test::read_body_json(resp).await;

    assert_eq!(result, PokemonInfo {
        name: "ditto".into(),
        description: "'t can freely recombine its own cellular structure to transform into other life-forms.".into(),
        is_legendary: false,
        habitat: "urban".into(),
    })
}

#[actix_rt::test]
#[ignore]
async fn get_pokemon_translated_not_found() {
    let app = create_test_app(&PROD_CONFIG).await;

    let req = test::TestRequest::with_uri("/pokemon/translated/mewthree")
        .method(Method::GET)
        .to_request();

    let resp: ServiceResponse = app.call(req).await.expect("valid response");

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
