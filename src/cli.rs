use std::{
    path::{Path, PathBuf},
    process::Command,
};

/// Defines the command-line interface for the application, specifying all
/// possible arguments and options.
#[derive(clap::Parser, Default, Debug, Clone)]
//#[command(name = "filessh")]
pub struct Cli {
    /// The remote host to connect to (e.g., 'example.com' or '192.168.1.100').
    #[clap(index = 1)]
    pub host: String,

    /// The port number to use for the SSH connection.
    #[clap(long, short, default_value_t = 22)]
    pub port: u16,

    /// The username for logging into the remote host.
    /// If not provided, the SSH client may use the current local username or a
    /// default specified in SSH configuration.
    #[clap(long, short)]
    pub username: Option<String>,

    /// The path to the private key file for public key authentication.
    #[clap(long, short = 'k')]
    pub private_key: PathBuf,

    /// An optional path to an OpenSSH certificate file for authentication.
    #[clap(long, short = 'o')]
    pub openssh_certificate: Option<PathBuf>,

    /// The initial directory path to open on the remote host after connecting.
    #[clap(index = 2, required = true)]
    pub path: PathBuf,

    /// Install the man pages for the application
    #[clap(long, default_value_t = false)]
    pub install_man_pages: bool,
}

impl Cli {
    pub fn build_ssh_command(&self) -> Command {
        let mut cmd = Command::new("ssh");

        if let Some(username) = &self.username {
            cmd.arg("-l").arg(username);
        }

        cmd.arg("-p").arg(self.port.to_string());
        cmd.arg("-i").arg(self.private_key.display().to_string());

        // Use user@host or fallback to "root@host"
        let user = self.username.as_deref().unwrap_or("root");
        cmd.arg(format!("{user}@{}", self.host));

        cmd
    }

    pub fn build_ssh_with_path<P>(&self, path: P) -> Command
    where
        P: AsRef<Path>,
    {
        let mut cmd = self.build_ssh_command();

        // Build the remote command: cd <path>; bash --login
        let remote_cmd = format!("cd {}; bash --login", path.as_ref().display());

        cmd.arg("-t").arg(remote_cmd);
        cmd
    }
}
