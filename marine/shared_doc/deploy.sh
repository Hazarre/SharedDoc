#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# build wasms
./build.sh

(
  fldist new_service --node-id 12D3KooWEFFCZnar1cUJQ3rMWjvPQg6yMV2aXWs2DkJNSRbduBWn \
         --ms artifacts/shared_doc.wasm:cfg.json \
         --name shared-doc-service
)
