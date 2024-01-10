#! /bin/sh

cargo test
cbindgen --lang C --output gorse.h
cp -v target/debug/deps/libgorse.* .
./gorse.py -v
LD_LIBRARY_PATH=. go test
LD_LIBRARY_PATH=. go build
LD_LIBRARY_PATH=. ./gorse
