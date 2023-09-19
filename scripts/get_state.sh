#!/bin/bash

# Check if python is installed
if ! [[ "$(python3 -V)" =~ "Python 3" ]]; then
    echo "python3 is not installed, please install it."
    exit 1
fi

if ! [ -z "$SKIP_DEP" ]; then
    # Install dependenices
    python3 -m venv ./scripts/penv
    source ./scripts/penv/bin/activate
    echo "Installing Python dependencies"
    pip install -r ./scripts/requirements.txt > /dev/null 2>&1
fi

CONFIG="${CONFIG:-./scripts/networks/biryani.yaml}"
python3 ./scripts/get_state.py --config "${CONFIG}"

cp ./target/release/data-avail ./output/binary