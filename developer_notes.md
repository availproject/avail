# Table of Contents

Document for developer notes that do not belong to README.md

1. [Distro Cross-Compiling](#distro-cross-compiling)
1. [Manual Session Keys Insertion](#manual-session-keys-insertion)

## Distro Cross-Compiling

We provide a simple script that allows us to cross-compile our app to different
linux distros. This is only supported if you are on x64 platform. Right now
cross-compiling from x64 to arm and vice versa is not supported.

Example:
```bash
ENGINE=podman ZIP="yes" DISTRO="ubuntu-2204" ./scripts/binaries/build.sh
# Supported engines: podman docker
# Supported distros: ubuntu-2204 ubuntu-2404 ubuntu-2606 fedora 44 debian 13
# ZIP="yes" additionally zips the final binary
```

## Manual Session Keys Insertion
There is the new way to manually insert session keys. For more information contact our dev-ops team.

```
srseed="0x9a97a54175e8fdb81f411e19be9c267b2bdc80ab3af0bf2b3e0762eb55ab9e66"  # SR Secret seed
edseed="0x9a97a54175e8fdb81f411e19be9c267b2bdc80ab3af0bf2b3e0762eb55ab9e66"  # ED Secret seed
edkey="0x9a97a54175e8fdb81f411e19be9c267b2bdc80ab3af0bf2b3e0762eb55ab9e66" # ED Public key (hex)
srkey="0x9a97a54175e8fdb81f411e19be9c267b2bdc80ab3af0bf2b3e0762eb55ab9e66" # SR Public key (hex)

curl --trace-ascii here.txt http://localhost:9944/ -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"gran\",\"$edseed\",\"$edkey\"]}"
curl --trace-ascii here.txt http://localhost:9944/ -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"babe\",\"$srseed\",\"$srkey\"]}"
curl --trace-ascii here.txt http://localhost:9944/ -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"imon\",\"$srseed\",\"$srkey\"]}"
curl --trace-ascii here.txt http://localhost:9944/ -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"audi\",\"$srseed\",\"$srkey\"]}"
```

