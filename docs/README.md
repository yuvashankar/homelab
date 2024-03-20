# Table of Contents
1. [Network Overview](Network.md)

# Deployment Instructions
## Configure the OPNSense Firewall
Refer to [opnsense installation](../config/opnsense/README.md) instructions to install a OPNSense config that mirrors the network described above.

## Configure Networking
Configure the Unifi networking, following the most popular guide available for the latest version of the unifi-controller. At the time of wriring [this](https://homenetworkguy.com/how-to/configure-vlan-per-ssid-unifi-access-point/) seems to serve its purpouse.

In order to continue we need to ensure that we are able to have a management and Wi-Fi network, those are the two important items.

## Configure the storage service
If using NAS storage service the following items need to be configured
* Ensure that the NAS has Samba share enabled
* Create a samba share user with a password login
* Create the necessary Samba Share folders. Refer to the folders_to_mount variable in the [vars.yaml](../ansible/vars/main.yml). 

## Stand-up the server
1. Download debian-server somehow
2. Run through the installation process, 
   * ensure the `automation_user` exists on the machine, from [vars.yaml](../ansible/vars/main.yml)
   * Ensure that the server has ethernet connected, and is able to communicate with the outside world
3. Follow the [Ansible instructions](../ansible/README.md)