#!/bin/bash

set -eux

pushd cli
cargo clippy -- -W clippy::style -W clippy::correctness -W clippy::complexity -W clippy::perf
popd
