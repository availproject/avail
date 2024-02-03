#!/bin/bash
export ENGINE="${ENGINE:-docker}"
export ARCH="${ARCH:-x64}"
export ZIP="true"

for distro in ubuntu-2004 ubuntu-2204 ubuntu-2304 ubuntu-2310 fedora-38 fedora-39 debian-11 debian-12 arch; do
    DISTRO=${distro} ./scripts/binaries/build.sh
done
