#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# This script builds all subprojects and puts all created Wasm modules in one dir
(
  cargo update --aggressive
  marine build --release
  cargo +nightly test -- --test-threads=1 --release 
)
