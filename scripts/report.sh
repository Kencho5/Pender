#!/bin/bash
goaccess /var/log/nginx/access.log /var/log/nginx/access.log.1 -o ../report.html --log-format=COMBINED
