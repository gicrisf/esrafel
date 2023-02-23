#!/usr/bin/env python3
from oxesrafel import sim_as_json
import json

# https://stackoverflow.com/questions/5508509/how-do-i-check-if-a-string-is-valid-json-in-python
# You can try to do json.loads(),
# which will throw a ValueError if the string you pass can't be decoded as JSON.
def is_json(myjson):
  try:
    json.loads(myjson)
  except ValueError as e:
    return False
  return True

example = "cs-example-acn"
with open("tests/data/{}.sim".format(example)) as f:
        json_pars = sim_as_json(f.read())
        # print(json_pars)
        if is_json(json_pars):
            print("Success! json_pars end of the test.")
        else:
            "test failed."

# TODO Manipulate and export as csv or other formats
