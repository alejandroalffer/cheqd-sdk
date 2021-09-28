#!/bin/bash

if [ $# -ne 1 ]
  then
    echo "ERROR: Incorrect number of arguments"
    echo "Usage:"
    echo "$0 <build-num>"
    exit 1
fi

BUILD_NUM=$1

set -eux

ls -a
pwd
ls libindy/
ls /builds/evernym/verity/vdr-tools/libindy/target/debug/

PACKAGE_TYPE=$(lsb_release -cs)
# REVISION=$(git rev-parse HEAD | cut -c 1-7)
VERSION=${CI_COMMIT_TAG:-0.0.1}~${BUILD_NUM}-${PACKAGE_TYPE}  # TODO: Autodetect main part
pushd libindy
cargo deb --no-build --deb-version ${VERSION} --variant vdr-tools-${PACKAGE_TYPE}
popd