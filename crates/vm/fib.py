def fibonacci_of(n):
    return n if n in {0, 1} else fibonacci_of(n - 1) + fibonacci_of(n - 2)


print(fibonacci_of(10))
