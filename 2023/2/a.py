# total_red = 12
# total_green = 13
# total_blue = 14
totals = {"red":12, "green":13, "blue":14}
count_possible = 0

for line in open("input.txt"):
    line = line.rstrip()
    game = line[5:line.index(":")]
    reveals = line[line.index(":")+2:].split(";")
    for reveal in reveals:
        for colour_rev in reveal.split(","):
            l = colour_rev.strip().split()
            num = l[0]
            col = l[1]
            if int(num) > totals[col]:
                break
        else:
            continue
        break
    else:
        count_possible += int(game)


    # print(reveals)
print(count_possible)