#!/bin/bash

run_docker () {
    TRAFIKVERKET_APIKEY="$(cat .apikey)"
    docker build -t road-lurker --build-arg TRAFIKVERKET_APIKEY=${TRAFIKVERKET_APIKEY} .
    docker run -p 8000:8000 road-lurker
}

run_docker
