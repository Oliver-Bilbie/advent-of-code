from dataclasses import dataclass
from typing import Self


@dataclass
class Position:
    x: int
    y: int

    def __add__(self, other: Self) -> Self:
        return Position(self.x + other.x, self.y + other.y)

    def __sub__(self, other: Self) -> Self:
        return Position(self.x - other.x, self.y - other.y)

    def __hash__(self) -> int:
        return 1000 * self.x + self.y

    @classmethod
    def from_direction(cls, direction: str) -> Self:
        match direction:
            case "R":
                return cls(1, 0)
            case "U":
                return cls(0, 1)
            case "L":
                return cls(-1, 0)
            case "D":
                return cls(0, -1)
            case _:
                raise ValueError(f"{direction} is not a valid direction")


def move_tail(head_position: Position, tail_position: Position) -> Position:
    difference = head_position - tail_position
    is_touching = abs(difference.x) <= 1 and abs(difference.y) <= 1

    if is_touching:
        return tail_position

    return tail_position + Position(normalize(difference.x), normalize(difference.y))


def normalize(value: int) -> int:
    if value > 0:
        return 1
    if value < 0:
        return -1
    return 0


def result(input_file: str) -> int:
    head_position = Position(0, 0)
    tail_position = Position(0, 0)
    visited = {tail_position}

    for line in input_file.splitlines():
        direction, steps = line.split(maxsplit=1)
        to_move = Position.from_direction(direction)
        for _ in range(int(steps)):
            head_position += to_move
            tail_position = move_tail(head_position, tail_position)
            visited.add(tail_position)

    return len(visited)


def solve(input_file: str) -> str:
    return f"The tail will visit {result(input_file)} positions"
