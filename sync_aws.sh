#!/bin/bash
aws s3 sync public/assets s3://pender-assets/assets/ --cache-control "max-age=31536000"
aws s3 sync public/static/css s3://pender-assets/css/ --cache-control "max-age=31536000"
aws s3 sync public/static/js s3://pender-assets/js/ --cache-control "max-age=31536000"
