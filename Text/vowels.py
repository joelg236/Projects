string = input("Enter string\n")

vowels = ['a', 'e', 'i', 'o', 'u']
count = [0 for x in range(len(vowels))]

for s in string:
    if s in vowels:
        count[vowels.index(s)] += 1
print(vowels)
print(count)
print('total', sum(count))
