count_powers = 0

for line in open("input.txt"):
    line = line.rstrip()
    game = line[5:line.index(":")]
    reveals = line[line.index(":")+2:].split(";")
    current_min = {"red":0, "green":0, "blue":0}
    for reveal in reveals:
        for colour_rev in reveal.split(","):
            l = colour_rev.strip().split()
            num = l[0]
            col = l[1]
            current_min[col] = max(current_min[col], int(num))
    count_powers += current_min["blue"] * current_min["green"] * current_min["red"]


print(count_powers)
