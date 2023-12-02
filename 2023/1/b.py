number_strings = {"zero":0,
 "one":1,
 "two":2,
 "three":3,
 "four":4,
 "five":5,
 "six":6,
 "seven":7,
 "eight":8,
 "nine":9}

# with open("test_input_b.txt") as data:
with open("input.txt") as data:
    sum = 0
    for line in data.read().split("\n"):
        print("=======\n\n", line)
        first = None
        for index in range(len(line)):
            # print(line[index], "scanning")
            if line[index].isdigit():
                first = int(line[index])
                break
            for k in number_strings.keys():
                try:
                    # print("\t", k, line.index(k), index)
                    if line.index(k) == index:
                        first = number_strings[k]
                        break
                except ValueError:
                    pass
            else:
                continue
            break
        # print("first:", first)

        last = None
        for index in range(len(line) - 1, -1, -1):
            # print(line[index], "scanning")
            if line[index].isdigit():
                last = int(line[index])
                break
            for k in number_strings.keys():
                try:
                    # print("\t", k, line.index(k), index)
                    if line.rindex(k) == index:
                        last = number_strings[k]
                        break
                except ValueError:
                    pass
            else:
                continue
            break

        # print(line, "->", first)

        if first == None and last == None:
            pass
            #empty line
        elif first == None or last == None:
            raise "failed to parse line"
        else:
            value = 10 * first + last
            print(line, ":", value)
            sum += value
    print(sum)
