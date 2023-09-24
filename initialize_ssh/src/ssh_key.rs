use anyhow::{anyhow, bail, Result};
use log::warn;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const DEFAULT_SSH_FILE_NAME: &str = "ansible_user";

#[derive(Debug)]
pub struct SshKeyGenConfig {
    /// The file name that the key will be stored
    pub filename: PathBuf,
    /// Any comments that will be added to the ssh key
    pub comment: String,
}

pub fn generate_key_file_name(file_name: Option<PathBuf>) -> PathBuf {
    match home::home_dir() {
        Some(home_dir) => match file_name {
            Some(f) => PathBuf::from(home_dir.as_path().join(Path::new(".ssh")).join(f.as_path())),
            None => PathBuf::from(
                home_dir
                    .as_path()
                    .join(Path::new(".ssh").join(DEFAULT_SSH_FILE_NAME)),
            ),
        },
        None => {
            warn!("Home directory could not be found, perhaps you don't have access to it? ");
            PathBuf::from(DEFAULT_SSH_FILE_NAME)
        }
    }
}

pub fn create_ssh_key(ssh_options: SshKeyGenConfig) -> Result<(), anyhow::Error> {
    //Check that the file does not already exist
    if ssh_options.filename.exists() {
        bail!("SSH key already exists, exiting.");
    }

    let comment_arg = vec!["-C", ssh_options.comment.as_str()];

    if let Some(file_path) = ssh_options.filename.as_path().to_str() {
        let file_arg = vec!["-f", file_path];

        Command::new("ssh-keygen")
            .args(["-t", "ed25519"])
            .args(comment_arg)
            .args(file_arg)
            .arg("-q")
            .args(["-N", "''"])
            .spawn()
            .expect("Failed to execute ssh-keygen command");
    } else {
        return Err(anyhow!(
            "Cannot convert filepath {:?} to string",
            ssh_options.filename
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_existing_file_not_overwritten() {
        let mut some_file =
            NamedTempFile::new().expect("Should have been able to create a temp file");

        let _write_output = writeln!(some_file, "nonsense");

        let config = SshKeyGenConfig {
            filename: PathBuf::from(some_file.path()),
            comment: String::new(),
        };

        let res = create_ssh_key(config);

        assert!(res.is_err());
    }

    #[test]
    fn test_no_ssh_filename() {
        let default_path = PathBuf::from(
            home::home_dir()
                .unwrap()
                .as_path()
                .join(Path::new(".ssh").join(DEFAULT_SSH_FILE_NAME)),
        );
        let filename = generate_key_file_name(None);

        assert_eq!(default_path, filename);
    }

    #[test]
    fn test_ssh_filename_is_added() {
        let filename = PathBuf::from("test_name");
        let expected_file_path = PathBuf::from(
            home::home_dir()
                .unwrap()
                .as_path()
                .join(Path::new(".ssh").join(filename.clone())),
        );

        let output = generate_key_file_name(Some(filename));

        assert_eq!(expected_file_path, output);
    }
}
