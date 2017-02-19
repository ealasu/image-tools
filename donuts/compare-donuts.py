#!/usr/bin/env python3

from subprocess import run, PIPE

with open('images', 'r') as f:
  images = [line.strip() for line in f]

ref = images[0]

for i in images[1:]:
  old = run(['./autoguider/run-donuts', ref, i], check=True, stdout=PIPE).stdout.decode('utf8')
  new = run(['./target/release/donuts-cli', ref, i], check=True, stdout=PIPE).stdout.decode('utf8')
  print(f'{i}\t{old}\t{new}')
