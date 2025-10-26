#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../.." && pwd)"
CAPSTONE_PATH_INPUT="${1:-third_party/capstone}"
if [[ "${CAPSTONE_PATH_INPUT}" != /* ]]; then
  CAPSTONE_PATH="${ROOT_DIR}/${CAPSTONE_PATH_INPUT}"
else
  CAPSTONE_PATH="${CAPSTONE_PATH_INPUT}"
fi

if [[ ! -d "${CAPSTONE_PATH}" ]]; then
  echo "Capstone directory not found: ${CAPSTONE_PATH}" >&2
  exit 1
fi

CORE_LIB="${CAPSTONE_PATH}/libcapstone.a"

if [[ ! -f "${CORE_LIB}" ]]; then
  echo "Building Capstone core in ${CAPSTONE_PATH}..."
  make -C "${CAPSTONE_PATH}"
else
  echo "Capstone core already built at ${CORE_LIB}."
fi

CSTOOL_BINARY="${CAPSTONE_PATH}/cstool/cstool"
if [[ ! -x "${CSTOOL_BINARY}" ]]; then
  echo "Building cstool tool..."
  make -C "${CAPSTONE_PATH}/cstool"
else
  echo "cstool already built at ${CSTOOL_BINARY}."
fi
