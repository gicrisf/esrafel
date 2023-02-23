#!/usr/bin/env python3

from oxesrafel import ascii_import, ascii_to_json
import matplotlib.pyplot as plt

spectrum = "na-example-acn"

with open("tests/data/{}.txt".format(spectrum)) as f:
        idx, x_fld, y_int = ascii_import(f.read())
        len_x = len(x_fld)
        len_y = len(y_int)
        if (len_x == len_y) and (len_x > 0):
            print("success. length is {}".format(len_x))

        print("now plotting...")
        plt.plot(x_fld, y_int)
        plt.savefig('tests/img/{}.png'.format(spectrum), dpi=500)
        # plt.show()
        print("ascii_import end of the test.")

# This works, it's just annoying
# with open("tests/data/{}.txt".format(spectrum)) as f:
        # json_spectrum = ascii_to_json(f.read())
        # print(json_spectrum)
        # print("ascii_as_json end of the test.")
