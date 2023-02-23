#!/usr/bin/env python3
from oxesrafel import Radical, Nucleus, Param, Simulator, get_from_sim
import matplotlib.pyplot as plt

# available:
# cs-example-acn.sim
# sr-example-acn.sim
with open("tests/data/cs-example-acn.sim") as f:
        points, sweep, rads = get_from_sim(f.read())
        sim = Simulator(sweep=sweep, points=points, rads=rads)

        theor = sim.calc()

        x = range(0, len(theor))
        plt.ylabel('Intensity')
        plt.xlabel('Field')

        plt.xticks([])
        plt.yticks([])

        plt.plot(x, theor)
        plt.savefig('tests/img/cs-example-acn.png', dpi=500)
