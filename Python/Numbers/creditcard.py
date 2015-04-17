def checksum(card):
    def numbers(string):
        return [int(x) for x in string]

    card_without_check = card[::-1][1:]
    print(card_without_check)
    odd_numbers = numbers(card_without_check[0::2])
    even_numbers = numbers(card_without_check[1::2])

    odd_numbers = [x * 2 for x in odd_numbers]
    odd_numbers = [x - 9 if x > 9 else x for x in odd_numbers]
    print(odd_numbers)
    print(even_numbers)
    return (sum(odd_numbers) + sum(even_numbers)) % 10

def check(card):
    return checksum(card) == int(card[-1])

card = str(input("Enter card number:\n"))
print("Valid? ", check(card))
