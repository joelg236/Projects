fib = [1, 2]

for i in 1..ARGV[0].to_i
    fib.push(fib.last + fib[fib.size - 2])
end

puts fib[0..(ARGV[0].to_i - 1)]
