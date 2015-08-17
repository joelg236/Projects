import os
import argparse

parser = argparse.ArgumentParser(description='Copy files in bulk')
parser.add_argument('-f', nargs='+', action="store", dest="files",
        help="files to copy", default="")
parser.add_argument('-t', nargs='+', action="store", dest="dest",
        help="copy to", default="")
options = parser.parse_args()

filesFrom = options.files
filesTo   = dest

print filesFrom
