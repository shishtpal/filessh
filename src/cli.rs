use std::path::PathBuf;

#[derive(clap::Parser, Default, Debug, Clone)]
pub struct Cli {
    #[clap(index = 1)]
    pub host: String,

    #[clap(long, short, default_value_t = 22)]
    pub port: u16,

    #[clap(long, short)]
    pub username: Option<String>,

    #[clap(long, short = 'k')]
    pub private_key: PathBuf,

    #[clap(long, short = 'o')]
    pub openssh_certificate: Option<PathBuf>,

    #[clap(index = 2, required = true)]
    pub path: PathBuf,
}
