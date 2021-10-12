#!/bin/bash

if [ $# -ne 0 ]
  then
    echo "ERROR: Incorrect number of arguments"
    echo "Usage:"
    echo "$0"
    exit 1
fi

function publish() {
  MODULE_DIR=$1

  pushd $MODULE_DIR
  mvn versions:set -DnewVersion=$DEV_VERSION
  mvn clean deploy -DskipTests -Dmaven.javadoc.skip=true --settings settings.xml
  popd
}

set -eux
publish wrappers/java