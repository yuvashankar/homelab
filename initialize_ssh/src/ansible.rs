use core::fmt;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[cfg(not(test))]
pub const DEFAULT_SSH_VAR_YAML_FILE: &str = "/workspaces/homelab/ansible/vars/ssh_vars.yaml";
#[cfg(not(test))]
const VAULT_PASS_FILENAME: &str = ".vault_pass.txt";

#[cfg(test)]
pub const DEFAULT_SSH_VAR_YAML_FILE: &str = "/tmp/temm_ssh_yaml.yaml";
#[cfg(test)]
const VAULT_PASS_FILENAME: &str = "/tmp/vault_pass_file_test/.test_vault_pass.txt";

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct VarsFile {
    pub ssh_public_key: String,
    pub ssh_private_key: String,
}

pub fn get_password(file_path: PathBuf) -> Result<String, Error> {
    if file_path.exists() {
        let file_contents = fs::read_to_string(file_path)?;
        if let Some(file_password) = file_contents.lines().next() {
            Ok(String::from(file_password))
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "Password should be in ASCII in a single line",
            ))
        }
    } else {
        rpassword::prompt_password("Enter the ansible vault password you would like to use: ")
    }
}

pub fn generate_default_vault_password_file() -> PathBuf {
    match home::home_dir() {
        Some(h_dir) => h_dir.as_path().join(VAULT_PASS_FILENAME),
        None => PathBuf::from(VAULT_PASS_FILENAME),
    }
}

pub fn generate_ssh_yaml_file(
    ssh_key_source: &Path,
    dest_path: &Path,
) -> Result<(), std::io::Error> {
    let private_key =
        fs::read_to_string(ssh_key_source).expect("Error reading ssh key private file");
    let public_key = fs::read_to_string(ssh_key_source.with_extension("pub"))
        .expect("Error reading ssh key public file");

    let file_contents = serde_yaml::to_string(&VarsFile {
        ssh_public_key: public_key,
        ssh_private_key: private_key,
    })
    .map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Unable to parse the public and private keys into a yaml file: {e:?}"),
        )
    })?;
    fs::write(dest_path, file_contents)?;
    Ok(())
}

pub fn write_vault_password_file(vault_password: &str) -> Result<PathBuf, std::io::Error> {
    let save_path = generate_default_vault_password_file();

    if save_path.exists() {
        Ok(save_path)
    } else {
        fs::write(&save_path, vault_password)?;
        Ok(save_path)
    }
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
    var_file_path: &Path,
    vault_pass_file: &Path,
) -> Result<(), std::io::Error> {
    let vault_pass_file = PathBuf::from(vault_pass_file);

    let vault_args = vec!["--vault-pass-file", vault_pass_file.to_str().unwrap()];

    let ansible_command = ansible_vault_command.to_string().trim().to_lowercase();

    Command::new("ansible-vault")
        .arg(ansible_command)
        .arg(var_file_path)
        .args(vault_args)
        .output()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::io::Write;
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn test_get_password_reads_file() {
        let mut some_file =
            NamedTempFile::new().expect("Should have been able to create a temp file");

        let _write_result = writeln!(some_file, "nonsense");

        let output = get_password(PathBuf::from(some_file.path()));

        assert_eq!(output.unwrap(), String::from("nonsense"));
    }

    #[test]
    fn test_empty_file_errors_out() {
        let some_file = NamedTempFile::new().expect("Should have been able to create a temp file");

        let output = get_password(PathBuf::from(some_file.path()));

        assert!(output.is_err());
    }

    #[test]
    fn test_generate_ssh_yaml_file_creates_yaml_file() -> Result<(), std::io::Error> {
        let public_contents = "public_contents";
        let private_contents = "private_contents";

        let tmp_dir = TempDir::new()?;

        let public_path = tmp_dir.path().join("temp_ssh_key").with_extension("pub");
        let private_path = tmp_dir.path().join("temp_ssh_key");

        fs::write(&public_path, public_contents)?;
        fs::write(&private_path, private_contents)?;

        let yaml_dest = NamedTempFile::new()?;

        generate_ssh_yaml_file(&private_path.as_path(), yaml_dest.path())?;

        let yaml_contents = fs::read_to_string(yaml_dest.path())?;

        assert!(yaml_contents.contains(public_contents));
        assert!(yaml_contents.contains(private_contents));

        Ok(())
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
