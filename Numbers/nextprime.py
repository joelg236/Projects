def factors(o):
    a = []
    for x in range(1, o + 1):
        if o % x == 0:
            a.append(x)
    return a

def is_prime(x):
    return len(factors(x)) == 2

def next_prime(x):
    while True:
        x = x + 1
        if is_prime(x):
            return x

input('press enter for next')

i = 0

while(x == ''):
    i = next_prime(i)
    print(i)
    x = raw_input()
