schematic: list[list[str]] = []
numbers = list([str(n) for n in range(0,10)])
print(numbers)
print("4" in numbers)

inNumber = False
parts = []
currentLine = -1
currentCol = 0

for line in open("input.txt"):
    currentLine += 1
    inNumber = False
    currentCol = 0
    currentNum = ""

    schematic.append([])
    for c in line:
        if c == "\n":
            continue
        if c in numbers:
            if not inNumber:
                parts.append({"row":currentLine, "cols":[], "adjacent":False, "val":""})
                inNumber = True
            parts[-1]["cols"].append(int(currentCol))
            parts[-1]["val"] += c
        else:
            inNumber = False
        schematic[-1].append(not(c in numbers or c == "."))
        currentCol += 1


# print(parts)
# print(schematic)


def inRange(row,col):
    if row > 0 and col > 0 and row < len(schematic) and col < len(schematic[0]):
        return True
    return False


for part in parts:
    row = part["row"]
    for col in part["cols"]:
        for x in [-1,0,1]:
            for y in [-1,0,1]:
                if inRange(nRow := row + y, nCol := col + x):
                    if schematic[nRow][nCol]:
                        part["adjacent"] = True

sum = 0
for part in parts:
    if part["adjacent"]:
        sum += int(part["val"])

print(sum)

# list of lists of positions of part numbers
