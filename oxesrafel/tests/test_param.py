#!/usr/bin/env python3

from oxesrafel import Param
value = 3.0
variation = 2.0

print("Creating a new Parameter with value: {}; variation: {}", value, variation)
new_par = Param(value, variation)
print("Getting value:")
print(new_par.val)
print("Getting variation:")
print(new_par.var)

value = 5.0
print("Setting new value as {}", value)
print("Get value:")
print(new_par.val)
print("Test passed.")

print("Randomizing values...")
new_par.randomize()
print("Getting new value:")
print(new_par.val)
print("Getting new variation:")
print(new_par.var)
