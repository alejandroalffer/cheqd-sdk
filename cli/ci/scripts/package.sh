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

PACKAGE_TYPE=$(lsb_release -cs)
# REVISION=$(git rev-parse HEAD | cut -c 1-7)
VERSION=1.95.0~${BUILD_NUM}-${PACKAGE_TYPE}  # TODO: Autodetect main part

pushd cli
cp Cargo.toml Cargo.toml.backup
sed -i -E -e "s/depends = \"libindy \(= [(,),0-9,.]+\),/depends = \"libindy-async \(= ${VERSION}\),/g" Cargo.toml
sed -i -E -e "/provides = \"indy-cli \(= [(,),0-9,.]+\)\"/a conflicts = \"indy-cli\"" Cargo.toml
sed -i -E -e "s/provides = \"indy-cli \(= [(,),0-9,.]+\)\"/provides = \"indy-cli \(= ${VERSION}\)\"/g" Cargo.toml
sed -i -E -e "s/name = \"indy-cli\"/name = \"indy-cli-async\"/g" Cargo.toml
sed -i -E -e "/maintainer-scripts = \".\/debian\"/d" Cargo.toml
cargo deb --no-build --deb-version ${VERSION} --variant indy-cli-${PACKAGE_TYPE}
mv -f Cargo.toml.backup Cargo.toml
popd
