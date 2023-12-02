with open("data.txt") as data:
    sum = 0
    for line in data.read().split("\n"):
        first = None
        last = None
        for c in line:
            # print(c,first,last)
            if c.isdigit():
                if first == None:
                    first = int(c)
                last = int(c)
        if first == None and last == None:
            pass
            #empty line
        elif first == None or last == None:
            raise "failed to parse line"
        else:
            value = 10 * first + last
            # print(value)
            sum += value
    print(sum)