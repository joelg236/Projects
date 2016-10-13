dollars = ARGV[0].split('.')[0].to_i
cents = ARGV[0].split('.')[1].to_i

hundreds = dollars / 100
dollars -= hundreds * 100

fifties = dollars / 50
dollars -= fifties * 50

twenties = dollars / 20
dollars -= twenties * 20

tens = dollars / 10
dollars -= tens * 10

fives = dollars / 5
dollars -= fives * 5

twoonies = dollars / 2
dollars -= twoonies * 2

loonies = dollars

quarters = cents / 25
cents -= quarters * 25

dimes = cents / 10
cents -= dimes * 10

nickles = cents / 5
cents -= nickles * 5

pennies = cents

puts "100 x " + hundreds.to_s
puts "50 x " + fifties.to_s
puts "20 x " + twenties.to_s
puts "10 x " + tens.to_s
puts "5 x " + fives.to_s
puts "2 x " + twoonies.to_s
puts "1 x " + loonies.to_s
puts "0.25 x " + quarters.to_s
puts "0.10 x " + dimes.to_s
puts "0.05 x " + nickles.to_s
puts "0.01 x " + pennies.to_s
