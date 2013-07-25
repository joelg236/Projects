# Find PI to the Nth Digit

from math import *
from decimal import *

# Slightly inacurate, but very good up to ~27 digits
def bellard(n):
    pi = Decimal(0)
    k = 0
    n = int(n)
    while k < n:
        pi += (Decimal(-1)**k/(1024**k))*( Decimal(256)/(10*k+1) + Decimal(1)/(10*k+9) - Decimal(64)/(10*k+3) - Decimal(32)/(4*k+1) - Decimal(4)/(10*k+5) - Decimal(4)/(10*k+7) -Decimal(1)/(4*k+3))
        k += 1
    pi = pi * 1/(2**6)
    return pi

digits = raw_input('Enter number of digits to round PI to: ')

print ('{0:.%df}' % min(30, int(digits))).format(bellard(digits))
