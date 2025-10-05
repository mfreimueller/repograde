use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "")]
    pub(crate) date: String,
    #[arg(short, long, default_value = "repograde.toml")]
    pub(crate) config_file: String,
}