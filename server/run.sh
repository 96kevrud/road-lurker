#!/bin/bash

run () {
    docker build -t road-lurker .
    docker run -p 8000:8000 road-lurker
}

run
