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

  pushd $MODULE_DIR
  RUST_LOG=indy::=debug mvn clean test
  popd
}

set -eux

test wrappers/java