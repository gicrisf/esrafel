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
                                         ]),
                                # A second radical
                                Radical(Param(1.0, 0.0),
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

how_many_rads = len(sim.rads)
print("How many Radicals in the Simulator?\n{}".format(how_many_rads))

if how_many_rads == 2:
    print("test passed.")
