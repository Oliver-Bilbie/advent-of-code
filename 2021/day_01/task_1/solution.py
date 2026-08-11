def result(input_file: str) -> int:
    depths = [int(line) for line in input_file.splitlines()]
    result = 0

    for i in range(1, len(depths)):
        if depths[i] > depths[i - 1]:
            result += 1

    return result


def solve(input_file: str) -> str:
    return f"{result(input_file)} measurements are larger than the previous measurement"
