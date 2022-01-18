#!/bin/sh

docker build -f docker/Dockerfile --network=host -t rspace .

