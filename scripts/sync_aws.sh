#!/bin/bash
aws s3 sync public/assets s3://pender-assets/assets/ --cache-control "max-age=3600"
aws s3 sync public/static/css s3://pender-assets/css/ --cache-control "max-age=3600"
aws s3 sync public/static/js s3://pender-assets/js/ --cache-control "max-age=3600"

SSH_USER="$SSH_USERNAME"
SSH_HOST="$SSH_HOSTNAME"
SSH_PASSWORD="$SSH_PASSWORD"

# SSH into the server and restart the service
sshpass -p "$SSH_PASSWORD" ssh ${SSH_USER}@${SSH_HOST} << EOF
    sudo systemctl restart pender-staging
    sudo systemctl restart pender-prod
EOF

