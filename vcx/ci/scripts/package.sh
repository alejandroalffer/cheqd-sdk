#!/bin/bash
set -e
OUTPUTDIR="output"
CURDIR=$(pwd)
export PATH=${PATH}:$(pwd)/vcx/ci/scripts
cd vcx/libvcx/
RUST_FLAG=$1
VERSION=$2
REVISION=$3
PACKAGE_TYPE=$4

echo "RUST_FLAG ${RUST_FLAG}"
echo "Updating Version in Cargo.toml file"
cargo update-version ${VERSION} ${REVISION}
echo "Updating Cargo"
if [ "${RUST_FLAG}" == "basic-tests" ]; then
    echo "Testing libvcx.so: run basic tests"
    TEST_POOL_IP=$INDY_POOL_PORT_9701_TCP_ADDR cargo test --no-default-features -- --test-threads=1
elif [ "${RUST_FLAG}" == "test"  ]; then
    echo "Testing libvcx.so: run all tests"
    TEST_POOL_IP=$INDY_POOL_PORT_9701_TCP_ADDR cargo test -- --test-threads=1
else
    echo "Skip testing of libvcx.so"
fi

echo "Building libvcx.so"
cargo build --no-default-features --features "ci"
echo "Updating libvcx.so File with Version"
cargo update-so
echo "Creating Libvcx Debian File"
cargo deb --no-build --deb-version ${VERSION}-${REVISION}-${PACKAGE_TYPE} --variant libvcx-${PACKAGE_TYPE}
echo "Moving Libvcx Debian File to Output Directory"
cp target/debian/*.deb $CURDIR/$OUTPUTDIR
