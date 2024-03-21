Configure Networking
=========
Configures the ethernet network devices for a debian based system. The molecule module for this will probably not work as it deals with network interfaces which docker containers do not have. 


Requirements
------------
None

Role Variables
--------------
```
wifi_network_block: "192.168.4.0/24"
```
The default route for the wifi networking block that we need to setup a IP route table entry for

```
network_interface:
  eth: "eno1"
  static_address: "192.168.2.2"
  netmask: "255.255.255.0"
  network: "192.168.2.0"
  broadcast: "192.168.2.255"
  gateway: "192.168.2.1"
```
The static IP settings for the primary ethernet interface. 

Dependencies
------------
None

Example Playbook
----------------

Including an example of how to use your role (for instance, with variables passed in as parameters) is always nice for users too:

    - hosts: all
      roles:
         - configure_networking

License
-------

BSD

Author Information
------------------

Vinay Yuvashankar
