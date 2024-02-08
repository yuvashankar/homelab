General Config
========
A small role that ensures the right users are set and that the right packages are available. 

Requirements
------------
None

Role Variables
--------------
```
github_user: "yuvashankar"
```
A github username that will be used to download public ssh keys. 
```
automation_user: "yuvashankar"
```
The username that the automation will be run with

```
automation_group: "sudo"
```
The group that the automation user will need to be in. 


Dependencies
------------
None

Example Playbook
----------------
    - hosts: all
      roles:
         - general_config
            vars:
                github_user: "yuvashankar"

License
-------

BSD

Author Information
------------------

Vinay Yuvashankar
