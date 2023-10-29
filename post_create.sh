#!/bin/bash
sudo apt-get update && sudo apt-get install sshpass
cd /workspaces/homelab/ && poetry install
ansible-galaxy install -r /workspaces/homelab/ansible/requirements.yml

# Create and install initialize_ssh
rustup update
cargo install cargo-deb
cd /workspaces/homelab/initialize_ssh/ && cargo-deb --no-strip --install