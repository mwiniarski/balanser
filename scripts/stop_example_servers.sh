#!/bin/bash
BASEDIR=$(dirname $0)

docker-compose -p balanser -f $BASEDIR/docker-compose.yml down -t 1
