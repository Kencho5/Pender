#!/bin/bash
goaccess /var/log/nginx/access.log* -o report.html --log-format=COMBINED --load-from-disk
