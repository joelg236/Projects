primes = []
current = 1

while STDIN.gets == "\n"
    while true
        current += 1
        is_prime = true
        for prime in primes
            if current % prime == 0
                is_prime = false
                break
            end
        end
        if is_prime
            primes.push(current)
            break
        end
    end

    puts current
end
