from typing import TypeAlias
from collections import deque

Coordinate: TypeAlias = tuple[int, int, int]


def read_cubes(input_file: str) -> tuple[set[Coordinate], Coordinate]:
    coordinates = set()
    boundaries = [0, 0, 0]

    for line in input_file.splitlines():
        items = line.split(",", maxsplit=2)
        x, y, z = tuple(int(v) for v in items)
        coordinates.add((x, y, z))
        for i, val in enumerate([x, y, z]):
            boundaries[i] = max(boundaries[i], val + 1)

    return coordinates, tuple(boundaries)


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


def in_bounds(position: Coordinate, boundaries: Coordinate) -> bool:
    return (
        1 <= position[0] <= boundaries[0]
        and 1 <= position[1] <= boundaries[1]
        and 1 <= position[2] <= boundaries[2]
    )


def is_bubble(
    start: Coordinate,
    cubes: set[Coordinate],
    boundaries: Coordinate,
    cache: dict[Coordinate, bool],
) -> bool:
    cache_result = cache.get(start)
    if cache_result is not None:
        return cache_result

    visited = {start}
    queue = deque([start])

    while len(queue) > 0:
        position = queue.popleft()
        if not in_bounds(position, boundaries):
            for p in visited:
                cache[p] = False
            return False

        for n in adjacent(position):
            if n not in cubes and n not in visited:
                visited.add(n)
                queue.append(n)

    for p in visited:
        cache[p] = True
    return True


def result(input_file: str) -> int:
    cubes, boundaries = read_cubes(input_file)
    bubble_cache: dict[Coordinate, bool] = {}
    surface_area = 0

    for cube in cubes:
        for adj_space in adjacent(cube):
            if adj_space not in cubes and not is_bubble(
                adj_space, cubes, boundaries, bubble_cache
            ):
                surface_area += 1

    return surface_area


def solve(input_file: str) -> str:
    return f"The surface area excluding air bubbles is: {result(input_file)}"
