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
elif [ "${RUST_FLAG}" == "mysql" ]; then
    echo "Testing libvcx.so with mysql wallet"
    mysql -h mysql -u root -p${MYSQL_ROOT_PASSWORD} ${MYSQL_DATABASE} < ../ci/db/wallet_schema_creation.2018-05-07.sql
    echo "show tables;" | mysql -h mysql -u root -p${MYSQL_ROOT_PASSWORD} wallet
    mkdir logs
    RUST_LOG=trace TEST_POOL_IP=$INDY_POOL_PORT_9701_TCP_ADDR cargo test --features="mysql" -- --test-threads=1 2>logs/libvcx-log.txt
else
    echo "Skip testing of libvcx.so"
fi

echo "Building libvcx.so"
cargo build --no-default-features --features "ci"
echo "Updating libvcx.so File with Version"
cargo update-so
echo "Creating Libvcx Debian File"
FINAL_VERSION=${VERSION}-${PACKAGE_TYPE}
if [[ $CI_COMMIT_REF_SLUG != "stable" ]];
then
    FINAL_VERSION=${FINAL_VERSION}~${CI_PIPELINE_IID}
fi
cargo deb --no-build --deb-version ${FINAL_VERSION} --variant libvcx-${PACKAGE_TYPE}
echo "Moving Libvcx Debian File to Output Directory"
cp target/debian/*.deb $CURDIR/$OUTPUTDIR
