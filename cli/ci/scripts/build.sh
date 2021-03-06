#!/bin/bash

if [ $# -ne 1 ]
  then
    echo "ERROR: Incorrect number of arguments"
    echo "Usage:"
    echo "$0 <debug|release>"
    exit 1
fi

BUILD_TYPE=$1

if [ $BUILD_TYPE == 'release' ]
  then
    CARGO_FLAGS='--release'
  else
    CARGO_FLAGS=''
fi

set -eux

cp libindy/target/${BUILD_TYPE}/libindy.so cli
cp libnullpay/target/${BUILD_TYPE}/libnullpay.so cli

pushd cli
LIBRARY_PATH=./ RUST_BACKTRACE=1 cargo build ${CARGO_FLAGS} --features fatal_warnings
popd
