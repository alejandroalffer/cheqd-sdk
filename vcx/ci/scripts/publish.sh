#!/bin/bash
set +x
set -e
if [[ $# -ne 3  && $# -ne 4 ]]; then
    echo "USAGE: $0 CREDENTIALS FILE URL [SUB_DIR]"
    exit 1
fi

CREDENTIALS=$1
FILENAME=$2
URL=$3
SUB_DIR=$4
LOOKUP_DIR="output"

if [ ! -z "${SUB_DIR}" ]; then
      LOOKUP_DIR="${LOOKUP_DIR}/${SUB_DIR}"
fi

echo "Filename: ${FILENAME}"
echo "TYPE: ${TYPE}"
echo "URL: $URL"

echo 'info:'
pwd
ls -al
echo 'end info'

find "./${LOOKUP_DIR}" -type f -name ${FILENAME} -exec curl -u $CREDENTIALS -X POST $URL -F 'file=@{}' \;


