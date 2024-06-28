#!/bin/bash
# goaccess <(cat /var/log/nginx/access.log /var/log/nginx/access.log.1) <(zcat /var/log/nginx/access.log.*.gz) -o report.html --log-format=COMBINED
goaccess /var/log/nginx/access.log /var/log/nginx/access.log.1 -o report.html --log-format=COMBINED
