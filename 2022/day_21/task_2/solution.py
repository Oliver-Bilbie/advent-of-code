from typing import Self
from dataclasses import dataclass
from enum import Enum


class Operator(Enum):
    Add = 0
    Subtract = 1
    Multiply = 2
    Divide = 3
    Equal = 4

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

    def reverse(self, op_result: int, op_input: int, input_is_left: bool) -> int:
        match self:
            case self.Add:
                return op_result - op_input
            case self.Subtract:
                return op_input - op_result if input_is_left else op_input + op_result
            case self.Multiply:
                return op_result // op_input
            case self.Divide:
                if input_is_left:
                    assert op_input % op_result == 0
                    return op_input // op_result
                return op_input * op_result
            case self.Equal:
                return op_input


@dataclass(frozen=True)
class Operation:
    left: str
    right: str
    operator: Operator


@dataclass(frozen=True)
class HumnOperation:
    humn_is_on_right: bool
    humn_value_name: str
    other_value: int
    operator: Operator


def read_input(input_file: str) -> tuple[dict[str, int], dict[str, Operation]]:
    results: dict[str, int] = {}
    operations: dict[str, Operation] = {}

    for line in input_file.splitlines():
        items = line.split()
        monkey = items[0][:4]

        if monkey == "root":
            operations[monkey] = Operation(items[1], items[3], Operator.Equal)

        elif monkey == "humn":
            continue

        elif len(items) == 2:
            results[monkey] = int(items[1])

        else:
            operations[monkey] = Operation(
                items[1], items[3], Operator.from_char(items[2])
            )

    return (results, operations)


def result(input_file: str) -> int:
    results, operations = read_input(input_file)
    humn_results: dict[str, HumnOperation] = {
        "humn": HumnOperation(True, "", 0, Operator.Add)
    }

    while humn_results.get("root") is None:
        next_operations: dict[str, Operation] = {}

        for monkey, o in operations.items():
            left_val = results.get(o.left)
            right_val = results.get(o.right)

            if left_val is not None and right_val is not None:
                results[monkey] = o.operator.apply(left_val, right_val)
                continue

            if left_val is not None:
                right_humn_val = humn_results.get(o.right)
                if right_humn_val is not None:
                    humn_results[monkey] = HumnOperation(
                        True, o.right, left_val, o.operator
                    )
                    continue

            if right_val is not None:
                left_humn_val = humn_results.get(o.left)
                if left_humn_val is not None:
                    humn_results[monkey] = HumnOperation(
                        False, o.left, right_val, o.operator
                    )
                    continue

            next_operations[monkey] = o

        operations = next_operations

    monkey = "root"
    current_operation = humn_results[monkey]
    current_input = 0
    while monkey != "humn":
        current_input = current_operation.operator.reverse(
            current_input,
            current_operation.other_value,
            current_operation.humn_is_on_right,
        )
        monkey = current_operation.humn_value_name
        current_operation = humn_results[monkey]

    return current_input


def solve(input_file: str) -> str:
    return f'You yell "{result(input_file)}"! and pass Root\'s equality test'
