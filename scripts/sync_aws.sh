#!/bin/bash

# Check if an argument is provided
if [ $# -eq 0 ]; then
    echo "Usage: $0 <environment>"
    exit 1
fi

# Store the argument in a variable
ENVIRONMENT=$1

aws s3 sync public/assets s3://pender-assets/$ENVIRONMENT/assets/ --cache-control "max-age=31536000"
aws s3 sync public/static/css s3://pender-assets/$ENVIRONMENT/static/css/ --cache-control "max-age=31536000"
aws s3 sync public/static/js s3://pender-assets/$ENVIRONMENT/static/js/ --cache-control "max-age=31536000"

SSH_USER="$SSH_USERNAME"
SSH_HOST="$SSH_HOSTNAME"
SSH_PASSWORD="$SSH_PASSWORD"

# SSH into the server and restart the service
sshpass -p "$SSH_PASSWORD" ssh ${SSH_USER}@${SSH_HOST} << EOF
    sudo systemctl restart pender-$ENVIRONMENT
EOF
