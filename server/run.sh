#!/bin/bash

run () {
    docker build -t road-lurker2 .
    docker run -p 8000:8000 road-lurker2 
}

run
