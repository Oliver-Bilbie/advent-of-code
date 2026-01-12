from typing import Self, Optional
from enum import IntEnum


class Facing(IntEnum):
    RIGHT = 0
    DOWN = 1
    LEFT = 2
    UP = 3

    def turn(self, clockwise: bool) -> Self:
        return Facing((self + 1 if clockwise else self - 1) % 4)

    def reverse(self) -> Self:
        return Facing((self + 2) % 4)


class Tile:
    def __init__(self, row: int, column: int):
        self.row = row
        self.column = column
        self._adjacent: tuple[
            Optional[Self], Optional[Self], Optional[Self], Optional[Self]
        ] = (None, None, None, None)
        self._rotations: tuple[int, int, int, int] = (0, 0, 0, 0)

    def set_adjacent(self, direction: Facing, adj_tile: Optional[Self]) -> None:
        adj_list = list(self._adjacent)
        adj_list[direction] = adj_tile
        assert len(adj_list) == 4
        self._adjacent = tuple(adj_list)

    def set_rotation(self, direction: Facing, rotation: int) -> None:
        rotation_list = list(self._rotations)
        rotation_list[direction] = rotation
        assert len(rotation_list) == 4
        self._rotations = tuple(rotation_list)

    def move(self, direction: Facing) -> tuple[Optional[Self], int]:
        new_direction = Facing((direction + self._rotations[direction]) % 4)
        return self._adjacent[direction], new_direction


def wrapping_pairs(
    face_size: int,
) -> list[tuple[tuple[int, int], tuple[int, int], Facing, Facing]]:
    """
    Returns a list of tuples where:
        overflowing from position [0] in direction [2]
        leads to position [1] facing in direction [3]
    """
    pairs = []
    for offset in range(face_size):
        pairs.extend(
            [
                (
                    (face_size - 1, 2 * face_size + offset),
                    (face_size + offset, 2 * face_size - 1),
                    Facing.DOWN,
                    Facing.LEFT,
                ),
                (
                    (0, face_size + offset),
                    (3 * face_size + offset, 0),
                    Facing.UP,
                    Facing.RIGHT,
                ),
                (
                    (4 * face_size - 1, offset),
                    (0, 2 * face_size + offset),
                    Facing.DOWN,
                    Facing.DOWN,
                ),
                (
                    (3 * face_size - 1, face_size + offset),
                    (3 * face_size + offset, face_size - 1),
                    Facing.DOWN,
                    Facing.LEFT,
                ),
                (
                    (offset, 3 * face_size - 1),
                    (3 * face_size - 1 - offset, 2 * face_size - 1),
                    Facing.RIGHT,
                    Facing.LEFT,
                ),
                (
                    (offset, face_size),
                    (3 * face_size - 1 - offset, 0),
                    Facing.LEFT,
                    Facing.RIGHT,
                ),
                (
                    (face_size + offset, face_size),
                    (2 * face_size, offset),
                    Facing.LEFT,
                    Facing.DOWN,
                ),
            ]
        )
    return pairs


def read_map(lines: list[str], face_size: int) -> Tile:
    start_tile: Optional[Tile] = None
    tiles: dict[tuple[int, int], Optional[Tile]] = {}

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

    # Populate non-wrapping adjacency values
    for tile in tiles.values():
        if tile is None:
            continue

        right_pos = (tile.row, tile.column + 1)
        if right_pos in tiles:
            tile.set_adjacent(Facing.RIGHT, tiles[right_pos])

        down_pos = (tile.row + 1, tile.column)
        if down_pos in tiles:
            tile.set_adjacent(Facing.DOWN, tiles[down_pos])

        left_pos = (tile.row, tile.column - 1)
        if left_pos in tiles:
            tile.set_adjacent(Facing.LEFT, tiles[left_pos])

        up_pos = (tile.row - 1, tile.column)
        if up_pos in tiles:
            tile.set_adjacent(Facing.UP, tiles[up_pos])

    # Populate wrapping adjacency and rotation values
    for key_a, key_b, facing_a, facing_b in wrapping_pairs(face_size):
        tile_a = tiles[key_a]
        tile_b = tiles[key_b]
        if tile_a is not None and tile_b is not None:
            tile_a.set_adjacent(facing_a, tile_b)
            tile_a.set_rotation(facing_a, (facing_b - facing_a) % 4)
            tile_b.set_adjacent(facing_b.reverse(), tile_a)
            tile_b.set_rotation(facing_b.reverse(), (facing_a - facing_b) % 4)

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


def result(input_file: str, face_size: int) -> int:
    lines = input_file.splitlines()
    tile = read_map(lines[:-2], face_size)
    facing = Facing.RIGHT
    moves, turns = read_directions(lines[-1])

    for i in range(len(turns)):
        for _ in range(moves[i]):
            next_tile, facing = tile.move(facing)
            if next_tile is not None:
                tile = next_tile
            else:
                continue
        facing = facing.turn(turns[i])

    for _ in range(moves[-1]):
        next_tile, facing = tile.move(facing)
        if next_tile is not None:
            tile = next_tile
        else:
            continue

    return 1000 * (tile.row + 1) + 4 * (tile.column + 1) + facing


def solve(input_file: str) -> str:
    return f"The final final password is: {result(input_file, 50)}"
