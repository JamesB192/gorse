#! /bin/sh

cargo test
cbindgen --lang C --output gorse.h
cp -v target/debug/deps/libgorse.so .
./gorse.py -v
go test
go build
