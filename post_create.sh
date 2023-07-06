#!/bin/bash
sudo apt-get update && sudo apt-get install sshpass
ansible-galaxy collection install pfsensible.core