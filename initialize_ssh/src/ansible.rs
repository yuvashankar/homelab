use anyhow::bail;
use core::fmt;
use serde::Serialize;
use std::io::{Error, ErrorKind};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[cfg(not(test))]
const DEFAULT_SSH_VAR_YAML_FILE: &str = "/workspaces/homelab/ansible/vars/ssh_vars.yaml";
#[cfg(not(test))]
const VAULT_PASS_FILENAME: &str = ".vault_pass.txt";

#[cfg(test)]
const DEFAULT_SSH_VAR_YAML_FILE: &str = "/tmp/temm_ssh_yaml.yaml";
#[cfg(test)]
const VAULT_PASS_FILENAME: &str = "/tmp/vault_pass_file_test/.test_vault_pass.txt";

#[derive(Debug, PartialEq, Serialize)]
struct VarsFile {
    ssh_public_key: String,
    ssh_private_key: String,
}

pub fn get_password(file_path: Option<PathBuf>) -> Result<String, Error> {
    if let Some(password_file) = file_path {
        match fs::read_to_string(password_file) {
            Ok(pass) => {
                if let Some(password) = pass.lines().next() {
                    Ok(String::from(password))
                } else {
                    Err(Error::new(ErrorKind::Other, "File may be empty"))
                }
            }
            Err(e) => Err(e),
        }
    } else {
        match rpassword::prompt_password("Enter the ansible vault password you would like to use: ")
        {
            Ok(password) => Ok(password),
            Err(e) => Err(e),
        }
    }
}

pub fn process_destination_path(dest_path: Option<PathBuf>) -> Result<PathBuf, anyhow::Error> {
    match dest_path {
        Some(file_path) => {
            if file_path.exists() {
                bail!(
                    "Destination file: {:?} already exists, please delete prior to running again. ",
                    file_path
                )
            } else {
                Ok(file_path)
            }
        }
        None => {
            let default_path = Path::new(DEFAULT_SSH_VAR_YAML_FILE);
            if default_path.exists() {
                bail!(
                    "Destination file {:?} already exists, please delete prior to running again. ",
                    default_path
                );
            } else {
                Ok(PathBuf::from(default_path))
            }
        }
    }
}

pub fn generate_ssh_yaml_file(
    ssh_key_source: &Path,
    dest_path: &Path,
) -> Result<(), serde_yaml::Error> {
    let private_key =
        fs::read_to_string(ssh_key_source).expect("Error reading ssh key private file");
    let public_key = fs::read_to_string(ssh_key_source.with_extension("pub"))
        .expect("Error reading ssh key public file");

    let file_contents = serde_yaml::to_string(&VarsFile {
        ssh_public_key: public_key,
        ssh_private_key: private_key,
    })?;
    let _output = fs::write(dest_path, file_contents).expect("Error writing yaml file");
    Ok(())
}

pub fn write_vault_password_file(vault_password: &str) -> Result<PathBuf, std::io::Error> {
    let save_path = if let Some(home_dir) = home::home_dir() {
        PathBuf::from(home_dir.as_path().join(VAULT_PASS_FILENAME))
    } else {
        PathBuf::from(VAULT_PASS_FILENAME)
    };

    if save_path.exists() {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            "Vault Password file already exists",
        ));
    }

    fs::write(&save_path, vault_password)?;
    Ok(save_path)
}

#[derive(Default, Debug)]
pub enum AnsibleVaultCommand {
    #[default]
    Encrypt,
    Decrypt,
}

impl fmt::Display for AnsibleVaultCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn vault_ssh_vars_file(
    ansible_vault_command: AnsibleVaultCommand,
    ssh_key_path: &Path,
    vault_pass_file: &Path,
) -> Result<(), std::io::Error> {
    let vault_pass_file = PathBuf::from(vault_pass_file);

    let vault_args = vec!["--vault-pass-file", vault_pass_file.to_str().unwrap()];

    let ansible_command = ansible_vault_command.to_string().trim().to_lowercase();

    Command::new("ansible-vault")
        .arg(ansible_command)
        .arg(ssh_key_path)
        .args(vault_args)
        .output()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::io::{Result, Write};
    use tempfile::{tempfile, NamedTempFile};

    #[test]
    fn test_get_password_reads_file() {
        let mut some_file =
            NamedTempFile::new().expect("Should have been able to create a temp file");

        let _write_result = writeln!(some_file, "nonsense");

        let output = get_password(Some(PathBuf::from(some_file.path())));

        assert_eq!(output.unwrap(), String::from("nonsense"));
    }

    #[test]
    fn test_empty_file_errors_out() {
        let some_file = NamedTempFile::new().expect("Should have been able to create a temp file");

        let output = get_password(Some(PathBuf::from(some_file.path())));

        assert!(output.is_err());
    }

    #[test]
    fn test_process_destination_path_wont_overwrite_files() {
        let mut some_file =
            NamedTempFile::new().expect("Should have been able to create a temp file");
        let _write_result = writeln!(some_file, "nonsense");

        let output = process_destination_path(Some(PathBuf::from(some_file.path())));

        assert!(output.is_err());
    }

    #[test]
    #[serial]
    fn test_process_destination_works_with_none() {
        let output = process_destination_path(None).unwrap();

        let default_path = Path::new(DEFAULT_SSH_VAR_YAML_FILE);

        if default_path.exists() {
            let _ = fs::remove_file(default_path);
        }

        assert_eq!(default_path, output);
    }

    #[test]
    #[serial]
    fn test_process_dest_wont_overwrite_default_path() {
        let default_path = Path::new(DEFAULT_SSH_VAR_YAML_FILE);

        if !default_path.exists() {
            let _temp_file_out = fs::write(default_path, String::new())
                .expect("Cannot write a temproary file in the temp directory");
        }

        let output = process_destination_path(None);

        let _ = fs::remove_file(default_path).expect("Cannot delete temp file");
        assert!(output.is_err());
    }

    #[test]
    #[serial]
    fn test_write_vault_pass_does_not_overwrite() {
        let vault_file_path = PathBuf::from(
            home::home_dir()
                .unwrap()
                .as_path()
                .join(VAULT_PASS_FILENAME),
        );

        if !&vault_file_path.exists() {
            let _ = fs::write(&vault_file_path, String::new());
        }

        let output = write_vault_password_file("some_pass");

        assert!(output.is_err());

        let _ = fs::remove_file(&vault_file_path);
    }

    #[test]
    fn test_ansible_vault_encrypts_file() -> std::io::Result<()> {
        let password = "fancy_password";
        let contents = "file_contents";

        let mut temp_contents_file = NamedTempFile::new()?;
        let mut password_file = NamedTempFile::new()?;

        // Write contents and password
        writeln!(temp_contents_file, "{}", contents)?;
        writeln!(password_file, "{}", password)?;

        // Encrypt file
        let _out = vault_ssh_vars_file(
            AnsibleVaultCommand::Encrypt,
            temp_contents_file.path(),
            password_file.path(),
        );

        let post_encrypt_file_contents = fs::read_to_string(temp_contents_file.path())?;

        assert_ne!(post_encrypt_file_contents, contents);
        Ok(())
    }

    #[test]
    fn test_ansible_vault_can_decrypt_file() -> std::io::Result<()> {
        let password = "fancy_password";
        let contents = "file_contents";

        let mut temp_contents_file = NamedTempFile::new()?;
        let mut password_file = NamedTempFile::new()?;

        // Write contents and password
        writeln!(temp_contents_file, "{}", contents)?;
        writeln!(password_file, "{}", password)?;

        // Encrypt file
        let _out = vault_ssh_vars_file(
            AnsibleVaultCommand::Encrypt,
            temp_contents_file.path(),
            password_file.path(),
        );

        // Encrypt file
        let _out = vault_ssh_vars_file(
            AnsibleVaultCommand::Decrypt,
            temp_contents_file.path(),
            password_file.path(),
        );

        let post_encrypt_file_contents = fs::read_to_string(temp_contents_file.path())?;

        let filtered_file_contents = post_encrypt_file_contents.lines().next().unwrap();

        assert_eq!(filtered_file_contents, contents);

        Ok(())
    }
}
