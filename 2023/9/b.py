class Sequence:
    def __init__(self, numbers: list[int]):
        self.numbers = numbers
        print(self.numbers)
    def findDifferences(self):
        self.differences: list[int] = []
        for i in range(len(self.numbers) - 1):
            self.differences.append(self.numbers[i+1] - self.numbers[i])
    def extrapolate(self) -> int:
        self.findDifferences()
        print("> ", self.differences)
        if not all(elem == 0 for elem in self.differences):
            self.differences.insert(0, Sequence(self.differences).extrapolate())
            return self.numbers[0] - self.differences[0]
        else:
            return self.numbers[0]

total = 0
with open("input.txt") as file:
    for line in file.readlines():
        total += Sequence(list(map(int,line.split()))).extrapolate()
        print("=======")

print(total)
