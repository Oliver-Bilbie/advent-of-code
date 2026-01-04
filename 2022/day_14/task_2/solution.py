from typing import TypeAlias

Position: TypeAlias = tuple[int, int]


def read_rock_positions(input_file: str) -> tuple[set[Position], int]:
    positions: set[Position] = set()
    max_y = 0

    for line in input_file.splitlines():
        items = line.split(" -> ")
        for i in range(len(items) - 1):
            x1, y1 = map(int, items[i].split(",", maxsplit=1))
            x2, y2 = map(int, items[i + 1].split(",", maxsplit=1))

            x_range = (x1, x2 + 1) if x1 <= x2 else (x2, x1 + 1)
            y_range = (y1, y2 + 1) if y1 <= y2 else (y2, y1 + 1)

            if y_range[1] > max_y:
                max_y = y_range[1]

            for x in range(*x_range):
                for y in range(*y_range):
                    positions.add((x, y))

    return positions, max_y


def add_sand(occupied: set[Position], max_y: int) -> bool:
    """
    Add sand from the point (500, 0).
    Returns True if the sand comes to rest, otherwise returns False.
    """

    sand_position: Position = (500, 0)

    if sand_position in occupied:
        return False  # The starting position is blocked

    while sand_position[1] < max_y:
        next_position = None
        for offset in [(0, 1), (-1, 1), (1, 1)]:
            x, y = sand_position
            dx, dy = offset
            try_position = (x + dx, y + dy)
            if try_position not in occupied:
                next_position = try_position
                break

        if next_position is None:
            occupied.add(sand_position)
            return True

        sand_position = next_position

    occupied.add(sand_position)
    return True


def result(input_file: str) -> int:
    occupied, max_y = read_rock_positions(input_file)

    sand_count = 0
    while add_sand(occupied, max_y):
        sand_count += 1

    return sand_count


def solve(input_file: str) -> str:
    return f"The result is: {result(input_file)}"
