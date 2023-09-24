use clap::Parser;
use log::error;
use std::path::PathBuf;

mod ansible;
mod ssh_key;

#[derive(Debug, Parser)]
struct Config {
    /// Any comments that will be added to the ssh key
    #[arg()]
    comment: String,
    /// The absolute path to the vault password, if none is provided we will prompt for a password.
    #[arg(short, long)]
    destination_path: Option<PathBuf>,
    #[arg(short, long)]
    vault_pass_file: Option<PathBuf>,
    /// The file name that the key will be stored to
    #[arg(short, long)]
    filename: Option<PathBuf>,
}

fn main() -> Result<(), std::process::ExitCode> {
    env_logger::init();
    let user_input = Config::parse();

    let ssh_config = ssh_key::SshKeyGenConfig {
        filename: ssh_key::generate_key_file_name(user_input.filename),
        comment: user_input.comment,
    };

    let yaml_dest_path =
        ansible::process_destination_path(user_input.destination_path).map_err(|e| {
            error!("Cannot get the ansible vars destination path \n{:?}", e);
            return std::process::ExitCode::FAILURE;
        })?;

    let _output =
        ansible::generate_ssh_yaml_file(&ssh_config.filename.as_path(), yaml_dest_path.as_path())
            .map_err(|e| {
            error!("Cannot generate ssh_vars.yaml file: {:?}", e);
            return std::process::ExitCode::FAILURE;
        })?;

    let vault_password = ansible::get_password(user_input.vault_pass_file).map_err(|e| {
        error!("Cannot get vault password from file or by input: {:?}", e);
        return std::process::ExitCode::FAILURE;
    })?;

    let vault_pass_file =
        ansible::write_vault_password_file(vault_password.as_str()).map_err(|e| {
            error!("Cannot write ansible vault password file: {:?}", e);
            return std::process::ExitCode::FAILURE;
        })?;

    let _vault_cmd_output = ansible::vault_ssh_vars_file(
        ansible::AnsibleVaultCommand::Encrypt,
        &yaml_dest_path,
        vault_pass_file.as_path(),
    )
    .map_err(|e| {
        error! {"Cannot encrypt ansible ssh vars file: {:?}", e};
        return std::process::ExitCode::FAILURE;
    })?;

    // let path_to_ssh_key = ssh_key::create_ssh_key(ssh_config);

    Ok(())
}
