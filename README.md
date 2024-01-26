# Homelab Configuration

The intended workflow is to use the devcontainer features on VSCode, but this can run natively as well assuming it's an debian based system. We first need to create ssh keys. I've created a helper CLI tool to create and encrypt the ssh keys first, it should be installed by default in the dev container. First we need to create a vault password to encrypt the ssh keys in `~/.vault_pass.txt`. Use your favourite text editor to add the file in the devcontainer. Alternatively run the following command, inside the devcontainer container

```bash
$ echo <your password> | tee -a ~/.vault_pass.txt
```

The vault password is important, losing this password will make it impossible to decrypt your ssh keys and bork any system that you run this against. Once the vault password is avaliable, run `initialize_ssh` to create and vault a new ssh key. 

```bash
$ initialize_ssh
```

To run ansible playbook
```bash
cd ansible/
ANSIBLE_CONFIG=$(pwd)/ansible.cfg ansible-playbook -i inventory/hosts.ini playbooks/test.yaml -K # No getting around the fact that the first time we have to supply the sudo password
```

# Role Commands

```bash
# Initialize a role
ansible-galaxy init role-name

# Initialize a molecule role
molecule init scenario
```

# Encrypt a string with ansible
```bash
 ansible-vault encrypt_string --vault-password-file ~/.vault_pass.txt "<string_to_encrypt>" --name "<name_of_variable_to_set>"
```