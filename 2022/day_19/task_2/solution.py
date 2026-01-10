from dataclasses import dataclass
from enum import Enum
from typing import Self


class Resource(Enum):
    ORE = 0
    CLAY = 1
    OBSIDIAN = 2
    GEODE = 3


@dataclass(frozen=True)
class Robots:
    ore: int
    clay: int
    obsidian: int
    geode: int

    def add(self, resource: Resource) -> Self:
        return Robots(
            self.ore + int(resource == Resource.ORE),
            self.clay + int(resource == Resource.CLAY),
            self.obsidian + int(resource == Resource.OBSIDIAN),
            self.geode + int(resource == Resource.GEODE),
        )


@dataclass(frozen=True)
class Resources:
    ore: int
    clay: int
    obsidian: int
    geode: int

    def add_robot_harvest(self, robots: Robots) -> Self:
        return Resources(
            self.ore + robots.ore,
            self.clay + robots.clay,
            self.obsidian + robots.obsidian,
            self.geode + robots.geode,
        )

    def pay_for_robot(self, cost: Self) -> tuple[Self, bool]:
        if (
            cost.ore > self.ore
            or cost.clay > self.clay
            or cost.obsidian > self.obsidian
            # Geodes are never required as payment so we do not need to check them
        ):
            return (self, False)

        return (
            Resources(
                self.ore - cost.ore,
                self.clay - cost.clay,
                self.obsidian - cost.obsidian,
                self.geode - cost.geode,
            ),
            True,
        )


@dataclass(frozen=True)
class Blueprint:
    ore: Resources
    clay: Resources
    obsidian: Resources
    geode: Resources


def read_blueprint(line: str) -> Blueprint:
    items = line.split()
    ore_rbt_cost = Resources(int(items[6]), 0, 0, 0)
    clay_rbt_cost = Resources(int(items[12]), 0, 0, 0)
    obsidian_rbt_cost = Resources(int(items[18]), int(items[21]), 0, 0)
    geode_rbt_cost = Resources(int(items[27]), 0, int(items[30]), 0)
    return Blueprint(ore_rbt_cost, clay_rbt_cost, obsidian_rbt_cost, geode_rbt_cost)


def find_max_inputs(blueprint: Blueprint) -> Resources:
    return Resources(
        max(
            blueprint.ore.ore,
            blueprint.clay.ore,
            blueprint.obsidian.ore,
            blueprint.geode.ore,
        ),
        max(
            blueprint.ore.clay,
            blueprint.clay.clay,
            blueprint.obsidian.clay,
            blueprint.geode.clay,
        ),
        max(
            blueprint.ore.obsidian,
            blueprint.clay.obsidian,
            blueprint.obsidian.obsidian,
            blueprint.geode.obsidian,
        ),
        max(
            blueprint.ore.geode,
            blueprint.clay.geode,
            blueprint.obsidian.geode,
            blueprint.geode.geode,
        ),
    )


def normalize_resources(
    resources: Resources, max_inputs: Resources, time_remaining: int
) -> Resources:
    """
    Reduce resource counts to the maximum that can be spent in
    the remaining time. This improves caching since resource
    counts beyond these limits represent equivalent states.
    """
    return Resources(
        min(resources.ore, max_inputs.ore * time_remaining),
        min(resources.clay, max_inputs.clay * time_remaining),
        min(resources.obsidian, max_inputs.obsidian * time_remaining),
        resources.geode,
    )


def backtrack(
    resources: Resources,
    robots: Robots,
    time_remaining: int,
    blueprint: Blueprint,
    max_inputs: Resources,
    global_max_geodes: list[int],
    cache: dict[tuple[Resources, Robots, int], int],
) -> int:
    if time_remaining <= 0:
        return resources.geode

    # Prune this branch if we cannot reach the current max value by building a geode robot
    # on every turn until the end
    if (
        global_max_geodes[0]
        >= resources.geode
        + robots.geode * time_remaining
        + time_remaining * (time_remaining - 1) // 2
    ):
        return resources.geode

    resources = normalize_resources(resources, max_inputs, time_remaining)
    cache_key = (resources, robots, time_remaining)
    cache_result = cache.get(cache_key)
    if cache_result is not None:
        return cache_result

    global_max_geodes[0] = max(global_max_geodes[0], resources.geode)
    local_max_geodes = 0

    # Try building an ore robot if we can afford to do so
    next_resources, can_afford = resources.pay_for_robot(blueprint.geode)
    if can_afford:
        next_resources = next_resources.add_robot_harvest(robots)
        next_robots = robots.add(Resource.GEODE)
        local_max_geodes = max(
            local_max_geodes,
            backtrack(
                next_resources,
                next_robots,
                time_remaining - 1,
                blueprint,
                max_inputs,
                global_max_geodes,
                cache,
            ),
        )
        global_max_geodes[0] = max(global_max_geodes[0], local_max_geodes)

        # If we are able to build a geode robot, we should always do so.
        # Therefore we can exit early now
        cache[cache_key] = local_max_geodes
        return local_max_geodes

    # Try building an ore robot if:
    # - we can afford to do so
    # - we are producing less ore than we can spend in one minute
    next_resources, can_afford = resources.pay_for_robot(blueprint.ore)
    if can_afford and robots.ore < max_inputs.ore:
        next_resources = next_resources.add_robot_harvest(robots)
        next_robots = robots.add(Resource.ORE)
        local_max_geodes = max(
            local_max_geodes,
            backtrack(
                next_resources,
                next_robots,
                time_remaining - 1,
                blueprint,
                max_inputs,
                global_max_geodes,
                cache,
            ),
        )
        global_max_geodes[0] = max(global_max_geodes[0], local_max_geodes)

    # Try building a clay robot if:
    # - we can afford to do so
    # - we are producing less clay than we can spend in one minute
    next_resources, can_afford = resources.pay_for_robot(blueprint.clay)
    if can_afford and robots.clay < max_inputs.clay:
        next_resources = next_resources.add_robot_harvest(robots)
        next_robots = robots.add(Resource.CLAY)
        local_max_geodes = max(
            local_max_geodes,
            backtrack(
                next_resources,
                next_robots,
                time_remaining - 1,
                blueprint,
                max_inputs,
                global_max_geodes,
                cache,
            ),
        )
        global_max_geodes[0] = max(global_max_geodes[0], local_max_geodes)

    # Try building an obsidian robot if:
    # - we can afford to do so
    # - we are producing less obsidian than we can spend in one minute
    next_resources, can_afford = resources.pay_for_robot(blueprint.obsidian)
    if can_afford and robots.obsidian < max_inputs.obsidian:
        next_resources = next_resources.add_robot_harvest(robots)
        next_robots = robots.add(Resource.OBSIDIAN)
        local_max_geodes = max(
            local_max_geodes,
            backtrack(
                next_resources,
                next_robots,
                time_remaining - 1,
                blueprint,
                max_inputs,
                global_max_geodes,
                cache,
            ),
        )
        global_max_geodes[0] = max(global_max_geodes[0], local_max_geodes)

    # Try not building anything right now
    next_resources = resources.add_robot_harvest(robots)
    local_max_geodes = max(
        local_max_geodes,
        backtrack(
            next_resources,
            robots,
            time_remaining - 1,
            blueprint,
            max_inputs,
            global_max_geodes,
            cache,
        ),
    )
    global_max_geodes[0] = max(global_max_geodes[0], local_max_geodes)

    cache[cache_key] = local_max_geodes
    return local_max_geodes


def result(input_file: str, n: int) -> int:
    time_limit = 32
    best_geodes_product = 1

    for line in input_file.splitlines()[:n]:
        resources = Resources(0, 0, 0, 0)
        robots = Robots(1, 0, 0, 0)
        blueprint = read_blueprint(line)
        max_inputs = find_max_inputs(blueprint)
        global_max = [0]
        cache = {}
        max_geodes_opened = backtrack(
            resources, robots, time_limit, blueprint, max_inputs, global_max, cache
        )
        best_geodes_product *= max_geodes_opened

    return best_geodes_product


def solve(input_file: str) -> str:
    return f"The product of the largest numbers of geodes is: {result(input_file, 3)}"
