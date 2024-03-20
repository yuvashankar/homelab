# Running Ansible

## Create a vault password file
The intended workflow is to use the devcontainer features on VSCode, but this can run natively as well assuming it's an debian based system. We first need to create ssh keys. I've created a helper CLI tool to create and encrypt the ssh keys first, it should be installed by default in the dev container. First we need to create a vault password to encrypt the ssh keys in `~/.vault_pass.txt`. Use your favourite text editor to add the file in the devcontainer. Alternatively run the following command, inside the devcontainer container. 

```bash
$ echo <your password> | tee -a ~/.vault_pass.txt
```
The vault password is important, losing this password will make it impossible to decrypt your ssh keys and lock you out of any system that you run this against. 

## Initialize ssh keys
Once the vault password is avaliable, run `initialize_ssh` to create and vault a new ssh key, or it will encrypt and store any vault password any ssh keys that exist in the dev container. 

```bash
$ initialize_ssh
```

## Create Encrypted Data
There are some variables in the [vars.yaml](vars/main.yml) file that users must provide for themselves. These values must be encrypted prior to being commited to any any repository. To encrypt a variable run the following command and copy it into the vars file. 
```bash
 ansible-vault encrypt_string --vault-password-file ~/.vault_pass.txt "<string_to_encrypt>" --name "<name_of_variable_to_set>"
 # Example
 ansible-vault encrypt_string --vault-password-file ~/.vault_pass.txt "1234567890123456" --name "samba_password"
```

## Copying SSH ID to the server to automate
In order to be able to run the automation against the server, we must first copy the ssh id to the server that we are automating against. 
```bash
ssh-copy-id -i ~/.ssh/id_ed25519.pub username@host-ip-address
# Example
ssh-copy-id -i ~/.ssh/id_ed25519.pub yuvashankar@192.168.2.2
```

## Run the ansible playbook
```bash
cd ansible/
ANSIBLE_CONFIG=$(pwd)/ansible.cfg ansible-playbook -i inventory/hosts.ini playbooks/main.yaml -K # No getting around the fact that the first time we have to supply the sudo password hence the -K
```

# Development Comments
The intent of this project was to create a generic method of securing and deploying a homelab running microservices. But that generally fights against the ethos of `homelabbing`. The idea is to hack together things that you have lying around into something it wasn't designed for. Every deployment is unique, and that's why this repo is such a hack job. That's what I'll keep telling myself. 


This is what I have created with the equipment and needs that I have at my home, you may not have the same hardware, but the general idea shoudl be the same. Go ahead and steal any items that you need from this repository, I certinaly didn't develop this in a vacume. 


## Molecule Unit Testing
I initally used molecule as an attempt to unit test the ansible workflow. This didn't play out as it did in my head as there is a large interaction with `systemd` in the ansible workflow, which molecule's docker system doesn't really support. Maybe someone with more paitence contending with the molecule documentation will be able to explain how to run unit tests with a VM or something. 

Until then, molecule unit testing may or may not work, 

## Initializing a role
I always forget how to initialize a role with ansible
```bash
# Initialize a role
ansible-galaxy init role-name

# Initialize a molecule role
molecule init scenario
```

