from typing import Optional
from dataclasses import dataclass


class Stack:
    def __init__(self) -> None:
        self._values: list[str] = []

    def push(self, value: str) -> None:
        self._values.append(value)

    def pop(self) -> Optional[str]:
        if not self.is_empty():
            return self._values.pop()

    def peek(self) -> Optional[str]:
        if not self.is_empty():
            return self._values[-1]

    def is_empty(self) -> bool:
        return len(self._values) == 0


@dataclass
class Instruction:
    quantity: int
    from_stack: int
    to_stack: int


def read_stacks(lines: list[str]) -> list[Stack]:
    stack_count = max(map(int, lines[-1].split()))
    stacks = [Stack() for _ in range(stack_count)]

    for line in reversed(lines[:-1]):
        for i in range(stack_count):
            crate = line[1 + 4 * i]
            if crate != " ":
                stacks[i].push(crate)

    return stacks


def read_instructions(lines: list[str]) -> list[Instruction]:
    instructions = []

    for line in lines:
        items = line.split()
        quantity = int(items[1])
        from_stack = int(items[3]) - 1
        to_stack = int(items[5]) - 1
        instructions.append(Instruction(quantity, from_stack, to_stack))

    return instructions


def result(input_file: str) -> str:
    lines = input_file.splitlines()
    empty_line = 0

    while lines[empty_line] != "":
        empty_line += 1

    stacks = read_stacks(lines[:empty_line])
    instructions = read_instructions(lines[empty_line + 1 :])

    for i in instructions:
        for _ in range(i.quantity):
            crate = stacks[i.from_stack].pop()
            stacks[i.to_stack].push(crate)

    top_crates = "".join(s.peek() for s in stacks)

    return top_crates


def solve(input_file: str) -> str:
    return f"The top crates are: {result(input_file)}"
