DISPLAY_WIDTH = 40


def draw(image: str, cycle: int, x_register: int) -> str:
    col = cycle % DISPLAY_WIDTH

    if x_register - 1 <= col <= x_register + 1:
        image += "#"
    else:
        image += "."

    if col == DISPLAY_WIDTH - 1:
        image += "\n"

    return image


def solve(input_file: str) -> str:
    image = ""
    cycle = 0
    x_register = 1

    for line in input_file.splitlines():
        image = draw(image, cycle, x_register)
        cycle += 1

        if line.startswith("addx"):
            image = draw(image, cycle, x_register)
            cycle += 1

            x_register += int(line[5:])

    return image
