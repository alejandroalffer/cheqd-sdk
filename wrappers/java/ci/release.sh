#!/bin/bash
function release() {
  MODULE_DIR=$1

  pushd $MODULE_DIR
  gpg --batch --passphrase $OSSRH_GPG_PASSPHRASE --import $OSSRH_GPG_SIGNING_KEY # set GPG key to keyring
  export RELEASE_VERSION=`echo $CI_COMMIT_TAG | cut -c2-`                        # pull version from tag name
  mvn versions:set -DnewVersion=$RELEASE_VERSION                                 # set version to tagged version
  mvn clean deploy -DskipTests -Dmaven.javadoc.skip=true --settings settings.xml
  popd
}

set -eux
release wrappers/java