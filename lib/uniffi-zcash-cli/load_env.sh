#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

if [ -f $SCRIPT_DIR/.env ]; then
  export $(echo $(cat .env | sed 's/#.*//g'| xargs) | envsubst)
fi