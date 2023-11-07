seed="0x51a1df0bb0d7e44543e2adb87f09a96456d7745b1cd9353be44e792729c91fbc"  # SR Controller Secret seed
srkey="0x246af1a9377e5277572cb9310f056ce053dfbceaf9130cda0b6c7036c7bfbf0a" # SR Controller Public key (hex)
edkey="0xac72b14cf54f2de4bcd6e111d3aabb5320c3ed526cc585a0ed6e1c6fa852e8e6" # ED Controller Public key (hex)

curl --trace-ascii here.txt http://localhost:8546 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"gran\",\"$seed\",\"$edkey\"]}"
curl --trace-ascii here.txt http://localhost:8546 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"babe\",\"$seed\",\"$srkey\"]}"
curl --trace-ascii here.txt http://localhost:8546 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"imon\",\"$seed\",\"$srkey\"]}"
curl --trace-ascii here.txt http://localhost:8546 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"audi\",\"$seed\",\"$srkey\"]}"
