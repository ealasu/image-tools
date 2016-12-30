#!/usr/bin/env bash

f=$1

gphoto2 --filename $f.jpg --force-overwrite --set-config imageformat=0 --capture-image-and-download
convert jpeg:$f.jpg \
  -gravity center \
            -extent 2000x2000 \
            -crop 2000x2000 \
            -depth 16 \
            -set colorspace Gray \
            -separate \
            -average \
            fits:$f.fits
