#! /bin/sh

cargo test
cbindgen --lang C --output gorse.h
go test
