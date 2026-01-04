from typing import Self, Optional
from collections import deque


class Tile:
    def __init__(self, height: int):
        self.height: int = height
        self.neighbors: list[Self] = []
        self.distance: Optional[int] = None


def read_map(input_file: str) -> Tile:
    lines = input_file.splitlines()
    map_height = len(lines)
    map_width = len(lines[0])

    grid: list[list[Tile]] = []
    end = None

    # read tile heights
    for i in range(map_height):
        row = []
        for j in range(map_width):
            v = lines[i][j]
            t = Tile(char_height(v))
            row.append(t)
            if v == "E":
                end = t
        grid.append(row)

    # read tile neighbors
    for i in range(map_height):
        for j in range(map_width):
            t = grid[i][j]
            if i > 0 and grid[i - 1][j].height + 1 >= t.height:
                t.neighbors.append(grid[i - 1][j])
            if i + 1 < map_height and grid[i + 1][j].height + 1 >= t.height:
                t.neighbors.append(grid[i + 1][j])
            if j > 0 and grid[i][j - 1].height + 1 >= t.height:
                t.neighbors.append(grid[i][j - 1])
            if j + 1 < map_width and grid[i][j + 1].height + 1 >= t.height:
                t.neighbors.append(grid[i][j + 1])

    if end is None:
        raise ValueError("The map does not contain an end location")

    return end


def char_height(c: str) -> int:
    if c == "S":
        return 0
    if c == "E":
        return 25
    return ord(c) - ord("a")


def bfs(end: Tile) -> int:
    queue = deque([end])
    end.distance = 0

    while len(queue) > 0:
        tile = queue.popleft()

        if tile.height == 0:
            return tile.distance

        for next_tile in tile.neighbors:
            if next_tile.distance is None:
                next_tile.distance = tile.distance + 1
                queue.append(next_tile)

    raise ValueError("Unable to find the end tile")


def result(input_file: str) -> int:
    end = read_map(input_file)
    distance = bfs(end)
    return distance


def solve(input_file: str) -> str:
    return f"{result(input_file)} steps are required"
