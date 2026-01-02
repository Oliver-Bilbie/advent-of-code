from dataclasses import dataclass
from typing import Self


@dataclass(frozen=True)
class IdRange:
    start: int
    end: int

    def overlaps_with(self, other: Self) -> bool:
        if self.start > other.start:
            return other.overlaps_with(self)
        return self.end >= other.start

    @classmethod
    def from_str(cls, s: str) -> Self:
        start, end = map(int, s.split("-", 1))
        return cls(start, end)


def result(input_file: str) -> int:
    count = 0

    for line in input_file.splitlines():
        first, second = map(IdRange.from_str, line.split(",", 1))
        if first.overlaps_with(second):
            count += 1

    return count


def solve(input_file: str) -> str:
    return f"{result(input_file)} Elves' ranges overlap with their partner"
