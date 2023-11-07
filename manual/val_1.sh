cat script.sh
seed="0x14522d37b579e02ae87b93899bc5f431515e2a09225b0148b6fdbff3e6caef2b"  # SR Controller Secret seed
srkey="0x7e9ad86435c61fd6f13d4e61b13fc418527c8834ff68b92e82b30768ee760036" # SR Controller Public key (hex)
edkey="0xb21ee1237b7158429be1292630f7759a2cc33b5000aea1bd682edd7ceb96908b" # ED Controller Public key (hex)

curl --trace-ascii here.txt http://localhost:8546 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"gran\",\"$seed\",\"$edkey\"]}"
curl --trace-ascii here.txt http://localhost:8546 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"babe\",\"$seed\",\"$srkey\"]}"
curl --trace-ascii here.txt http://localhost:8546 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"imon\",\"$seed\",\"$srkey\"]}"
curl --trace-ascii here.txt http://localhost:8546 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"audi\",\"$seed\",\"$srkey\"]}"
