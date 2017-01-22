#!/usr/bin/env bash
#set -e

echo 'aligning'
/Volumes/data/home/code/image-tools/target/release/align --output=align.json good/*
echo 'stacking'
/Volumes/data/home/code/image-tools/target/release/stack --output=out.fits --flat=/Volumes/data/home/pictures/mount-test/flats/f6.7/20170117/flat.fits --alignment=align.json
echo 'post-processing'
/Volumes/data/home/code/image-tools/target/release/post --output=out.tif out.fits
#echo 'converting to jpeg'
#convert out.tif -modulate 300 -quality 100 out.jpg
