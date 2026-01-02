from dataclasses import dataclass
from typing import Self


@dataclass(frozen=True)
class IdRange:
    start: int
    end: int

    def contains(self, other: Self) -> bool:
        return self.start <= other.start and self.end >= other.end

    @classmethod
    def from_str(cls, s: str) -> Self:
        start, end = map(int, s.split("-", 1))
        return cls(start, end)


def result(input_file: str) -> int:
    count = 0

    for line in input_file.splitlines():
        first, second = map(IdRange.from_str, line.split(",", 1))
        if first.contains(second) or second.contains(first):
            count += 1

    return count


def solve(input_file: str) -> str:
    return f"{result(input_file)} Elves' ranges are eclipsed by their partner"
