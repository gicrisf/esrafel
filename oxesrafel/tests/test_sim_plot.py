#!/usr/bin/env python3

from oxesrafel import Radical, Nucleus, Param, Simulator

sim = Simulator(100.0, 1024.0, [Radical(Param(1.0, 0.0),
                                        Param(50.0, 0.0),
                                        Param(100.0, 0.0),
                                        Param(0.0, 0.0),
                                        [Nucleus(Param(0.5, 0.0),
                                                 Param(15.0, 0.0),
                                                 Param(1.0, 0.0)),
                                         Nucleus(Param(2.5, 0.0),
                                                 Param(4.0, 0.0),
                                                 Param(1.0, 0.0))
                                         ])])

theor = sim.calc()

import matplotlib.pyplot as plt
x = range(0, len(theor))
plt.ylabel('Intensity')
plt.xlabel('Field')

plt.xticks([])
plt.yticks([])

plt.plot(x, theor)
plt.show()
