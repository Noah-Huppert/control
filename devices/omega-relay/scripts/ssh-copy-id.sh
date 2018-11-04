#!/usr/bin/env bash
#?
# ssh-copy-id.sh - Copies an SSH key to the Omega
#
# USAGE
#
#	./ssh-copy-id.sh
#
# BEHAVIOR
#
#	Copies you SSH public key to the Omega in the /etc/dropbear/authorized_keys
#	file
#?

if [ -z "$ip" ]; then 
	echo "ip environment variable must be set" &> 2
	exit 1
fi

echo "Found public keys:"
echo
ls $HOME/.ssh/*.pub
echo
echo "Name of SSH key to key?"
read "key_name"

key_path="$HOME/.ssh/$key_name.pub"

if [ ! -f "$key_path" ]; then
	echo "Public key file does not exist: $key_path" &> 2
	exit 1
fi

echo "Copying $key_path"

cat "$key_path" | ssh "root@$ip" "cat > /etc/dropbear/authorized_keys"

echo "Setting 600 permissions on authorized keys file"

ssh "root@$ip" "chmod 600 /etc/dropbear/authorized_keys"
