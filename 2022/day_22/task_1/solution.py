from typing import Self, Optional
from enum import IntEnum


class Facing(IntEnum):
    RIGHT = 0
    DOWN = 1
    LEFT = 2
    UP = 3

    def turn(self, clockwise: bool) -> Self:
        return Facing((self + 1 if clockwise else self - 1) % 4)


class Tile:
    def __init__(self, row: int, column: int):
        self.row = row
        self.column = column
        self._adjacent: tuple[
            Optional[Self], Optional[Self], Optional[Self], Optional[Self]
        ] = (None, None, None, None)

    def set_adjacent(self, direction: Facing, adj_tile: Optional[Self]) -> None:
        adj_list = list(self._adjacent)
        adj_list[direction] = adj_tile
        assert len(adj_list) == 4
        self._adjacent = tuple(adj_list)

    def move(self, direction: Facing) -> Optional[Self]:
        return self._adjacent[direction]


def read_map(lines: list[str]) -> Tile:
    start_tile: Optional[Tile] = None
    tiles: dict[tuple[int, int], Optional[Tile]] = {}
    max_row = len(lines)
    max_col = len(lines[0])

    # Initialize tiles
    for row, line in enumerate(lines):
        for col, item in enumerate(line):
            match item:
                case ".":
                    tile = Tile(row, col)
                    tiles[(row, col)] = tile
                    if start_tile is None:
                        start_tile = tile
                case "#":
                    tiles[(row, col)] = None
                case " ":
                    continue
                case _:
                    raise ValueError(f"{item} is not a valid tile type")

    # Populate adjacency values
    for tile in tiles.values():
        if tile is None:
            continue

        right_pos = (tile.row, tile.column + 1)
        if right_pos not in tiles:
            col = 0
            while (tile.row, col) not in tiles:
                col += 1
            right_pos = (tile.row, col)
        tile.set_adjacent(Facing.RIGHT, tiles[right_pos])

        down_pos = (tile.row + 1, tile.column)
        if down_pos not in tiles:
            row = 0
            while (row, tile.column) not in tiles:
                row += 1
            down_pos = (row, tile.column)
        tile.set_adjacent(Facing.DOWN, tiles[down_pos])

        left_pos = (tile.row, tile.column - 1)
        if left_pos not in tiles:
            col = max_col - 1
            while (tile.row, col) not in tiles:
                col -= 1
            left_pos = (tile.row, col)
        tile.set_adjacent(Facing.LEFT, tiles[left_pos])

        up_pos = (tile.row - 1, tile.column)
        if up_pos not in tiles:
            row = max_row - 1
            while (row, tile.column) not in tiles:
                row -= 1
            up_pos = (row, tile.column)
        tile.set_adjacent(Facing.UP, tiles[up_pos])

    if start_tile is None:
        raise ValueError("No tiles were found in the input")
    return start_tile


def read_directions(line: str) -> tuple[list[int], list[bool]]:
    moves: list[int] = []
    turns: list[bool] = []
    start = 0
    end = 0

    while start < len(line):
        while end < len(line) and line[end] not in ["R", "L"]:
            end += 1

        moves.append(int(line[start:end]))
        if end < len(line):
            turns.append(line[end] == "R")

        start = end + 1
        end = start + 1

    return moves, turns


def result(input_file: str) -> int:
    lines = input_file.splitlines()
    tile = read_map(lines[:-2])
    facing = Facing.RIGHT
    moves, turns = read_directions(lines[-1])

    for i in range(len(turns)):
        for _ in range(moves[i]):
            next_tile = tile.move(facing)
            if next_tile is not None:
                tile = next_tile
            else:
                continue
        facing = facing.turn(turns[i])

    for _ in range(moves[-1]):
        next_tile = tile.move(facing)
        if next_tile is not None:
            tile = next_tile
        else:
            continue

    return 1000 * (tile.row + 1) + 4 * (tile.column + 1) + facing


def solve(input_file: str) -> str:
    return f"The final password is: {result(input_file)}"
