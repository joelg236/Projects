cost = input("What is the cost in dollars? ")
money = input("Money given: ")
change = money - cost

if change < 0:
    print("You didn't pay enough!")
    exit()

dollars = int(change)
change %= 1

quarters = int(change / 0.25)
change %= 0.25

dimes = int(change / 0.1)
change %= 0.1

nickels = int(change / 0.05)
change %= 0.05

pennies = change / 0.01

print("""Your change is:
    %i dollars
    %i quarters
    %i dimes
    %i nickels
    %.0f pennies
""" % (dollars, quarters, dimes, nickels, pennies))
