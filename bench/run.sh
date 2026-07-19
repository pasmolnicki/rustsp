#!/bin/sh

SCRIPT_DIR="$(cd "$(dirname $0)" && pwd)"
BASELINE="${SCRIPT_DIR}/baseline/rustsp"
TARGET="${SCRIPT_DIR}/../target/release/rustsp"
HYPERFINE_RUN_ARGS="--warmup=3"

if [ ! -e "${TARGET}" ]; then
  echo "Couldn't find the target executable: ${TARGET}"
  exit 1
fi

if [ ! -e "${BASELINE}" ]; then
  echo "No baseline executable found: ${BASELINE}"
  exit 1
fi

# Using hyperfine for benchmarking the code
for f in $(find ${SCRIPT_DIR}/data/*); do
  echo "File: $(basename ${f})"
  hyperfine "${HYPERFINE_RUN_ARGS}" "${BASELINE} ${f}" "${TARGET} ${f}"
done
