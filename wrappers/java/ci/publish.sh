#!/bin/bash

if [ $# -ne 2 ]
  then
    echo "ERROR: Incorrect number of arguments"
    echo "Usage:"
    echo "$0 <branch> <pipeline uid>"
    exit 1
fi


function publish() {
  MODULE_DIR=$1

  cp settings.xml $MODULE_DIR

  pushd $MODULE_DIR
  mvn clean deploy -DskipTests -Dmaven.javadoc.skip=true --settings settings.xml
  popd
}

function update_version() {
  MODULE_DIR=$1
  BRANCH=$2
  UUID=$3

  pushd $MODULE_DIR
  if [ "$BRANCH" == "master" ]
  then
    sed -i -E -e "H;1h;\$!d;x" -e "s/<version>([0-9,.]+)</<version>\1-${UUID}$</" pom.xml
  fi
  if [ "$BRANCH" != "stable" ]
  then
    sed -i -E -e "H;1h;\$!d;x" -e "s/<version>([0-9,.]+)</<version>\1-${UUID}-${BRANCH}</" pom.xml
  fi
  popd
}

set -eux

update_version wrappers/java $1 $2
publish wrappers/java