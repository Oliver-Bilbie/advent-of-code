def result(input_file: str) -> int:
    print(f"Received input:\n{input_file}")
    return 123


def solve(input_file: str) -> str:
    return f"The result is: {result(input_file)}"
