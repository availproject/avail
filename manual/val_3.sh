cat script.sh
seed="0x98b6023ac02059071606cfe613019baa116ce61b6cf0e070ac7170b77f9ca47e"  # SR Controller Secret seed
srkey="0xa0c024c8e0ac2a6156e97e73e7dbd0398608997cbe52eee7f7dfde74b6996836" # SR Controller Public key (hex)
edkey="0xfd27369461d8bcb76569b3f33d4f60c56075b894c80dcd7cff53cd425d991bb0" # ED Controller Public key (hex)

curl --trace-ascii here.txt http://localhost:8546 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"gran\",\"$seed\",\"$edkey\"]}"
curl --trace-ascii here.txt http://localhost:8546 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"babe\",\"$seed\",\"$srkey\"]}"
curl --trace-ascii here.txt http://localhost:8546 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"imon\",\"$seed\",\"$srkey\"]}"
curl --trace-ascii here.txt http://localhost:8546 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"audi\",\"$seed\",\"$srkey\"]}"
