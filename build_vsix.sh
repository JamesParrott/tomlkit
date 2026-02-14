#!/bin/bash
set -euxo pipefail

# Assumes this file is indeed in the tomlkit's root directory
REPO_ROOT_DIR=$(realpath $(dirname $0))

cd tomlkit-core
wasm-pack build --target web
cd $REPO_ROOT_DIR
npx -y @vscode/vsce package