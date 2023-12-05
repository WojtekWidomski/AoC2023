# from pprint import pprint

FILENAME = "input.in"

seeds = []

maps = dict()

def get_from_map(mapname, x):
    M = maps[mapname]
    for r in M:
        dest_start = r[0]
        src_start = r[1]
        lenght = r[2]

        if x >= src_start and x < src_start + lenght:
            return x + dest_start - src_start
    return x


with open(FILENAME) as f:
    lines = f.readlines()
    seeds_str = lines[0].rstrip().split(" ")[1:]
    for s in seeds_str:
        seeds.append(int(s))
    
    new_map = True
    last_map = None
    for l in lines[2:]:
        if new_map:
            last_map = l.rstrip().split(" ")[0]
            maps[last_map] = []
            new_map = False
            continue
        if l == "\n":
            new_map = True
            continue

        map_int = []
        for i in l.rstrip().split(" "):
            map_int.append(int(i))

        maps[last_map].append(map_int)


min_loc = -1

for s in seeds:
    soil = get_from_map("seed-to-soil", s)
    fert = get_from_map("soil-to-fertilizer", soil)
    water = get_from_map("fertilizer-to-water", fert)
    light = get_from_map("water-to-light", water)
    temp = get_from_map("light-to-temperature", light)
    hum = get_from_map("temperature-to-humidity", temp)
    loc = get_from_map("humidity-to-location", hum)
    if loc < min_loc or min_loc == -1:
        min_loc = loc

print(min_loc)