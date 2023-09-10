#!/bin/bash

run_local () {
    echo "hej"
    export TRAFIKVERKET_APIKEY="$(cat .apikey)"
    cargo run --manifest-path road-lurker/Cargo.toml
}

run_local
