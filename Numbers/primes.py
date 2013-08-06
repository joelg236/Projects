print 'What number should prime factors be found from?'
n = int(eval(raw_input()))

def factors(o):
    a = []
    for x in range(1, o + 1):
        if o % x == 0:
            a.append(x)
    return a

def isPrime(x):
    return len(factors(x)) == 2

def primeFactors(x):
    a = []
    for f in factors(x):
        if isPrime(f):
            a.append(f)
    return a
    
print primeFactors(n)
