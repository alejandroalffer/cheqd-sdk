#!/bin/bash

if [ $# -ne 1 ]
  then
    echo "ERROR: Incorrect number of arguments"
    echo "Usage:"
    echo "$0 <test_pool_ip>"
    exit 1
fi

export TEST_POOL_IP=$1

function test() {
  MODULE_DIR=$1
  export INDY_PATH=$(pwd)/libindy/target/${BUILD_TYPE}/
  export LIBNULLPAY_PATH=$(pwd)/libnullpay/target/${BUILD_TYPE}/
  export CLI_PATH=$(pwd)/cli/target/${BUILD_TYPE}/
  export LD_LIBRARY_PATH=${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}:$INDY_PATH:$LIBNULLPAY_PATH:$CLI_PATH
  pushd $MODULE_DIR
  npm run prepare
  npm install
  npm test --fail-fast=false
  popd
}

set -eux

test wrappers/nodejs