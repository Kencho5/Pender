#!/bin/bash
rm /var/log/nginx/access.log.7.gz
goaccess <(cat /var/log/nginx/access.log /var/log/nginx/access.log.1) <(zcat /var/log/nginx/access.log.*.gz) -o report.html --log-format=COMBINED
