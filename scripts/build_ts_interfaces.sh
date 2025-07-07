mkdir ts-interfaces
current_id=$(pwd)
echo "$current_id"
TS_RS_EXPORT_DIR="$current_id/ts-interfaces" cargo test --features ts