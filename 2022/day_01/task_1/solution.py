def solve(input_file: str) -> str:
    return f"The Elf is carrying {result(input_file)} Calories"


def result(input_file: str) -> int:
    max_calories = 0
    current_calories = 0

    for item in input_file.splitlines():
        if item == "":
            max_calories = max(max_calories, current_calories)
            current_calories = 0
        else:
            current_calories += int(item)

    max_calories = max(max_calories, current_calories)

    return max_calories
