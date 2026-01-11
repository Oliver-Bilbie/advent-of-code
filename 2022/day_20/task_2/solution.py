from typing import TypeAlias

IdValuePair: TypeAlias = tuple[int, int]


class WrappingList[T]:
    def __init__(self):
        self._data: list[T] = []

    def __iter__(self):
        yield from self._data

    def size(self) -> int:
        return len(self._data)

    def push_back(self, item: T) -> None:
        self._data.append(item)

    def get_nth(self, n: int) -> T:
        return self._data[n % len(self._data)]

    def move_item(self, current_index: int, offset: int) -> None:
        moved_value = self._data[current_index]

        offset %= len(self._data) - 1
        target_index = (current_index + offset) % (len(self._data) - 1)

        if target_index > current_index:
            self._data[current_index:target_index] = self._data[
                current_index + 1 : target_index + 1
            ]
            self._data[target_index] = moved_value

        elif target_index < current_index:
            self._data[target_index + 1 : current_index + 1] = self._data[
                target_index:current_index
            ]
            self._data[target_index] = moved_value


def read_values(input_file: str, decryption_key: int) -> WrappingList[IdValuePair]:
    values = WrappingList()
    for i, num in enumerate([int(n) for n in input_file.splitlines()]):
        values.push_back((i, decryption_key * num))
    return values


def result(input_file: str) -> int:
    decryption_key = 811589153
    values = read_values(input_file, decryption_key)

    for _ in range(10):
        for move_id in range(values.size()):
            for idx, element in enumerate(values):
                if element[0] == move_id:
                    values.move_item(idx, element[1])
                    break

    for idx, element in enumerate(values):
        if element[1] == 0:
            return sum(values.get_nth(idx + n)[1] for n in [1000, 2000, 3000])

    raise ValueError("The input does not contain a zero value")


def solve(input_file: str) -> str:
    return f"The sum of the actual grove coordinates is: {result(input_file)}"
