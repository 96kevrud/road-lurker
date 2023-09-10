#!/bin/bash

run_local () {
    export TRAFIKVERKET_APIKEY="$(cat .apikey)"
    cd road-lurker
    cargo run
}

run_local
