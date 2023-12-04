FILENAME = "input.in"

points_sum = 0

with open(FILENAME) as f:
    lines = f.readlines()
    for l in lines:
        if l == "\n":
            break
        winning_str, numbers_str = l.rstrip().split(" | ")
        winning_str = winning_str.split(": ")[1]
        winning_set = set(winning_str.split(" "))
        numbers = list(numbers_str.split(" "))

        points = 0

        for n in numbers:
            if n == '':
                continue
            if n in winning_set:
                if points == 0:
                    points = 1
                else:
                    points *= 2
        
        points_sum += points

print(points_sum)
