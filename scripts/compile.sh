#!/bin/bash

cargo run --bin powr-compiler "const a = 1"
# opt ./compiled.ll
llc -filetype=obj compiled.ll -o compiled.o
clang compiled.o -o compiled
./compiled
