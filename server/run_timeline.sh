#!/bin/bash

run_cargo () {
    export TRAFIKVERKET_APIKEY="$(cat .apikey)"
    cd road-lurker
    cargo run --bin timeline
}

run_cargo
