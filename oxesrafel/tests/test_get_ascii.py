#!/usr/bin/env python3

from oxesrafel import ascii_import, get_from_ascii
import matplotlib.pyplot as plt

with open("tests/data/na-example-acn.txt") as f:
        x_fld, y_int = ascii_import(f.read())
        len_x = len(x_fld)
        len_y = len(y_int)
        if (len_x == len_y) and (len_x > 0):
            print("success. length is {}".format(len_x))

        print("now plotting...")
        plt.plot(x_fld, y_int)
        plt.savefig('tests/img/na-example-acn.png', dpi=500)
        # plt.show()
        print("end of the test.")
