i = input("Enter input:")
inputs = i.split()
for x in inputs:
    print(x[1:] + '-' + x[0] + 'ay', end=' ')
print()
