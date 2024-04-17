#! /bin/sh

cargo test && \
[ -n "$(command -v cbindgen)" ] && \
cbindgen --lang C --output gorse.h && ( \
cp -v target/debug/deps/libgorse.* . && \
./gorse.py -v && \
DYLD_LIBRARY_PATH=. LD_LIBRARY_PATH=. go test && \
DYLD_LIBRARY_PATH=. LD_LIBRARY_PATH=. go run .) || \
echo "Failed to run cbindgen"; exit 1
