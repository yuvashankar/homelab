use clap::Parser;
use log::{error, info};
use std::path::PathBuf;

mod ansible;
mod ssh_key;

#[derive(Debug, Parser)]
struct Config {
    /// Any comments that will be added to the ssh key
    #[arg(short, long, default_value_t = String::from("ansible_user"))]
    comment: String,
    /// Where the ssh vars will be stored and encrypted
    #[arg(short, long, default_value_os_t = PathBuf::from(ansible::DEFAULT_SSH_VAR_YAML_FILE))]
    destination_path: PathBuf,
    /// The absolute path to the vault password, if none is provided we will prompt for a password.
    #[arg(short, long, default_value_os_t = ansible::generate_default_vault_password_file())]
    vault_pass_file: PathBuf,
    /// The file name that the key will be stored to
    #[arg(short, long, default_value_os_t = ssh_key::default_ssh_key_file_name())]
    filename: PathBuf,
}

fn main() -> Result<(), std::process::ExitCode> {
    env_logger::init();
    let user_input = Config::parse();

    let vault_password = ansible::get_password(user_input.vault_pass_file).map_err(|e| {
        error!("Cannot get vault password from file or by input: {:?}", e);
        std::process::ExitCode::FAILURE
    })?;

    let vault_pass_file =
        ansible::write_vault_password_file(vault_password.as_str()).map_err(|e| {
            error!("Cannot write ansible vault password file: {:?}", e);
            std::process::ExitCode::FAILURE
        })?;

    if user_input.destination_path.exists() {
        info!("ssh_vars.yaml detected, will decrypt and store keys in ~/.ssh/");
        ansible::vault_ssh_vars_file(
            ansible::AnsibleVaultCommand::Decrypt,
            user_input.destination_path.as_path(),
            vault_pass_file.as_path(),
        )
        .map_err(|e| {
            error!("Cannot run ansible-vault command: {:?}", e);
            std::process::ExitCode::FAILURE
        })?;
        // Don't error out until we have re-vaulted the vars file
        let store_ssh_keys_output = ssh_key::store_ssh_key(
            user_input.destination_path.as_path(),
            user_input.filename.as_path(),
        )
        .map_err(|e| {
            error!("Cannot store ssh keys! {}", e);
            std::process::ExitCode::FAILURE
        });

        // Re-vault the yaml file
        ansible::vault_ssh_vars_file(
            ansible::AnsibleVaultCommand::Encrypt,
            user_input.destination_path.as_path(),
            vault_pass_file.as_path(),
        )
        .map_err(|e| {
            error!("Cannot run ansible-vault command: {:?}", e);
            std::process::ExitCode::FAILURE
        })?;

        match store_ssh_keys_output {
            Ok(_ok) => {
                info!("Successfully decrypted and stored ssh keys");
                Ok(())
            }
            Err(e) => {
                error!("Could not decrpt ssh_vars.yaml:");
                Err(e)
            }
        }?
    } else {
        ssh_key::create_ssh_key(&user_input.filename, user_input.comment.as_str()).map_err(
            |e| {
                error!("Cannot run ssh-keyge command: {:?}", e);
                std::process::ExitCode::FAILURE
            },
        )?;

        ansible::generate_ssh_yaml_file(
            user_input.filename.as_path(),
            user_input.destination_path.as_path(),
        )
        .map_err(|e| {
            error!("Cannot generate ssh_vars.yaml file: {:?}", e);
            std::process::ExitCode::FAILURE
        })?;

        ansible::vault_ssh_vars_file(
            ansible::AnsibleVaultCommand::Encrypt,
            &user_input.destination_path,
            &vault_pass_file,
        )
        .map_err(|e| {
            error!("Cannot run ansible-vault command: {:?}", e);
            std::process::ExitCode::FAILURE
        })?;
    }
    Ok(())
}
