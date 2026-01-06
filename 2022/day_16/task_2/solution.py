from typing import Self
from dataclasses import dataclass


@dataclass(frozen=True)
class Valve:
    flow_rate: int
    leads_to: list[str]


@dataclass
class GlobalState:
    valves: dict[str, Valve]
    max_total_found: int
    prune_limits: list[int]  # theoretical maximum pressure relief within N minutes


GLOBAL_STATE = GlobalState({}, 0, [])


def read_valves(input_file: str) -> None:
    for line in input_file.splitlines():
        items = line.split(maxsplit=9)
        name = items[1]
        flow_rate = int(items[4][5:-1])
        leads_to = items[9].split(", ")

        GLOBAL_STATE.valves[name] = Valve(flow_rate, leads_to)


def compute_prune_limits(time_limit: int) -> None:
    GLOBAL_STATE.prune_limits = [0] * (time_limit + 1)
    flow_rates = sorted(
        (v.flow_rate for v in GLOBAL_STATE.valves.values() if v.flow_rate > 0),
        reverse=True,
    )
    opened = []
    idx = 0

    for t in range(1, time_limit + 1):
        GLOBAL_STATE.prune_limits[t] = GLOBAL_STATE.prune_limits[t - 1] + sum(opened)
        # Every two minutes we open the two next largest valves
        if t % 2 == 1:
            for _ in range(2):
                if idx < len(flow_rates):
                    opened.append(flow_rates[idx])
                    idx += 1


def backtrack(
    valve1_name: str,
    valve2_name: str,
    time_remaining: int,
    opened: tuple[str],
    accumulated: int,
    cache: dict[tuple[str, str, int, tuple[str]], int],
) -> int:
    if time_remaining <= 1:
        return 0

    # If this path cannot possibly beat the current max value we will prune it
    if (
        GLOBAL_STATE.max_total_found
        > accumulated + GLOBAL_STATE.prune_limits[time_remaining]
    ):
        return 0

    # Sort valves to de-duplicate symmetrical states
    valve1_name, valve2_name = sorted((valve1_name, valve2_name))

    # Attempt to use a cached result
    cache_key = (valve1_name, valve2_name, time_remaining, opened)
    cache_result = cache.get(cache_key)
    if cache_result is not None:
        return cache_result

    local_max_additional = 0
    valve1 = GLOBAL_STATE.valves[valve1_name]
    valve2 = GLOBAL_STATE.valves[valve2_name]

    # Try moving both
    for next_valve1 in valve1.leads_to:
        for next_valve2 in valve2.leads_to:
            additional = backtrack(
                next_valve1,
                next_valve2,
                time_remaining - 1,
                opened,
                accumulated,
                cache,
            )
            local_max_additional = max(local_max_additional, additional)

    # Try opening valve 1 only
    if valve1_name not in opened and valve1.flow_rate > 0:
        now_opened = tuple(sorted((*opened, valve1_name)))
        released = valve1.flow_rate * (time_remaining - 1)
        now_accumulated = accumulated + released
        for next_valve2 in valve2.leads_to:
            additional = released + backtrack(
                valve1_name,
                next_valve2,
                time_remaining - 1,
                now_opened,
                now_accumulated,
                cache,
            )
            local_max_additional = max(local_max_additional, additional)

    # Try opening valve 2 only
    if valve2_name not in opened and valve2.flow_rate > 0:
        now_opened = tuple(sorted((*opened, valve2_name)))
        released = valve2.flow_rate * (time_remaining - 1)
        now_accumulated = accumulated + released
        for next_valve1 in valve1.leads_to:
            additional = released + backtrack(
                next_valve1,
                valve2_name,
                time_remaining - 1,
                now_opened,
                now_accumulated,
                cache,
            )
            local_max_additional = max(local_max_additional, additional)

    # Try opening both valves
    if (
        valve1_name not in opened
        and valve2_name not in opened
        and valve1.flow_rate > 0
        and valve2.flow_rate > 0
        and valve1_name != valve2_name
    ):
        now_opened = tuple(sorted((*opened, valve1_name, valve2_name)))
        released = (valve1.flow_rate + valve2.flow_rate) * (time_remaining - 1)
        now_accumulated = accumulated + released
        additional = released + backtrack(
            valve1_name,
            valve2_name,
            time_remaining - 1,
            now_opened,
            now_accumulated,
            cache,
        )
        local_max_additional = max(local_max_additional, additional)

    # Update global max with accumulated + this additional
    GLOBAL_STATE.max_total_found = max(
        GLOBAL_STATE.max_total_found, accumulated + local_max_additional
    )
    cache[cache_key] = local_max_additional
    return local_max_additional


def result(input_file: str) -> int:
    start_valve = "AA"
    time_remaining = 26

    read_valves(input_file)
    compute_prune_limits(time_remaining)

    return backtrack(start_valve, start_valve, time_remaining, (), 0, {})


def solve(input_file: str) -> str:
    return f"The maximum pressure release is: {result(input_file)}"
