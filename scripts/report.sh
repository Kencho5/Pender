#!/bin/bash
rm /var/log/access.log.6.gz
goaccess <(cat /var/log/nginx/access.log /var/log/nginx/access.log.1) <(zcat /var/log/nginx/access.log.*.gz) -o report.html --log-format=COMBINED
