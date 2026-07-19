#!/usr/bin/sh

# /bench directory
SCRIPT_DIR="$(cd "$(dirname $0)" && pwd)"

# Updates the baseline executable based on the current /target/release/rustsp
TARGET_BINARY_FILE="${SCRIPT_DIR}/../target/release/rustsp"

if [ ! -e "${TARGET_BINARY_FILE}" ]; then
  echo "Couldn't the target executable: ${TARGET_BINARY_FILE}"
  exit 1
fi

# Print command
set -o xtrace
if [ ! -e "${SCRIPT_DIR}/baseline/" ]; then
  mkdir "${SCRIPT_DIR}/baseline"
fi

cp "${TARGET_BINARY_FILE}" "${SCRIPT_DIR}/baseline/"
