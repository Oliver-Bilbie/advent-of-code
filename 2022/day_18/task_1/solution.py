from typing import TypeAlias

Coordinate: TypeAlias = tuple[int, int, int]


def read_cubes(input_file: str) -> set[Coordinate]:
    coordinates = set()

    for line in input_file.splitlines():
        items = line.split(",", maxsplit=2)
        x, y, z = tuple(int(v) for v in items)
        coordinates.add((x, y, z))

    return coordinates


def adjacent(position: Coordinate) -> set[Coordinate]:
    return set(
        [
            (position[0] + 1, position[1], position[2]),
            (position[0], position[1] + 1, position[2]),
            (position[0], position[1], position[2] + 1),
            (position[0] - 1, position[1], position[2]),
            (position[0], position[1] - 1, position[2]),
            (position[0], position[1], position[2] - 1),
        ]
    )


def result(input_file: str) -> int:
    cubes = read_cubes(input_file)
    surface_area = 0

    for cube in cubes:
        for adj_space in adjacent(cube):
            if adj_space not in cubes:
                surface_area += 1

    return surface_area


def solve(input_file: str) -> str:
    return f"The surface area is: {result(input_file)}"
