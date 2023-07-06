# homelab

To run ansible playbook
```
cd ansible/
ANSIBLE_CONFIG=$(pwd)/ansible.cfg ansible-playbook -i -k inventory/default_pfSense playbooks/test.yaml
```