FILENAME = "input.in"

cal_sum = 0

digits = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}

with open(FILENAME) as f:
    lines = f.readlines()
    for l in lines:
        if l == "\n":
            break
        first_digit = None
        first_digit_pos = 1000000
        last_digit = None
        last_digit_pos = 1000000
        for i in range(len(l)):
            c = l[i]
            if c.isdigit():
                first_digit = c
                first_digit_pos = i
                break
        for i in range(len(l)):
            c = "".join(reversed(l))[i]
            if c.isdigit():
                last_digit = c
                last_digit_pos = i
                break

        min_first_digit_pos = -1
        min_last_digit_pos = -1
        min_first_digit = None
        min_last_digit = None

        for digit_name in digits:
            first_digit_name_pos = l.find(digit_name)
            last_digit_name_pos = "".join(reversed(l)).find("".join(reversed(digit_name)))

            if first_digit_name_pos != -1 and (min_first_digit_pos == -1 or first_digit_name_pos < min_first_digit_pos):
                min_first_digit_pos = first_digit_name_pos
                min_first_digit = digit_name
            if last_digit_name_pos != -1 and (min_last_digit_pos == -1 or last_digit_name_pos < min_last_digit_pos):
                min_last_digit_pos = last_digit_name_pos
                min_last_digit = digit_name
        
        if min_first_digit != None and min_first_digit_pos < first_digit_pos:
            first_digit = str(digits[min_first_digit])
        if min_last_digit != None and min_last_digit_pos < last_digit_pos:
            last_digit = str(digits[min_last_digit])
        
        cal_sum += int(first_digit + last_digit)


print(cal_sum)
