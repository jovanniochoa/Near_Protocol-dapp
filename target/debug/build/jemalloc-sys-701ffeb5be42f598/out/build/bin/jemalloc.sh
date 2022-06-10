#!/bin/sh

prefix=/Users/jovanni/Documents/GitHub/Near_Protocol-dapp/target/debug/build/jemalloc-sys-701ffeb5be42f598/out
exec_prefix=/Users/jovanni/Documents/GitHub/Near_Protocol-dapp/target/debug/build/jemalloc-sys-701ffeb5be42f598/out
libdir=${exec_prefix}/lib

DYLD_INSERT_LIBRARIES=${libdir}/libjemalloc.2.dylib
export DYLD_INSERT_LIBRARIES
exec "$@"
