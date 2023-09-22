#!/bin/bash

STEPS="${STEPS:-50}"
REPEAT="${REPEAT:-20}"
TEMPLATE_PATH="${TEMPLATE_PATH:-./.maintain/frame-weight-template.hbs}"
OUTPUT_PATH="${OUTPUT_PATH:-./output}"
BINARY_LOCATION="./target/release/data-avail"
PALLETS="${PALLETS:-*}"

echo "STEPS: $STEPS, REPEAT: $REPEAT"
echo "PALLETS: $PALLETS"
echo "TEMPLATE_PATH: $TEMPLATE_PATH"
echo "OUTPUT_PATH: $OUTPUT_PATH"
echo "OUR_PALLETS: $OUR_PALLETS"
echo "BINARY_LOCATION: $BINARY_LOCATION"
echo "EXTRA: $EXTRA"

run_benchmark() {
    echo "Pallets: ${PALLETS[@]}"
    
    rm -f $ERR_FILE
    mkdir -p "$OUTPUT_PATH"
    
    for PALLET in "${PALLETS[@]}"; do
        if is_pallet_excluded; then
            echo "[ ] Skipping pallet $PALLET"
            continue
        fi
        
        file_name="$PALLET.rs"
        
        benchmark "$template_arg" "$file_name"
        
        if is_custom_pallet; then
            template_name="${PALLET}_weights.rs"
            benchmark "--template $TEMPLATE_PATH" "$template_name"
        fi
        
    done
}

benchmark() {
    echo "[+] Benchmarking $PALLET"
    
    if ! [ -z "$EXTRA" ]; then
        EXTRA="--extra"
    else
        unset EXTRA
    fi

    OUTPUT=$($BINARY_LOCATION benchmark pallet --chain=dev --steps=$STEPS --repeat=$REPEAT --pallet="$PALLET" $EXTRA --extrinsic="*"  --heap-pages=4096 --header=./HEADER-APACHE2 --log=warn --output "$OUTPUT_PATH/$2" $1 2>&1)
    if [ $? -ne 0 ]; then
        echo "$OUTPUT" >>"$ERR_FILE"
        echo "[-] Failed to benchmark $PALLET. Error written to $ERR_FILE; continuing..."
    fi
}

is_pallet_excluded() {
    for EXCLUDED_PALLET in "${EXCLUDED_PALLETS[@]}"; do
        if [ "$EXCLUDED_PALLET" == "$PALLET" ]; then
            return 0
        fi
    done
    
    return 1
}

is_custom_pallet() {
    for CUSTOM_PALLETS in "${CUSTOM_PALLETS[@]}"; do
        if [ "$CUSTOM_PALLETS" == "$PALLET" ]; then
            return 0
        fi
    done
    
    return 1
}

populate_pallet_list() {
    # Manually exclude some pallets.
    EXCLUDED_PALLETS=(
        # Helper pallets
        "pallet_election_provider_support_benchmarking"
        # Pallets without automatic benchmarking
        "pallet_babe" "pallet_grandpa"
        "pallet_mmr" "pallet_offences"
    )
    
    CUSTOM_PALLETS=()
    for f in ./pallets/*/Cargo.toml; do
        pallet_name=$(awk -F' = ' '$1 == "name" {print $2}' $f | tr -d '"' | tr '-' '_')
        CUSTOM_PALLETS+=($pallet_name)
    done
    
    if ! [ "$PALLETS" = "*" ]; then
        PALLETS=($PALLETS)
    fi
    if [ "$PALLETS" = "*" ]; then
        PALLETS=($($BINARY_LOCATION benchmark pallet --list --chain=dev | tail -n+2 | cut -d',' -f1 | sort | uniq))
    fi
    if ! [ -z "$OUR_PALLETS" ]; then
        PALLETS=("${CUSTOM_PALLETS[@]}")
    fi
}

ERR_FILE="$OUTPUT_PATH/benchmarking_errors.txt"

echo "Building the client in Release mode"
cargo build --release --locked --features=runtime-benchmarks

populate_pallet_list
run_benchmark
