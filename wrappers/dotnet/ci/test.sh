#!/bin/bash

# -- Test -- #
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
  INDY_PATH=$(pwd)/libindy/target/${BUILD_TYPE}/
  export LD_LIBRARY_PATH=${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}:$INDY_PATH
  pushd $MODULE_DIR
  dotnet restore -p:Configuration=Release
  dotnet build -c Release                   # DotNet SDK build with warning as error
  dotnet test                               # DotNet SDK tests
  popd
}

set -eux

test wrappers/dotnet/indy-sdk-dotnet-test