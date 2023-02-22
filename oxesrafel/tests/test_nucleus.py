#!/usr/bin/env python3

from oxesrafel import Param, Nucleus
eqs = Param(1.0, 0.0)
spin = Param(1.0, 0.0)
hpf = Param (15.0, 1.0)

nuc = Nucleus(spin, hpf, eqs)
print("Spin value is {}", nuc.spin.val)
print("Hpf value is {}", nuc.hpf.val)
print("Eqs value is {}", nuc.eqs.val)

nuc.hpf = Param(10.0, 1.0)
print("New Hpf value is {}", nuc.hpf.val)

probe = Nucleus.probe()
print("Spin value is {}", probe.spin.val)
print("Hpf value is {}", probe.hpf.val)
print("Eqs value is {}", probe.eqs.val)
print("Test passed.")
