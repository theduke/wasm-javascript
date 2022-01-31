#! /usr/bin/env bash

set -euxo pipefail

wit-bindgen spidermonkey ./code.js \
  --import-spidermonkey \
  --export ./wit/js_api.wit \
  --import ./wit/host_api.wit  \
  --out-dir ./wasm

