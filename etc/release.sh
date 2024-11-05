#!/bin/sh

VERSION=$1
NOW=$(date +%s)
HOME=/var/lib/supercell

cp ${HOME}/production.env ${HOME}/backups/${NOW}-production.env

sqlite3 ${HOME}/database.db ".backup '${HOME}/backups/${NOW}-database.db'"

chown supercell:supercell ${HOME}/backups/*

systemctl restart supercell

