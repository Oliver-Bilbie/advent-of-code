class Tree:
    def __init__(self, height: int):
        self.height: int = height
        self.score: int = 1


class MonotonicStack:
    def __init__(self):
        self._values: list[tuple[Tree, int]] = []

    def push(self, new_tree: Tree, position: int):
        """
        Push an item to the stack and evaluates visibility for popped trees.
        """
        while len(self._values) > 0 and new_tree.height >= self._values[-1][0].height:
            old_tree, old_position = self._values.pop()
            old_tree.score *= position - old_position
        self._values.append((new_tree, position))

    def resolve(self, n: int):
        """
        Evaluate visibility for remaining trees.
        """
        while len(self._values) > 0:
            old_tree, old_position = self._values.pop()
            old_tree.score *= n - 1 - old_position


def read_trees(input_file: str) -> list[list[Tree]]:
    trees = []

    for line in input_file.splitlines():
        row = []
        for height in line:
            row.append(Tree(int(height)))
        trees.append(row)

    return trees


def compute_scores(trees: list[list[Tree]]) -> None:
    n = len(trees)

    for i in range(n):
        # Scan left to right
        heights = MonotonicStack()
        for j in range(n):
            heights.push(trees[i][j], j)
        heights.resolve(n)

        # Scan right to left
        heights = MonotonicStack()
        for j in range(n):
            heights.push(trees[i][-j - 1], j)
        heights.resolve(n)

        # Scan top to bottom
        heights = MonotonicStack()
        for j in range(n):
            heights.push(trees[j][i], j)
        heights.resolve(n)

        # Scan bottom to top
        heights = MonotonicStack()
        for j in range(n):
            heights.push(trees[-j - 1][i], j)
        heights.resolve(n)


def result(input_file: str) -> int:
    trees = read_trees(input_file)
    compute_scores(trees)
    highest_score = max(t.score for row in trees for t in row)
    return highest_score


def solve(input_file: str) -> str:
    return f"The highest possible scenic score is {result(input_file)}"
