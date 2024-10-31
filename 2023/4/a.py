def points(matches: int) -> int:
    if matches:
        return 2**(matches - 1)
    return 0


cards: tuple[int, int] = []
total_points = 0
for line in open("input.txt").readlines():
    line = line[5:-1] #remove newline and "Card"
    # print(line)
    cardNum, values = line.split(":")
    winningNumString, actualNumString = values.split("|")
    winningNums = set(winningNumString.split())
    actualNums = set(actualNumString.split())
    # print(winningNums, actualNums)
    # print(winningNums.intersection(actualNums))
    matches = len(winningNums.intersection(actualNums))
    # print(matches, points(matches))
    total_points += points(matches)
    cards.append((matches, 1))
    # print("")

print(cards)

sum = 0
for index, tup in enumerate(cards):
    matches, copies = tup
    print(index, matches, copies)
    sum += copies
    for i in range(matches):
        if i < len(cards):
            c_matches, c_copies = cards[index + i + 1]
            print("\twinning", index + i + 1)
            cards[index + i + 1] = c_matches, c_copies + copies

print("Points:", total_points)
print("Total:", sum)
