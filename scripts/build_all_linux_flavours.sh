#!/bin/bash
export ENGINE="${ENGINE:-docker}"

mkdir -p ./output/zips/

for flavor in ubuntu-2004 ubuntu-2204 ubuntu-2304 ubuntu-2310 fedora-38 fedora-39 debian-11 debian-12 arch; do
    FLAVOUR=${flavor} ./scripts/build_linux_flavours.sh
    cd ./output/"$flavor"
    tar -czf ./../../output/zips/x86_64-"$flavor"-data-avail.tar.gz data-avail
    cd ./../../
done
