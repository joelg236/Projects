def b2d(x):
    i = 0
    for c in range(0, len(x)):
        if x[c] == '1':
            i += 2 ** (len(x) - c - 1)
    return i

def d2b(x):
    i = []
    while x > 0:
        i.append(x % 2)
        x /= 2
    i.reverse()
    c = ""
    for a in i:
        c += str(a)
    return c

if input('Binary to decimal (1) or decimal to binary (0)? ') == 1:
    print b2d(raw_input('Enter number: '))
else:
    print d2b(input('Enter number: '))
