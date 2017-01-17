#!/usr/bin/env bash
set -e

cd /mnt/ramdisk

pkill PTPCamera || true

gphoto2 \
  --set-config capturetarget=0 \
  --set-config imageformat=0 \
  --set-config shutterspeed=4 \
  --set-config iso=19 \
  --capture-image-and-download \
  --force-overwrite

solve-field --scale-units degwidth --scale-low 10.2 --scale-high 10.4 --overwrite --downsample 4 --no-plots --new-fits none --parity neg ./capt0000.jpg
