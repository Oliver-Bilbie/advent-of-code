from typing import TypeAlias

Position: TypeAlias = tuple[int, int]


class Blizzard:
    def __init__(self, position: Position, direction: str):
        self.position = position
        match direction:
            case "v":
                self._move_by = (1, 0)
            case "^":
                self._move_by = (-1, 0)
            case ">":
                self._move_by = (0, 1)
            case "<":
                self._move_by = (0, -1)
            case _:
                raise ValueError(f"{direction} is not a valid direction")

    def move(self, valley_size: tuple[int, int]) -> None:
        next_row = self.position[0] + self._move_by[0]
        if next_row >= valley_size[0]:
            next_row = 1
        elif next_row <= 0:
            next_row = valley_size[0] - 1

        next_col = self.position[1] + self._move_by[1]
        if next_col >= valley_size[1]:
            next_col = 1
        elif next_col <= 0:
            next_col = valley_size[1] - 1

        self.position = (next_row, next_col)


def read_valley_size(lines: list[str]) -> tuple[int, int]:
    return (len(lines) - 1, len(lines[0]) - 1)


def read_start_position(lines: list[str]) -> Position:
    return (0, lines[0].index("."))


def read_end_position(lines: list[str]) -> Position:
    return (len(lines) - 1, lines[-1].index("."))


def read_blizzards(lines: list[str]) -> list[Blizzard]:
    blizzard_chars = ["^", "v", ">", "<"]
    blizzards = []

    for row, line in enumerate(lines):
        for col, item in enumerate(line):
            if item in blizzard_chars:
                blizzards.append(Blizzard((row, col), item))

    return blizzards


def result(input_file: str) -> int:
    lines = input_file.splitlines()
    valley_size = read_valley_size(lines)
    start_position = read_start_position(lines)
    end_position = read_end_position(lines)
    blizzards = read_blizzards(lines)

    minutes_elapsed = 0

    for first_pos, last_pos in [
        (start_position, end_position),
        (end_position, start_position),
        (start_position, end_position),
    ]:
        possible_positions: set[Position] = set([first_pos])

        while last_pos not in possible_positions:
            next_positions: set[Position] = set()

            for b in blizzards:
                b.move(valley_size)

            for current_pos in possible_positions:
                for move in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)]:
                    next_pos = (current_pos[0] + move[0], current_pos[1] + move[1])

                    is_in_bounds = (
                        (
                            1 <= next_pos[0] < valley_size[0]
                            and 1 <= next_pos[1] < valley_size[1]
                        )
                        or next_pos == start_position
                        or next_pos == end_position
                    )
                    if is_in_bounds:
                        next_positions.add(next_pos)

            for b in blizzards:
                next_positions.discard(b.position)

            possible_positions = next_positions
            minutes_elapsed += 1

    return minutes_elapsed


def solve(input_file: str) -> str:
    return f"The fewest number of minutes required to reach the goal, go back to the start, then reach the goal again is {result(input_file)}"
