#!/usr/bin/env python3

from oxesrafel import Radical, Nucleus
rad = Radical.probe()
print("LWA value is {}", rad.lwa.val)
print("Lrtz value is {}", rad.lrtz.val)
print("Amount value is {}", rad.amount.val)
print("dh1 value is {}", rad.dh1.val)
# TODO Nuclei getter (as PyList)
# print("How many nuclei? {}", rad.nucs.len())

rad.push_nuc(Nucleus.probe())
rad.pop_nuc()
print("Test passed.")
