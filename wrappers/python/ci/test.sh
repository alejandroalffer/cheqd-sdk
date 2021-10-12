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
  INDY_PATH=$(pwd)/libindy/target/${BUILD_TYPE}/
  LIBNULLPAY_PATH=$(pwd)/libnullpay/target/${BUILD_TYPE}/
  CLI_PATH=$(pwd)/cli/target/${BUILD_TYPE}/
  export LD_LIBRARY_PATH=${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}:$INDY_PATH:LIBNULLPAY_PATH:CLI_PATH
  pip3 install pytest==3.6.4 base58 pytest-asyncio==0.10.0
  python3 -m pytest wrappers/python/
}

set -eux

test