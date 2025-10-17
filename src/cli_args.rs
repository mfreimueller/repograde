use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "", help = "The start date used to calculate differences from. The default value is yesterdays' date, therefor the change set since yesterday is used.")]
    pub(crate) date: String,
    #[arg(short, long, default_value = "repograde.toml", help = "The location of the config file.")]
    pub(crate) config_file: String,
    #[arg(long, default_value = "False", help = "If true, only outputs the grading result to stdout, instead of writing it to the CSV file.")]
    pub(crate) dry_run: bool,
}