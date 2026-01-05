from dataclasses import dataclass
from typing import Self


@dataclass(frozen=True)
class Sensor:
    position: tuple[int, int]
    beacon: tuple[int, int]


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
    return [
        Sensor(
            position=read_nth_position(line, 1),
            beacon=read_nth_position(line, 2),
        )
        for line in input_file.splitlines()
    ]


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


def count_scanned(ranges: list[Range]) -> int:
    return sum(1 + r.last - r.first for r in ranges)


def result(input_file: str, y: int) -> int:
    sensors = read_sensors(input_file)
    scanned_in_row: list[Range] = []
    beacons_in_row: set[tuple[int, int]] = set()

    for s in sensors:
        if s.beacon[1] == y:
            beacons_in_row.add(s.beacon)

        scan_radius = manhattan_distance(s.position, s.beacon)
        dy = abs(y - s.position[1])
        x_offset = scan_radius - dy
        if x_offset >= 0:
            x_range = Range(s.position[0] - x_offset, s.position[0] + x_offset)
            scanned_in_row.append(x_range)

    merge_ranges(scanned_in_row)
    return count_scanned(scanned_in_row) - len(beacons_in_row)


def solve(input_file: str) -> str:
    return f"{result(input_file, 2000000)} positions cannot contain a beacon"
