use clap::Parser;
use std::{path::{Path, PathBuf}};
use home::home_dir;

mod ssh_key;


#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct SshKeyGenConfig {
    /// The ansible vault password that will be used to encrypt the keys
    valut_password: String,
    /// The file name that the key will be stored
    filename: Option<String>,
    /// Any comments that will be added to the ssh key
    comment: Option<String>,
    
}

fn default_ssh_directory() -> Option<PathBuf> {
    if let Some(home_dir) = home::home_dir() {
        Some(PathBuf::from(home_dir.as_path().join(".ssh/")))
    } else {
        None
    }
}

fn main() {
    dbg!(default_ssh_directory());

    // let ssh_config = SshKeyGenConfig::parse();
    
    // dbg!(ssh_config);
}
