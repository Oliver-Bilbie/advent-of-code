from dataclasses import dataclass
from typing import Self

MAX_XY = 4000000


@dataclass(frozen=True)
class Sensor:
    position: tuple[int, int]
    radius: int


@dataclass(frozen=True)
class Range:
    first: int
    last: int

    def merge(self, other: Self) -> list[Self]:
        if self.first > other.first:
            return other.merge(self)

        if self.last >= other.last:
            return [self]

        if self.last >= other.first:
            return [Range(self.first, other.last)]

        return [self, other]


def read_nth_position(line: str, n: int) -> tuple[int, int]:
    """Returns the nth listed position in a given input line."""
    x = int(line.split("x=", maxsplit=n)[n].split(",", maxsplit=1)[0])
    y = int(line.split("y=", maxsplit=n)[n].split(":", maxsplit=1)[0])
    return (x, y)


def read_sensors(input_file: str) -> list[Sensor]:
    sensors = []

    for line in input_file.splitlines():
        position = read_nth_position(line, 1)
        beacon = read_nth_position(line, 2)
        radius = manhattan_distance(position, beacon)
        sensors.append(Sensor(position, radius))

    return sensors


def manhattan_distance(p1: tuple[int, int], p2: tuple[int, int]) -> int:
    dx = p1[0] - p2[0] if p1[0] >= p2[0] else p2[0] - p1[0]
    dy = p1[1] - p2[1] if p1[1] >= p2[1] else p2[1] - p1[1]
    return dx + dy


def merge_ranges(ranges: list[Range]) -> None:
    ranges.sort(key=lambda r: r.first)

    i = 0
    while i + 1 < len(ranges):
        merged = ranges[i].merge(ranges[i + 1])
        if len(merged) == 1:
            ranges[i] = merged[0]
            ranges.pop(i + 1)
        else:
            i += 1


def find_possible_rows(sensors: list[Sensor], max_y: int) -> list[int]:
    """
    Find y values adjacent to intersections of search boundaries
    """
    pos_diagonals = []
    neg_diagonals = []

    for s in sensors:
        sx, sy = s.position
        r = s.radius + 1

        pos_diagonals.append(sx + sy + r)
        pos_diagonals.append(sx + sy - r)

        neg_diagonals.append(sx - sy + r)
        neg_diagonals.append(sx - sy - r)

    candidate_rows = set()

    for a in pos_diagonals:
        for b in neg_diagonals:
            diff = a - b
            if diff % 2 != 0:
                continue

            y = diff // 2
            if 0 <= y <= max_y:
                candidate_rows.add(y)

    return list(candidate_rows)


def result(input_file: str, max_y: int) -> int:
    sensors = read_sensors(input_file)

    for y in find_possible_rows(sensors, max_y):
        scanned_in_row: list[Range] = []

        for s in sensors:
            dy = abs(y - s.position[1])
            x_offset = s.radius - dy
            if x_offset > 0:
                x_range = Range(
                    max(0, s.position[0] - x_offset),
                    min(MAX_XY, s.position[0] + x_offset),
                )
                scanned_in_row.append(x_range)

        merge_ranges(scanned_in_row)
        if len(scanned_in_row) == 2:
            x = scanned_in_row[0].last + 1
            return x * MAX_XY + y

    raise ValueError("The input does not contain a distress beacon")


def solve(input_file: str) -> str:
    return f"The tuning frequency is {result(input_file, MAX_XY)}"
