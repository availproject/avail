cat script.sh
seed="0xbac0399f9f9128b496078fc08b174d29ddb3a917dcf209139ece71c0bae28407"  # SR Controller Secret seed
srkey="0x66100102320acb7cbe901fca73fa814c03302505e112129752834eba2cebe336" # SR Controller Public key (hex)
edkey="0x3f4758ac3ef193e719f8a6d2826a137acbd8a99317015a92d336043b7dcfc6ad" # ED Controller Public key (hex)

curl --trace-ascii here.txt http://localhost:8546 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"gran\",\"$seed\",\"$edkey\"]}"
curl --trace-ascii here.txt http://localhost:8546 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"babe\",\"$seed\",\"$srkey\"]}"
curl --trace-ascii here.txt http://localhost:8546 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"imon\",\"$seed\",\"$srkey\"]}"
curl --trace-ascii here.txt http://localhost:8546 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"audi\",\"$seed\",\"$srkey\"]}"
