import random

def generate_key():
    while True:
        seed = random.getrandbits(32)
        yield seed

generate = generate_key()

while raw_input("Press enter for key, q to quit\n") != "q":
    print(next(generate))
