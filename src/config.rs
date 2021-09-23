use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Config {
    #[structopt(short, long, env = "PORT", default_value = "8080")]
    pub port: u16,

    #[structopt(long, env = "POKEMON_API", default_value = "https://pokeapi.co")]
    pub pokemon_api: String,

    #[structopt(long, env = "TRANSLATIONS_API", default_value = "https://api.funtranslations.com")]
    pub translations_url: String,
}

/// Parse the environment/arguments into [`Config`]
pub fn parse() -> Result<Config, clap::Error> {
    Config::from_args_safe()
}
