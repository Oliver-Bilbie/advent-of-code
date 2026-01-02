LOWERCASE_A = ord("a")
UPPERCASE_A = ord("A")


def solve(input_file: str) -> str:
    return f"The sum of priorities of badges is: {result(input_file)}"


def result(input_file: str) -> int:
    lines = input_file.splitlines()
    total = 0

    for i in range(0, len(lines), 3):
        backpack_1 = set(lines[i])
        backpack_2 = set(lines[i + 1])
        backpack_3 = set(lines[i + 2])

        (badge,) = backpack_1 & backpack_2 & backpack_3
        total += get_priority(badge)

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
