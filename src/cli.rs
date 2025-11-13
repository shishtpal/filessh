use std::{
    path::{Path, PathBuf},
    process::Command,
};

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
