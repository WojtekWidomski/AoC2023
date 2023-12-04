import queue

FILENAME = "input.in"

card_points = []

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
                points += 1
        
        card_points.append(points)
    
Q = queue.SimpleQueue()

for c in range(len(card_points)):
    Q.put(c)

card_number = 0

while not Q.empty():
    c = Q.get()
    card_number += 1
    for i in range(c+1, c+1+card_points[c]):
        Q.put(i)

print(card_number)
