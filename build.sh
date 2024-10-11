#!/usr/bin/sh

echo "*** Compiling rust code"
cargo build

echo "*** Compiling and running go code"
go run go/main.go