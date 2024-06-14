#!/bin/bash
rm -r /var/log/nginx/access.log.*.gz
goaccess /var/log/nginx/access.log* -o report.html --log-format=COMBINED --load-from-disk
