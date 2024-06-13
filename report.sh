#!/bin/bash
(zcat /var/log/nginx/access.log*.gz ; cat /var/log/nginx/access.log*) | goaccess -o ./report.html --log-format=COMBINED
