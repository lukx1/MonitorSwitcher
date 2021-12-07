use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "1.0", author = "Lukx <lukx@lukx.net>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(short, long, default_value="Switcher.toml")]
    pub config: String,
    #[clap(short = 'p', long)]
    pub force_prod: Option<bool>,
    #[clap(short = 'd', long)]
    pub force_dev: Option<bool>
}