class Range:
    def __init__(self, line: str):
        s, e = line.split(" -> ", maxsplit=1)
        self.start = tuple(map(lambda v: int(v), s.split(",", maxsplit=1)))
        self.end = tuple(map(lambda v: int(v), e.split(",", maxsplit=1)))


def result(input_file: str) -> int:
    vents: dict[tuple[int, int], int] = {}

    for line in input_file.splitlines():
        rng = Range(line)

        if rng.start[0] == rng.end[0]:
            x = rng.start[0]
            y_min = min(rng.start[1], rng.end[1])
            y_max = max(rng.start[1], rng.end[1])
            for y in range(y_min, y_max + 1):
                vents[(x, y)] = vents.get((x, y), 0) + 1

        elif rng.start[1] == rng.end[1]:
            y = rng.start[1]
            x_min = min(rng.start[0], rng.end[0])
            x_max = max(rng.start[0], rng.end[0])
            for x in range(x_min, x_max + 1):
                vents[(x, y)] = vents.get((x, y), 0) + 1

    return sum(1 for v in vents.values() if v > 1)


def solve(input_file: str) -> str:
    return f"There are {result(input_file)} points where lines overlap"
