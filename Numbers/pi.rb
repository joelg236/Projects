include Math

def pi_digit(x)
    return ((Math::PI * (10 ** (x - 1))) % 10).to_i
end

puts pi_digit(ARGV[0].to_i)
