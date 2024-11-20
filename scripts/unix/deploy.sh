#!/bin/sh

# Check if the argument starts with "--ssh-key="
if [[ $1 == --ssh-key=* ]]; then
  SSH_KEY="${1#--ssh-key=}"

  echo "Hello, the ssh key is \"$SSH_KEY\""
else
  # If the argument doesn't match, print an error message
  echo "Usage: $0 --ssh-key=<value>"
  exit 1
fi
