#!/usr/bin/env python3
from oxesrafel import Radical, Nucleus, Param, Simulator

sim = Simulator(sweep=100.0,
                points=1024.0,
                rads=[Radical(lwa=Param(1.2, 0.0),
                              lrtz=Param(70.09, 0.0),
                              amount=Param(100.0, 0.0),
                              dh1=Param(-0.3, 0.0),
                              nucs=[Nucleus(spin=Param(1.0, 0.0),
                                       hpf=Param(2.5, 0.0),
                                       eqs=Param(1, 0.0)),
                               Nucleus(spin=Param(1.5, 0.0),
                                       hpf=Param(15.5, 0.0),
                                       eqs=Param(1.0, 0.0))
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
