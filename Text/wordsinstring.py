i = input("Enter string\n");

words = 0
for x in i.split(' '):
    if x != "":
        words += 1
print(words, "words in this string")
