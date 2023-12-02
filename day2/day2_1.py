FILENAME = "input.in"

cube_max_numbers = {
    "red": 12,
    "green": 13,
    "blue": 14
}

possible_id_sum = 0

with open(FILENAME) as f:
    lines = f.readlines()
    for l in lines:
        if l == "\n":
            break

        possible = True
        id, data = l.rstrip().split(": ")

        id = int(id.split(" ")[1]) # convert "Game i" to i

        cube_sets = data.split("; ")
        for s in cube_sets:
            cubes_data = s.split(", ")
            for c in cubes_data:
                nmb, col = c.split(" ")
                if (int(nmb) > cube_max_numbers[col]):
                    possible = False
        
        if possible:
            possible_id_sum += id

print(possible_id_sum)
