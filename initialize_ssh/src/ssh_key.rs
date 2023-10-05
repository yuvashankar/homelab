use anyhow::{anyhow, bail, Result};
use log::info;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::ansible::VarsFile;

pub const DEFAULT_SSH_FILE_NAME: &str = "ansible_user";

pub fn default_ssh_key_file_name() -> PathBuf {
    match home::home_dir() {
        Some(home_dir) => home_dir
            .as_path()
            .join(Path::new(".ssh").join(DEFAULT_SSH_FILE_NAME)),
        None => PathBuf::from(DEFAULT_SSH_FILE_NAME),
    }
}

pub fn store_ssh_key(var_file: &Path, ssh_priv_path: &Path) -> Result<(), std::io::Error> {
    if ssh_priv_path.exists() {
        info!(
            "ssh keys already exist in: {}, exiting",
            ssh_priv_path.display()
        );
        Ok(())
    } else {
        let yaml_file_contents = fs::read_to_string(var_file)?;

        let ssh_yaml_file_contents: VarsFile = serde_yaml::from_str(yaml_file_contents.as_str())
            .map_err(|e| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Cannot parse ssh_yaml file, {}", e),
                )
            })?;

        let public_key_file_name = ssh_priv_path.clone().with_extension("pub");

        fs::write(public_key_file_name, ssh_yaml_file_contents.ssh_public_key)?;
        fs::write(ssh_priv_path, ssh_yaml_file_contents.ssh_private_key)?;

        Ok(())
    }
}

pub fn create_ssh_key(filename: &Path, comment: &str) -> Result<(), anyhow::Error> {
    //Check that the file does not already exist
    if filename.exists() {
        bail!("SSH key already exists, exiting.");
    }

    let comment_arg = vec!["-C", comment];

    if let Some(file_path) = filename.to_str() {
        let file_arg = vec!["-f", file_path];

        Command::new("ssh-keygen")
            .args(["-t", "ed25519"])
            .args(comment_arg)
            .args(file_arg)
            .arg("-q")
            .args(["-N", "''"])
            .spawn()
            .map(|mut c| c.wait().expect("ssh-keygen command wasn't running"))?;
    } else {
        return Err(anyhow!("Cannot convert filepath {:?} to string", filename));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn test_ssh_keygen_does_not_overwrite_existing_file() {
        let mut some_file =
            NamedTempFile::new().expect("Should have been able to create a temp file");

        let _write_output = writeln!(some_file, "nonsense");
        let filename = PathBuf::from(some_file.path());
        let comment = String::new();

        let res = create_ssh_key(&filename, &comment);

        assert!(res.is_err());
    }

    #[test]
    fn test_ssh_keys_are_created() -> Result<(), std::io::Error> {
        let comment = "user_supplied_comment";
        let key_folder = TempDir::new()?;
        let file_path = &key_folder.path().join("test_ssh_key");
        let public_file_path = &key_folder.path().join("test_ssh_key").with_extension("pub");

        let _out =
            create_ssh_key(&file_path.as_path(), comment).expect("failed ssh-keygen command");

        let public_file_contents = fs::read_to_string(public_file_path.as_path())?;

        assert!(public_file_contents.contains(comment));

        Ok(())
    }

    #[test]
    fn test_store_ssh_keys_does_not_overwrite_file() {
        let mut existing_file = NamedTempFile::new().expect("Should be able to create temp file");
        let file_contents = "nonsense";

        let _write_output = writeln!(existing_file, "{file_contents}");

        let store_file_path = PathBuf::from(existing_file.path());
        let var_file_path = PathBuf::default();

        let result = store_ssh_key(&var_file_path.as_path(), &store_file_path);

        let result_file_contents =
            fs::read_to_string(store_file_path).expect("cannot read temp file");

        let stripped_file_contents = result_file_contents.lines().next().unwrap();

        assert_eq!(file_contents, stripped_file_contents);

        assert!(result.is_ok());
    }

    #[test]
    fn test_yaml_files_written() -> Result<(), std::io::Error> {
        let test_input = VarsFile {
            ssh_public_key: String::from("public stuff"),
            ssh_private_key: String::from("private_tuff"),
        };

        // Write yaml file
        let ssh_key_folder_path = TempDir::new()?;

        let ssh_private_path = ssh_key_folder_path.path().join("test_ssh_key");

        let yaml_vars_file = NamedTempFile::new()?;

        let file_contents = serde_yaml::to_string(&test_input).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unable to parse the public and private keys into a yaml file: {e:?}"),
            )
        })?;
        fs::write(&yaml_vars_file, file_contents)?;

        let _out = store_ssh_key(yaml_vars_file.path(), ssh_private_path.as_path())?;

        let private_raw_string = fs::read_to_string(ssh_private_path.as_path())?;
        let public_raw_string =
            fs::read_to_string(ssh_private_path.as_path().with_extension("pub"))?;

        assert_eq!(
            private_raw_string.lines().next().unwrap(),
            test_input.ssh_private_key
        );
        assert_eq!(
            public_raw_string.lines().next().unwrap(),
            test_input.ssh_public_key
        );

        Ok(())
    }
}
