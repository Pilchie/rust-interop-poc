#!/usr/bin/sh

echo "*** Setup python venv"
python3 -m venv .venv
. .venv/bin/activate
pip install -U pip maturin
pip freeze

echo "*** Compiling rust code"
cargo build

echo "*** Running rust tests"
cargo test

echo "*** Compiling python module"
cd rust/azure_data_cosmos_shared
maturin develop
cd ../..

echo "*** Compiling node module"
cd rust/azure_data_cosmos_shared
npm run build
cd ../..

echo "*** Compiling and running go code"
go run go/main.go

echo "*** Running python consumer"
python3 python/main.py

echo "*** Running node consumer"
node node/main.js
