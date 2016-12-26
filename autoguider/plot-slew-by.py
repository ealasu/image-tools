#!/usr/bin/env python3

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
                ra += float(m.group(1))
                dec += float(m.group(2))
                # res.append(ra)
                res.append({'x': x, 'ra': ra, 'dec': dec})
                x += 1
    return res

def plot(data, name):
    # data = assemble(data)
    p = ggplot(aes(x='x', y='ra'), data=DataFrame(data)) \
        + geom_line()
        # + scale_y_continuous(limits=(0, 30))
        # + stat_smooth(colour='blue', span=0.2)
    filename = f'{name}.svg'
    print(filename)
    p.save(filename)

def main():
    file = './log/autoguider.log'
    data = parse(file)
    plot(data, file)

main()
