def result(input_file: str) -> int:
    depths = [int(line) for line in input_file.splitlines()]
    sums = [depths[i] + depths[i + 1] + depths[i + 2] for i in range(len(depths) - 2)]
    result = 0

    for i in range(1, len(sums)):
        if sums[i] > sums[i - 1]:
            result += 1

    return result


def solve(input_file: str) -> str:
    return f"{result(input_file)} sums are larger than the previous sum"
