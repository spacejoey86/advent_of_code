from enum import Enum
from typing import Self

class TileType(Enum):
    EMPTY = "."
    LRMIRROR = "/"
    RLMIRROR = "\\"
    VSPLITTER = "|"
    HSPLITTER = "-"
    def __repr__(self) -> str:
        return self.name

class Coord:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
    def __repr__(self) -> str:
        return "(" + str(self.x) + "," + str(self.y) + ")"

class Direction(Enum):
    UP = (0, -1)
    DOWN = (0, 1)
    LEFT = (-1, 0)
    RIGHT = (1, 0)
    def movCoord(self, coord: Coord) -> Coord:
        return Coord(coord.x + self.value[0], coord.y + self.value[1])
    def lrmirror(self) -> Self:
        match (self):
            case Direction.UP:
                return Direction.RIGHT
            case Direction.LEFT:
                return Direction.DOWN
            case Direction.DOWN:
                return Direction.LEFT
            case Direction.RIGHT:
                return Direction.UP
    def rlmirror(self) -> Self:
        match (self):
            case Direction.UP:
                return Direction.LEFT
            case Direction.LEFT:
                return Direction.UP
            case Direction.DOWN:
                return Direction.RIGHT
            case Direction.RIGHT:
                return Direction.DOWN

class Tile:
    def __init__(self, tileType: TileType):
        self.tileType = tileType
        self.energised: bool = False
    def __repr__(self) -> str:
        return self.tileType.__repr__()

def inBounds(coord: Coord) -> bool:
    return not (coord.x < 0 | coord.x >= width | coord.y < 0 | coord.y >= height)

contraption: list[list[Tile]] = []

with open("test_input.txt") as file:
    for line in file.readlines():
        contraption.append([])
        for tile_string in line.strip():
            contraption[-1].append(Tile(TileType(tile_string)))

height = len(contraption)
width = len(contraption[0])

# print(contraption)

for line in contraption:
    for tile in line:
        print(tile.tileType.value, end="")
    print()

print()

remaining: list[tuple[Coord, Direction]] = [(Coord(0, 0), Direction.RIGHT)]
visited: set[tuple[Coord, Direction]] = set()

while len(remaining) > 0:
    current = remaining.pop()
    if current in visited:
        print("already visited ", current)
        continue
    current_coord, current_direction = current
    current_tile = contraption[current_coord.x][current_coord.y]
    current_tile.energised = True
    visited.add(current)
    print("visiting", current)
    match (current_tile.tileType):
        case TileType.EMPTY:
            new_coord = current_direction.movCoord(current_coord)
            if inBounds(new_coord):
                remaining.append((new_coord, current_direction))

        case TileType.LRMIRROR:
            new_direction = current_direction.lrmirror()
            new_coord = new_direction.movCoord(current_coord)
            if inBounds(new_coord):
                remaining.append((new_coord, new_direction))

        case TileType.RLMIRROR:
            new_direction = current_direction.rlmirror()
            new_coord = new_direction.movCoord(current_coord)
            if inBounds(new_coord):
                remaining.append((new_coord, new_direction))

        case TileType.VSPLITTER:
            match (current_direction):
                case Direction.UP | Direction.DOWN:
                    new_coord = current_direction.movCoord(current_coord)
                    if inBounds(new_coord):
                        remaining.append((new_coord, current_direction))
                case Direction.LEFT | Direction.RIGHT:
                    coordOne = Direction.UP.movCoord(current_coord)
                    if inBounds(coordOne):
                        remaining.append((coordOne, Direction.UP))
                    coordTwo = Direction.DOWN.movCoord(current_coord)
                    if inBounds(coordTwo):
                        remaining.append((coordTwo, Direction.DOWN))

        case TileType.HSPLITTER:
            match (current_direction):
                case Direction.LEFT | Direction.RIGHT:
                    new_coord = current_direction.movCoord(current_coord)
                    if inBounds(new_coord):
                        remaining.append((new_coord, current_direction))
                case Direction.UP | Direction.DOWN:
                    coordOne = Direction.LEFT.movCoord(current_coord)
                    if inBounds(coordOne):
                        remaining.append((coordOne, Direction.LEFT))
                    coordTwo = Direction.RIGHT.movCoord(current_coord)
                    if inBounds(coordTwo):
                        remaining.append((coordTwo, Direction.RIGHT))
