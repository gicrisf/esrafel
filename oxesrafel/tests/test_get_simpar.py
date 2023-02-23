#!/usr/bin/env python3
#
from oxesrafel import get_from_sim
import matplotlib.pyplot as plt

with open("tests/data/sr-example-acn.sim") as f:
        points, sweep, rads = get_from_sim(f.read())
        print("points: {}".format(points))
        print("sweep: {}".format(sweep))
        print("rads: {}".format(rads[0].lwa.val))
