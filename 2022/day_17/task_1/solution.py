from typing import TypeAlias, Optional

CHAMBER_WIDTH = 7

Position: TypeAlias = tuple[int, int]


class Jets:
    def __init__(self, directions: str):
        sanitized_directions = "".join(c for c in directions if c in [">", "<"])
        self._direction = sanitized_directions
        self._index = 0

    def next(self) -> str:
        direction = self._direction[self._index]
        self._index = (self._index + 1) % len(self._direction)
        return direction


class Rocks:
    def __init__(self):
        self._index = 0
        self._rocks: list[set[Position]] = [
            {(0, 0), (1, 0), (2, 0), (3, 0)},
            {(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)},
            {(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)},
            {(0, 0), (0, 1), (0, 2), (0, 3)},
            {(0, 0), (1, 0), (0, 1), (1, 1)},
        ]

    def next(self, height: int) -> set[Position]:
        positions = self._rocks[self._index]
        offset = (2, height + 4)
        positions = set((p[0] + offset[0], p[1] + offset[1]) for p in positions)
        self._index = (self._index + 1) % len(self._rocks)
        return positions


def handle_jet(
    rock: set[Position], direction: str, occupied: set[Position]
) -> set[Position]:
    match direction:
        case ">":
            new_rock = set()
            for position in rock:
                new_position = (position[0] + 1, position[1])
                if new_position[0] >= CHAMBER_WIDTH or new_position in occupied:
                    return rock
                new_rock.add(new_position)
            return new_rock

        case "<":
            new_rock = set()
            for position in rock:
                new_position = (position[0] - 1, position[1])
                if new_position[0] < 0 or new_position in occupied:
                    return rock
                new_rock.add(new_position)
            return new_rock

        case _:
            raise ValueError(f"{direction} is not a valid direction")


def handle_rock_fall(
    rock: set[Position], occupied: set[Position]
) -> tuple[Optional[set[Position]], Optional[int]]:
    new_rock = set((p[0], p[1] - 1) for p in rock)
    for position in new_rock:
        if position in occupied:
            occupied.update(rock)
            max_rock_height = max(p[1] for p in rock)
            return None, max_rock_height
    return new_rock, None


def result(input_file: str) -> int:
    rocks = Rocks()
    jets = Jets(input_file)
    occupied: set[Position] = {(x, 0) for x in range(CHAMBER_WIDTH)}
    max_height = 0

    for _ in range(2022):
        rock = rocks.next(max_height)
        while rock is not None:
            rock = handle_jet(rock, jets.next(), occupied)
            rock, new_height = handle_rock_fall(rock, occupied)
        max_height = max(max_height, new_height)

    return max_height


def solve(input_file: str) -> str:
    return f"The tower is {result(input_file)} units tall after 2022 rocks have stopped fallilng"
