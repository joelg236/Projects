import math

x = ""
while True:
    x = raw_input()
    if x.startswith('sin('):
        x = math.sin(float(x[4 : len(x) - 1]))
    elif x.startswith('cos('):
        x = math.cos(float(x[4 : len(x) - 1]))
    elif x.startswith('tan('):
        x = math.tan(float(x[4 : len(x) - 1]))
    elif x.startswith('asin('):
        x = math.asin(float(x[5 : len(x) - 1]))
    elif x.startswith('acos('):
        x = math.acos(float(x[5 : len(x) - 1]))
    elif x.startswith('atan('):
        x = math.atan(float(x[5 : len(x) - 1]))
    elif x.startswith('factorial('):
        x = math.factorial(float(x[10 : len(x) - 1]))
    elif x.startswith('log('):
        x = math.log(float(x[4 : len(x) - 1]))
    else:
        x = eval(x)
    print x
