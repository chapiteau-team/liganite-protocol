#!/bin/bash

set -e

# Ensure a runtime path is provided
if [ "$#" -ne 2 ]; then
  echo "Usage: $0 <path-to-runtime> <output-dir>"
  exit 1
fi

RUNTIME_PATH="$1"
OUTPUT_DIR="$2"

# Create output directory if it doesn't exist
mkdir -p "$OUTPUT_DIR"

# Get the list of all pallets
ALL_PALLETS=($(frame-omni-bencher v1 benchmark pallet --runtime "$RUNTIME_PATH" --list=pallets))
echo "Found pallets: " "${ALL_PALLETS[@]}"

# Define excluded pallets
EXCLUDED_PALLETS=(
  "pallet"
  "pallet_aura"
  "pallet_babe"
  "pallet_grandpa"
)
echo "Excluded pallets: " "${EXCLUDED_PALLETS[@]}"

# Filter out the excluded pallets by concatenating the arrays and discarding duplicates.
PALLETS=($(comm -23 <(printf '%s\n' "${ALL_PALLETS[@]}" | sort) <(printf '%s\n' "${EXCLUDED_PALLETS[@]}" | sort)))
echo "Benchmarking pallets: ${#PALLETS[@]}."

# Iterate over pallets and execute benchmark command
TOTAL_PALLETS=${#PALLETS[@]}
for i in "${!PALLETS[@]}"; do
  pallet="${PALLETS[$i]}"
  OUTPUT_PATH="$OUTPUT_DIR/${pallet}.rs"
  echo "Benchmarking pallet [$((i+1))/$TOTAL_PALLETS]: $pallet"
  frame-omni-bencher v1 benchmark pallet --runtime "$RUNTIME_PATH" --pallet "$pallet" --extrinsic "*" --output "$OUTPUT_PATH"
done
