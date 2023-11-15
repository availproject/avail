#!/bin/bash
export SYSTEM="${SYSTEM:-docker}"

for flavor in ubuntu-2004 ubuntu-2204 ubuntu-2304 ubuntu-2310 fedora-38 fedora-39 debian-11 debian-12 arch; do
    cd ./output/"$flavor"/
    cp data-avail .amd64-"$flavor"-data-avail
    tar czf ./../amd64-"$flavor"-data-avail.tar.gz amd64-"$flavor"-data-avail
    cd ./../..
done