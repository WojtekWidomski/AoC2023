FILENAME = "input.in"

power_sum = 0

with open(FILENAME) as f:
    lines = f.readlines()
    for l in lines:
        if l == "\n":
            break

        min_cubes = {
            "red": 0,
            "blue": 0,
            "green": 0
        }

        id, data = l.rstrip().split(": ")

        id = int(id.split(" ")[1]) # convert "Game i" to i

        cube_sets = data.split("; ")
        for s in cube_sets:
            cubes_data = s.split(", ")
            for c in cubes_data:
                nmb, col = c.split(" ")
                nmb = int(nmb)
                if nmb > min_cubes[col]:
                    min_cubes[col] = nmb
        
        power_sum += min_cubes["red"]*min_cubes["green"]*min_cubes["blue"]

print(power_sum)
