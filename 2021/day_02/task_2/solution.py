def result(input_file: str) -> int:
    position = 0
    depth = 0
    aim = 0

    for line in input_file.splitlines():
        direction, value = line.split(" ", maxsplit=1)
        match direction:
            case "forward":
                x = int(value)
                position += x
                depth += aim * x
            case "down":
                aim += int(value)
            case "up":
                aim -= int(value)
            case _:
                raise ValueError(f"{direction} is not a valid direction")

    return position * depth


def solve(input_file: str) -> str:
    return f"The result is: {result(input_file)}"
