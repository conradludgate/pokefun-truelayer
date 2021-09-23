use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Config {
    #[structopt(short, long, env = "PORT", default_value = "8080")]
    pub port: u16,
}

/// Parse the environment/arguments into [`Config`]
pub fn parse() -> Result<Config, clap::Error> {
    Config::from_args_safe()
}
