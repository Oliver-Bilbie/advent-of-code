from typing import TypeAlias, Optional

CHAMBER_WIDTH = 7
TOTAL_ROCKS = 1000000000000

Position: TypeAlias = tuple[int, int]
Row: TypeAlias = tuple[int, int, int, int, int, int, int]


class Jets:
    def __init__(self, directions: str):
        sanitized_directions = "".join(c for c in directions if c in [">", "<"])
        self._direction = sanitized_directions
        self.index = 0

    def next(self) -> str:
        direction = self._direction[self.index]
        self.index = (self.index + 1) % len(self._direction)
        return direction


class Rocks:
    def __init__(self):
        self.index = 0
        self._rocks: list[set[Position]] = [
            {(0, 0), (1, 0), (2, 0), (3, 0)},
            {(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)},
            {(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)},
            {(0, 0), (0, 1), (0, 2), (0, 3)},
            {(0, 0), (1, 0), (0, 1), (1, 1)},
        ]

    def next(self, height: int) -> set[Position]:
        positions = self._rocks[self.index]
        offset = (2, height + 4)
        positions = set((p[0] + offset[0], p[1] + offset[1]) for p in positions)
        self.index = (self.index + 1) % len(self._rocks)
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
    rock: set[Position], occupied: set[Position], tops: Row
) -> tuple[Optional[set[Position]], Row]:
    new_rock = set((p[0], p[1] - 1) for p in rock)

    for position in new_rock:
        if position in occupied:
            occupied.update(rock)

            new_tops = list(tops)
            for p in rock:
                new_tops[p[0]] = max(new_tops[p[0]], p[1])

            return None, tuple(new_tops)

    return new_rock, tops


def normalize_tops(tops: Row) -> Row:
    min_height = min(tops)
    return tuple(h - min_height for h in tops)


def result(input_file: str) -> int:
    rocks = Rocks()
    jets = Jets(input_file)
    occupied: set[Position] = {(x, 0) for x in range(CHAMBER_WIDTH)}
    tops = (0,) * CHAMBER_WIDTH

    rock_num = 1
    cache: dict[tuple[int, int, Row], tuple[int, int]] = {}

    while rock_num <= TOTAL_ROCKS:
        max_height = max(tops)

        cache_key = (rocks.index, jets.index, normalize_tops(tops))
        cache_result = cache.get(cache_key)
        if cache_result is None:
            cache[cache_key] = (rock_num, max_height)
        else:
            prev_rock_num, prev_max_height = cache_result
            cycle_length = rock_num - prev_rock_num
            cycle_height = max_height - prev_max_height

            skipped_cycles = (TOTAL_ROCKS - rock_num) // cycle_length
            skipped_height = cycle_height * skipped_cycles

            rock_num += cycle_length * skipped_cycles
            max_height += skipped_height
            tops = tuple(h + skipped_height for h in tops)
            occupied = {(p[0], p[1] + skipped_height) for p in occupied}

        rock = rocks.next(max_height)
        while rock is not None:
            rock = handle_jet(rock, jets.next(), occupied)
            rock, new_tops = handle_rock_fall(rock, occupied, tops)

        tops = new_tops
        rock_num += 1

    return max(tops)


def solve(input_file: str) -> str:
    return f"The tower is {result(input_file)} units tall after {TOTAL_ROCKS} rocks have stopped fallilng"
