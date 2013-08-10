cost = input("What is the cost in dollars? ")
money = input("Money given: ")
change = int(round(money - cost, 2) * 100)

if change < 0:
    print("You didn't pay enough!")
    exit()

dollars = change / 100
change %= 100

quarters = change / 25
change %= 25

dimes = change / 10
change %= 10

nickels = change / 5
change %= 5

pennies = change

print("""Your change is:
    %i dollars
    %i quarters
    %i dimes
    %i nickels
    %i pennies
""" % (dollars, quarters, dimes, nickels, pennies))
