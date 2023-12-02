FILENAME = "input.in"

cal_sum = 0

with open(FILENAME) as f:
    lines = f.readlines()
    for l in lines:
        if l == "\n":
            break
        first_digit = None
        last_digit = None
        for c in l:
            if c.isdigit():
                first_digit = c
                break
        for c in "".join(reversed(l)):
            if c.isdigit():
                last_digit = c
                break
        cal_sum += int(first_digit + last_digit)

print(cal_sum)
