class BingoBoard:
    def __init__(self, input: list[str]):
        grid = [[int(v) for v in row.split()] for row in input]
        self.lines: list[set[int]] = []
        for row in grid:
            self.lines.append(set(row))
        for col_num in range(5):
            self.lines.append(set(grid[i][col_num] for i in range(5)))

    def mark(self, number: int) -> int | None:
        has_won = False

        for l in self.lines:
            l.discard(number)
            if len(l) == 0:
                has_won = True

        if has_won:
            return number * sum(set(v for l in self.lines for v in l))

        return None


def result(input_file: str) -> int:
    lines = input_file.splitlines()
    numbers = [int(n) for n in lines[0].split(",")]
    boards = [BingoBoard(lines[i : 5 + i]) for i in range(2, len(lines) - 4, 6)]

    for n in numbers:
        remove_boards = set()
        for i, b in enumerate(boards):
            score = b.mark(n)
            if score is not None:
                if len(boards) == 1:
                    return score
                else:
                    remove_boards.add(i)
        boards = [b for (i, b) in enumerate(boards) if i not in remove_boards]

    raise ValueError("No boards contained a bingo")


def solve(input_file: str) -> str:
    return f"The final score is {result(input_file)}"
