cat script.sh
seed="0xba07f4ab8a4c7c1f999ccc69bd7d0538b1a56a2a5521a66f9e32b697cf8e6c3f"  # SR Controller Secret seed
srkey="0x808c7f462edb17a7aef0ed46944c0511ec25059fc4a21db9dbb5230fdea7a87f" # SR Controller Public key (hex)
edkey="0x5aa081ac79a8070911aaa18e3a59537e88e9fa90f436d8007c2c19ddba5c110a" # ED Controller Public key (hex)

curl --trace-ascii here.txt http://localhost:8546 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"gran\",\"$seed\",\"$edkey\"]}"
curl --trace-ascii here.txt http://localhost:8546 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"babe\",\"$seed\",\"$srkey\"]}"
curl --trace-ascii here.txt http://localhost:8546 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"imon\",\"$seed\",\"$srkey\"]}"
curl --trace-ascii here.txt http://localhost:8546 -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"audi\",\"$seed\",\"$srkey\"]}"
