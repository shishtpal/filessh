use crate::ssh::Session;
use crate::{cli::Cli, files::FileEntry};
use clap::Parser;
use color_eyre::eyre::Result;
use russh_sftp::client::SftpSession;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tracing::info;

mod cli;
mod files;
mod logging;
mod ssh;
mod tui;

fn main() -> Result<()> {
    logging::init()?;

    info!("Starting...");
    let cli = Cli::parse();

    info!("Connecting to {}:{}", cli.host, cli.port);
    info!("Key path: {:?}", cli.private_key);
    info!("OpenSSH Certificate path: {:?}", cli.openssh_certificate);

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    crate::tui::tui(cli.path.display().to_string(), cli, rt)?;
    Ok(())
}
