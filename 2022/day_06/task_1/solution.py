MARKER_LENGTH = 4


def solve(input_file: str) -> str:
    return f"{result(input_file)} characters need to be processed"


def result(input_file: str) -> int:
    for i in range(len(input_file) - MARKER_LENGTH + 1):
        if len(set(input_file[i : i + MARKER_LENGTH])) == MARKER_LENGTH:
            return i + MARKER_LENGTH

    raise ValueError("The input does not contain a marker")
