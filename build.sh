#!/bin/bash

# Run the client generation script
./scripts/generate_client.sh

# Now that the client is generated, add it to the workspace
cat <<EOT > Cargo.toml
[workspace]
members = [
    "generated-client"
]
resolver = "2"
EOT

# Proceed with the build
cargo build

