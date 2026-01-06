from typing import Self
from dataclasses import dataclass


@dataclass(frozen=True)
class Valve:
    flow_rate: int
    leads_to: list[str]


@dataclass
class GlobalState:
    max_flow_rate: int
    max_total_found: int
    valves: dict[str, Valve]


GLOBAL_STATE = GlobalState(0, 0, {})


def read_valves(input_file: str) -> None:
    for line in input_file.splitlines():
        items = line.split(maxsplit=9)
        name = items[1]
        flow_rate = int(items[4][5:-1])
        leads_to = items[9].split(", ")

        GLOBAL_STATE.valves[name] = Valve(flow_rate, leads_to)
        GLOBAL_STATE.max_flow_rate = max(GLOBAL_STATE.max_flow_rate, flow_rate)


def backtrack(
    valve_name: str,
    time_remaining: int,
    opened: tuple[str],
    pressure_reduced: int,
    cache: dict[tuple[str, int, tuple[str]], int],
) -> int:
    if time_remaining <= 1:
        return 0

    theoretical_max = pressure_reduced + GLOBAL_STATE.max_flow_rate * sum(
        range(time_remaining, 0, -2)
    )
    if theoretical_max <= GLOBAL_STATE.max_total_found:
        return 0

    cache_key = (valve_name, time_remaining, opened)
    cache_result = cache.get(cache_key)
    if cache_result is not None:
        return cache_result

    local_max_found = pressure_reduced
    valve = GLOBAL_STATE.valves[valve_name]

    # Try moving
    for next_valve in valve.leads_to:
        local_max_found = max(
            local_max_found,
            backtrack(next_valve, time_remaining - 1, opened, pressure_reduced, cache),
        )

    # Try opening the valve
    if valve_name not in opened and valve.flow_rate > 0:
        pressure_reduced += valve.flow_rate * (time_remaining - 1)
        for next_valve in valve.leads_to:
            local_max_found = max(
                local_max_found,
                backtrack(
                    next_valve,
                    time_remaining - 2,
                    (*opened, valve_name),
                    pressure_reduced,
                    cache,
                ),
            )

    GLOBAL_STATE.max_total_found = max(GLOBAL_STATE.max_total_found, local_max_found)
    cache[cache_key] = local_max_found
    return local_max_found


def result(input_file: str) -> int:
    read_valves(input_file)
    start_valve = "AA"
    time_remaining = 30

    return backtrack(start_valve, time_remaining, (), 0, {})


def solve(input_file: str) -> str:
    return f"The maximum pressure release is: {result(input_file)}"
