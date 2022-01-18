#!/bin/sh

xhost +

docker run --rm \
       --privileged \
       -v /tmp/.X11-unix:/tmp/.X11-unix \
       -e "DISPLAY=unix${DISPLAY}" \
       -e QT_DEVICE_PIXEL_RATIO \
       --device /dev/snd \
       --device /dev/dri \
       --group-add audio \
       --group-add video \
       -v $HOME:$HOME \
       --name rspace \
       rspace
