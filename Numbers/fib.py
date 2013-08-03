print 'To which number should be calculated?'
n = int(eval(raw_input()))

x = [0]
for i in range(n):
    if i < 1:
        x.append(1)
    x.append(x[len(x) - 1] + x[len(x) - 2])

print x
