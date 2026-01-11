from typing import Self
from dataclasses import dataclass
from enum import Enum


class Operator(Enum):
    Add = 0
    Subtract = 1
    Multiply = 2
    Divide = 3

    @classmethod
    def from_char(cls, char: str) -> Self:
        match char:
            case "+":
                return cls.Add
            case "-":
                return cls.Subtract
            case "*":
                return cls.Multiply
            case "/":
                return cls.Divide
            case _:
                raise ValueError(f"{char} is not a valid operator")

    def apply(self, left: int, right: int) -> int:
        match self:
            case self.Add:
                return left + right
            case self.Subtract:
                return left - right
            case self.Multiply:
                return left * right
            case self.Divide:
                assert left % right == 0
                return left // right


@dataclass(frozen=True)
class Operation:
    left: str
    right: str
    operator: Operator


def read_input(input_file: str) -> tuple[dict[str, int], dict[str, Operation]]:
    results: dict[str, int] = {}
    operations: dict[str, Operation] = {}

    for line in input_file.splitlines():
        items = line.split()
        monkey = items[0][:4]

        if len(items) == 2:
            results[monkey] = int(items[1])

        else:
            operations[monkey] = Operation(
                items[1], items[3], Operator.from_char(items[2])
            )

    return (results, operations)


def result(input_file: str) -> int:
    results, operations = read_input(input_file)

    while results.get("root") is None:
        next_operations: dict[str, Operation] = {}

        for monkey, o in operations.items():
            left_val = results.get(o.left)
            if left_val is None:
                next_operations[monkey] = o
                continue

            right_val = results.get(o.right)
            if right_val is None:
                next_operations[monkey] = o
                continue

            results[monkey] = o.operator.apply(left_val, right_val)

        operations = next_operations

    return results["root"]


def solve(input_file: str) -> str:
    return f'The monkey named Root yells "{result(input_file)}"!'
