#!/usr/bin/env python3

import sys
import re
from ggplot import *
from pandas import DataFrame
from pprint import pprint

def assemble(data):
    res = []
    x = 0
    for v in data:
        res.append({'x': x, 'y': v})
        x += 1
    return res

def parse(path):
    res = []
    x = 0
    ra = 0.0
    dec = 0.0
    with open(path, 'r') as f:
        for line in f:
            m = re.search(r' slew_by ra: (.+), dec: (.+)', line)
            if m:
                ra = 0.0
                dec = 0.0
                ra += float(m.group(1))
                dec += float(m.group(2))
                # res.append(ra)
                res.append({'x': x, 'ra': ra, 'dec': dec})
                x += 1
    return res

def plot(data, axis, filename):
    # data = assemble(data)
    p = ggplot(aes(x='x', y=axis), data=DataFrame(data)) \
        + geom_line()
        # + scale_y_continuous(limits=(0, 30))
        # + stat_smooth(colour='blue', span=0.2)
    p.save(filename)

def main():
    file = sys.argv[1]
    data = parse(file)
    plot(data, 'ra', f'{file}.slew.ra.svg')
    plot(data, 'dec', f'{file}.slew.dec.svg')

main()
