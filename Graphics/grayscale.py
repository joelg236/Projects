import cv2
import argparse

parser = argparse.ArgumentParser()
parser.add_argument('-i', action="store", dest="image", help="image file")
parser.add_argument('-o', action="store", dest="output", help="output")
options = parser.parse_args()

if options.image == None or options.output == None:
    parser.print_help()
    exit()

img = cv2.imread(options.image)
gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)

cv2.imwrite(options.output, gray)
