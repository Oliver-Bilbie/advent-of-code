def result(input_file: str) -> int:
    sum_of_strengths = 0
    cycle = 1
    x_register = 1

    for line in input_file.splitlines():
        if (cycle - 20) % 40 == 0:
            sum_of_strengths += cycle * x_register
        cycle += 1

        if line.startswith("addx"):
            if (cycle - 20) % 40 == 0:
                sum_of_strengths += cycle * x_register
            cycle += 1
            x_register += int(line[5:])

    return sum_of_strengths


def solve(input_file: str) -> str:
    return f"The sum of signal stengths is: {result(input_file)}"
