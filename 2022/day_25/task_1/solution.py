from typing import Self, Optional


class SnafuNumber:

    def __init__(self, snafu_str: Optional[str] = None):
        self._value = 0

        if snafu_str is None:
            return

        column_value = 1
        for c in reversed(snafu_str):
            if c == "-":
                self._value -= column_value
            elif c == "=":
                self._value -= 2 * column_value
            elif c in ["0", "1", "2"]:
                self._value += int(c) * column_value
            else:
                raise ValueError(f"{c} is not a valid SNAFU digit")
            column_value *= 5

    def __add__(self, other: Self) -> Self:
        sum_value = SnafuNumber()
        sum_value._value = self._value + other._value
        return sum_value

    def __radd__(self, other):
        if other == 0:
            return self
        return self.__add__(other)

    @property
    def value(self) -> str:
        rtl_values: list[str] = []
        remaining = self._value
        carry = 0

        while remaining > 0 or carry > 0:
            base5_digit = (remaining % 5) + carry

            if base5_digit <= 2:
                rtl_values.append(str(base5_digit))
                carry = 0
            elif base5_digit == 3:
                rtl_values.append("=")
                carry = 1
            elif base5_digit == 4:
                rtl_values.append("-")
                carry = 1
            elif base5_digit == 5:
                rtl_values.append("0")
                carry = 1

            remaining //= 5

        return "".join(reversed(rtl_values))


def result(input_file: str) -> str:
    total_fuel = sum(SnafuNumber(l) for l in input_file.splitlines())
    return total_fuel.value


def solve(input_file: str) -> str:
    return f"You must supply the SNAFU number {result(input_file)} to Bob's console"
