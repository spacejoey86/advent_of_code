from intervaltree import Interval, IntervalTree

class Range:
    def __init__(self, start, length):
        self.start = start
        self.length = length
        # self.map_range = map_range

class Almanac:
    entries: list[tuple[int, int, int]]
    def __init__(self):
        self.entries = []
        self.sorted: bool = True
    def add_mapping(self, dest_range_start: int,
                    source_range_start: int,
                    length: int):
        # check for overlap
        for entry in self.entries:
            existing_d_range_start, existing_s_range_start, existing_len = entry
            if ((source_range_start + length - 1 >= existing_s_range_start) and
                (source_range_start < existing_s_range_start + existing_len)):
                raise LookupError("ranges overlap")
        self.entries.append((dest_range_start, source_range_start, length))
        print(dest_range_start, source_range_start, length)
        sorted = False
    def lookup(self, key: int) -> int:
        if not self.sorted:
            self.entries.sort()
        print("Looking up", key)
        for entry in self.entries:
            d_range_start, s_range_start, length = entry
            print("\tentry:", d_range_start, s_range_start, length)
            if key >= s_range_start and key < s_range_start + length:
                print("lookup succeeded")
                return key - s_range_start + d_range_start
        print("lookup fell through")
        return key
    def lookup_range(self, source_start: int, source_len: int) -> list[tuple[int, int]]:
        output: list[tuple[int, int]] = []
        for entry in self.entries:
            existing_d_range_start, existing_s_range_start, existing_len = entry
            if ((source_start + existing_len - 1 >= existing_s_range_start) and
                (source_start < existing_s_range_start + existing_len)):
                o_r_start = self.lookup(source_start)
                remaining = source_start + source_len - existing_s_range_start
                if remaining > 0:
                    output.extend(self.lookup_range(source_start + source_len - remaining))



with open("input.txt") as file:
    seeds = map(int, file.readline()[7:].split())

    almanacs: list[Almanac] = []
    nextIsNewAlmanac = False
    for line in file.readlines():
        if nextIsNewAlmanac:
            almanacs.append(Almanac())
            nextIsNewAlmanac = False
            continue
        if line == "\n":
            nextIsNewAlmanac = True
            continue
        almanacs[-1].add_mapping(*map(int, line.split()))

minimum: int = None
for seed in seeds:
    for almanac in almanacs:
        seed = almanac.lookup(seed)
    print(seed)
    if minimum == None:
        minimum = seed
    else:
        minimum = min(minimum, seed)

# print(len(almanacs))
print(minimum)