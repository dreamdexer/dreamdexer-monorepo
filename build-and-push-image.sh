#!/bin/bash

while getopts s:t: flag
do
    case "${flag}" in
        s) service=${OPTARG};;
        t) tag=${OPTARG};;
    esac
done


docker build . --no-cache --build-arg service=$service -t registry.digitalocean.com/dreamdexer-registry/$service:$tag;
docker push registry.digitalocean.com/dreamdexer-registry/$service:$tag;
