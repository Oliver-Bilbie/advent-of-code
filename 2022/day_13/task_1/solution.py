from enum import Enum
from typing import Self, TypeAlias

Packet: TypeAlias = int | list["Packet"]


class OrderStatus(Enum):
    UNDETERMINED = 0
    CORRECT = 1
    INCORRECT = 2

    @classmethod
    def from_comparison(cls, left: int, right: int) -> Self:
        if left < right:
            return cls.CORRECT
        if left > right:
            return cls.INCORRECT
        return cls.UNDETERMINED


def read_list(list_str: str) -> Packet:
    if list_str == "[]":
        return []

    list_data: list[Packet] = []
    read_depth = 0
    read_start = 1

    for i in range(1, len(list_str) - 1):
        c = list_str[i]

        if c == "," and read_depth == 0:
            item = list_str[read_start:i]
            read_start = i + 1
            if item.startswith("["):
                list_data.append(read_list(item))
            else:
                list_data.append(int(item))

        elif c == "[":
            read_depth += 1

        elif c == "]":
            read_depth -= 1

    item = list_str[read_start:-1]
    if item.startswith("["):
        list_data.append(read_list(item))
    else:
        list_data.append(int(item))

    return list_data


def read_pairs(input_file: str) -> list[tuple[Packet, Packet]]:
    lines = input_file.splitlines()
    pairs = []

    for i in range(0, len(lines), 3):
        p = (read_list(lines[i]), read_list(lines[i + 1]))
        pairs.append(p)

    return pairs


def in_right_order(left: Packet, right: Packet) -> OrderStatus:
    if isinstance(left, int) and isinstance(right, int):
        return OrderStatus.from_comparison(left, right)

    if isinstance(left, int):
        return in_right_order([left], right)

    if isinstance(right, int):
        return in_right_order(left, [right])

    for i in range(min(len(left), len(right))):
        status = in_right_order(left[i], right[i])
        if status != OrderStatus.UNDETERMINED:
            return status

    return OrderStatus.from_comparison(len(left), len(right))


def result(input_file: str) -> int:
    pairs = read_pairs(input_file)
    sum_of_indices = 0

    for i, p in enumerate(pairs):
        if in_right_order(*p) == OrderStatus.CORRECT:
            sum_of_indices += i + 1

    return sum_of_indices


def solve(input_file: str) -> str:
    return f"The sum of indices is: {result(input_file)}"
