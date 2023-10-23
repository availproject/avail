#!/bin/bash

# Finding no of active avail services already present on server to cover edge case of env vars being modified between workflow runs. 
cd /etc/systemd/system
TOTAL_COUNT=$(ls -dq *avail* | wc -l)

for (( i=1; i<=$TOTAL_COUNT; i++ ))
do 
    sudo systemctl stop avail-val-${i}.service
    sudo systemctl stop avail-full-${i}.service
    sudo systemctl stop avail-light-${i}.service
    sudo systemctl disable avail-light-${i}.service
    sudo systemctl disable avail-full-${i}.service
    sudo systemctl disable avail-val-${i}.service
    sudo rm /etc/systemd/system/avail-val-${i}.service
    sudo rm /etc/systemd/system/avail-full-${i}.service
    sudo rm /etc/systemd/system/avail-light-${i}.service
done

rm -rf $HOME/avail-home

rm -rf $HOME/avail-keys

rm -rf $HOME/avail-apps

rm -rf $HOME/build

rm -rf $HOME/data-avail

rm -rf $HOME/light-bootstrap

rm $HOME/avail-test/load-test-config.yaml

rm $HOME/endpoints.txt

sudo rm -rf /var/www/html/*

sudo systemctl daemon-reload