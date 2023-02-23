#!/usr/bin/env python3
# TODO
from oxesrafel import get_from_sim

with open("tests/data/sr-example-acn.sim") as f:
        points, sweep, rads = get_from_sim(f.read())
