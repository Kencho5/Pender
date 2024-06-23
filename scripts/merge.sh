#!/bin/bash
git checkout main
git merge staging
git p
git checkout staging

. ./scripts/sync_aws.sh staging
. ./scripts/sync_aws.sh prod
