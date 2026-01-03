from dataclasses import dataclass
from typing import Callable
from collections import deque

Operation = Callable[[int], int]


@dataclass
class InspectionResult:
    worry_level: int
    target: int


class Monkey:
    def __init__(
        self,
        items: list[int],
        operation: Operation,
        divisor: int,
        pass_target: int,
        fail_target: int,
    ):
        self.inspected_count = 0
        self._items = deque(items)
        self._operation = operation
        self._divisor = divisor
        self._pass_target = pass_target
        self._fail_target = fail_target

    def give_item(self, item: int) -> None:
        self._items.append(item)

    def inspect_item(self) -> InspectionResult:
        worry_level = self._operation(self._items.popleft()) // 3
        is_pass = worry_level % self._divisor == 0
        target = self._pass_target if is_pass else self._fail_target

        self.inspected_count += 1

        return InspectionResult(worry_level, target)

    def has_items(self) -> bool:
        return len(self._items) > 0


def read_monkeys(input_file: str) -> list[Monkey]:
    lines = input_file.splitlines()
    monkeys = []

    for i in range(0, len(lines), 7):
        monkeys.append(
            Monkey(
                map(int, lines[i + 1][18:].split(", ")),
                read_operation(lines[i + 2]),
                int(lines[i + 3][21:]),
                int(lines[i + 4][29:]),
                int(lines[i + 5][30:]),
            )
        )

    return monkeys


def read_operation(line: str) -> Callable[[int], int]:
    operator, value = line[23:].split(maxsplit=1)

    if operator == "+":
        if value == "old":
            return lambda x: x + x
        return lambda x: x + int(value)

    if operator == "*":
        if value == "old":
            return lambda x: x * x
        return lambda x: x * int(value)

    raise ValueError(f"{operator} is not a valid operation")


def monkey_business(monkeys: list[Monkey]) -> int:
    counts = [m.inspected_count for m in monkeys]
    counts.sort(reverse=True)
    return counts[0] * counts[1]


def result(input_file: str) -> int:
    monkeys = read_monkeys(input_file)

    for _round in range(20):
        for m in monkeys:
            while m.has_items():
                r = m.inspect_item()
                monkeys[r.target].give_item(r.worry_level)

    return monkey_business(monkeys)


def solve(input_file: str) -> str:
    return f"The level of monkey business after 20 rounds of stuff-slinging simian shenanigans is: {result(input_file)}"
