from typing import TypeAlias, Self
from enum import Enum


Position: TypeAlias = tuple[int, int]


class Direction(Enum):
    NORTH = 0
    SOUTH = 1
    WEST = 2
    EAST = 3

    def move(self, position: Position) -> Position:
        match self:
            case Direction.NORTH:
                return (position[0] - 1, position[1])
            case Direction.SOUTH:
                return (position[0] + 1, position[1])
            case Direction.WEST:
                return (position[0], position[1] - 1)
            case Direction.EAST:
                return (position[0], position[1] + 1)


def read_starting_positions(input_file: str) -> dict[int, Position]:
    positions = {}
    elf_id = 0

    for row, line in enumerate(input_file.splitlines()):
        for col, item in enumerate(line):
            if item == "#":
                positions[elf_id] = (row, col)
                elf_id += 1

    return positions


def none_adjacent(position: Position, occupied: set[Position]) -> bool:
    adjacent_spaces = [
        (position[0] - 1, position[1] - 1),
        (position[0] - 1, position[1]),
        (position[0] - 1, position[1] + 1),
        (position[0], position[1] - 1),
        (position[0], position[1] + 1),
        (position[0] + 1, position[1] - 1),
        (position[0] + 1, position[1]),
        (position[0] + 1, position[1] + 1),
    ]
    return not any(p in occupied for p in adjacent_spaces)


def direction_is_clear(
    position: Position, direction: Direction, occupied: set[Position]
) -> bool:
    to_search: set[Position] = set()
    match direction:
        case Direction.NORTH:
            to_search = [
                (position[0] - 1, position[1] - 1),
                (position[0] - 1, position[1]),
                (position[0] - 1, position[1] + 1),
            ]
        case Direction.SOUTH:
            to_search = [
                (position[0] + 1, position[1] - 1),
                (position[0] + 1, position[1]),
                (position[0] + 1, position[1] + 1),
            ]
        case Direction.WEST:
            to_search = [
                (position[0] - 1, position[1] - 1),
                (position[0], position[1] - 1),
                (position[0] + 1, position[1] - 1),
            ]
        case Direction.EAST:
            to_search = [
                (position[0] - 1, position[1] + 1),
                (position[0], position[1] + 1),
                (position[0] + 1, position[1] + 1),
            ]
    return not any(p in occupied for p in to_search)


def count_tiles(positions: dict[int, Position]) -> int:
    if len(positions) == 0:
        return 0

    max_row = None
    min_row = None
    max_col = None
    min_col = None

    for pos in positions.values():
        if max_row is None or pos[0] > max_row:
            max_row = pos[0]
        if min_row is None or pos[0] < min_row:
            min_row = pos[0]
        if max_col is None or pos[1] > max_col:
            max_col = pos[1]
        if min_col is None or pos[1] < min_col:
            min_col = pos[1]

    rectangle_area = (1 + max_row - min_row) * (1 + max_col - min_col)
    return rectangle_area - len(positions)


def result(input_file: str) -> int:
    positions = read_starting_positions(input_file)
    moves: dict[Position, list[int]] = {}
    directions: list[Direction] = [
        Direction.NORTH,
        Direction.SOUTH,
        Direction.WEST,
        Direction.EAST,
    ]

    for _ in range(10):
        moves.clear()
        occupied: set[Position] = set(positions.values())

        for elf, pos in positions.items():
            if none_adjacent(pos, occupied):
                continue

            for d in directions:
                if direction_is_clear(pos, d, occupied):
                    next_pos = d.move(pos)
                    if next_pos in moves:
                        moves[next_pos].append(elf)
                    else:
                        moves[next_pos] = [elf]
                    break

        for pos, elves in moves.items():
            if len(elves) == 1:
                elf = elves[0]
                positions[elf] = pos

        directions = directions[1:] + directions[:1]

    return count_tiles(positions)


def solve(input_file: str) -> str:
    return f"There are {result(input_file)} empty ground tiles after 10 rounds"
