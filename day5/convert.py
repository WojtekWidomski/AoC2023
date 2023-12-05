# Convert input to format, that will be easier in C++
from pprint import pprint
INPUTNAME = "input.in"

seeds = []

maps = []

with open(INPUTNAME) as f:
    lines = f.readlines()
    seeds_str = lines[0].rstrip().split(" ")[1:]
    for s in seeds_str:
        seeds.append(int(s))
        
    
    new_map = True
    for l in lines[2:]:
        if new_map:
            maps.append([])
            
            new_map = False
            continue
        if l == "\n":
            new_map = True
            continue

        map_int = []
        for i in l.rstrip().split(" "):
            map_int.append(int(i))

        maps[-1].append(map_int)

print(len(seeds))

for s in seeds:
    print(s, end=" ")
print("")

for m in maps:
    print(len(m))
    for r in m:
        print(r[0], r[1], r[2])
