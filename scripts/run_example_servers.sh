#!/bin/bash
BASEDIR=$(dirname $0)

docker-compose -p balanser -f $BASEDIR/docker-compose.yml up --scale hello=3 -d
