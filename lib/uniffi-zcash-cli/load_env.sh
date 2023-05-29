#!/bin/bash

SCRIPT_DIR="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

if [ -f $SCRIPT_DIR/.env ]; then
  export $(echo $(cat $SCRIPT_DIR/.env | sed 's/#.*//g'| xargs) | envsubst)
else
  echo "could not find .env file at ${SCRIPT_DIR}"
fi

