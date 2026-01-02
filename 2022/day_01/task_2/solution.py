import heapq


def solve(input_file: str) -> str:
    return f"The top 3 Elves are carrying a combined {result(input_file)} Calories"


def result(input_file: str) -> int:
    top3 = []
    current_calories = 0

    for item in input_file.splitlines():
        if item == "":
            push_top_n(top3, current_calories, 3)
            current_calories = 0
        else:
            current_calories += int(item)

    push_top_n(top3, current_calories, 3)

    return sum(top3)


def push_top_n(pq: list[int], value: int, n: int) -> None:
    heapq.heappush(pq, value)
    if len(pq) > n:
        heapq.heappop(pq)
