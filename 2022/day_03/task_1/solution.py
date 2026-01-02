LOWERCASE_A = ord("a")
UPPERCASE_A = ord("A")


def solve(input_file: str) -> str:
    return f"The sum of priorities of duplicate items is: {result(input_file)}"


def result(input_file: str) -> int:
    total = 0

    for backpack in input_file.splitlines():
        first_set = set(backpack[: len(backpack) // 2])
        second_set = set(backpack[len(backpack) // 2 :])

        for item in first_set:
            if item in second_set:
                total += get_priority(item)

    return total


def get_priority(item: str) -> int:
    if len(item) != 1:
        raise ValueError(f"{item} does not contain exactly one character")

    value = ord(item)

    if LOWERCASE_A <= value < LOWERCASE_A + 26:
        return 1 + value - LOWERCASE_A

    if UPPERCASE_A <= value < UPPERCASE_A + 26:
        return 27 + value - UPPERCASE_A

    raise ValueError(f"{item} is not in the range a-z or A-Z")
