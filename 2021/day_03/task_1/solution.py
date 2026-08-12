def result(input_file: str) -> int:
    lines = input_file.splitlines()
    bit_counts = [0] * len(lines[0])
    on_threshold = len(lines) // 2

    for line in lines:
        for bit, value in enumerate(line):
            bit_counts[bit] += int(value)

    gamma = 0
    epsilon = 0

    for value in bit_counts:
        gamma *= 2
        epsilon *= 2
        if value > on_threshold:
            gamma += 1
        elif value < on_threshold:
            epsilon += 1

    return gamma * epsilon


def solve(input_file: str) -> str:
    return f"The power consumption of the submarine is {result(input_file)}"
