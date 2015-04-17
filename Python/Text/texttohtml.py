import markdown2
import sys

md = ""
if len(sys.argv) > 1:
    with open(sys.argv[1], "r") as fi:
        md = fi.read()
else:
    md = input("Input markdown in one line\n")

print(markdown2.markdown(md))
