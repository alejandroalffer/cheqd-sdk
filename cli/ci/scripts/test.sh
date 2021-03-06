#!/bin/bash

if [ $# -ne 2 ]
  then
    echo "ERROR: Incorrect number of arguments"
    echo "Usage:"
    echo "$0 <debug|release> <test-pool-ip>"
    exit 1
fi

BUILD_TYPE=$1
export TEST_POOL_IP=$2

if [ $BUILD_TYPE == 'release' ]
  then
    CARGO_FLAGS='--release'
  else
    CARGO_FLAGS=''
fi

set -eux

pushd cli
LIBRARY_PATH=./ RUST_BACKTRACE=1 cargo test ${CARGO_FLAGS} --features "nullpay_plugin" --no-run
LD_LIBRARY_PATH=./ RUST_BACKTRACE=1 RUST_LOG=indy::=debug RUST_TEST_THREADS=1 cargo test ${CARGO_FLAGS} --features "nullpay_plugin"
popd
