#!/bin/bash
sudo apt-get update && sudo apt-get install sshpass
pip3 install -r /workspaces/homelab/requirements.txt
ansible-galaxy install -r /workspaces/homelab/ansible/requirements.yml