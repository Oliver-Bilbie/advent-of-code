class Tree:
    def __init__(self, height: int):
        self.height: int = height
        self.is_visible: bool = False


def read_trees(input_file: str) -> list[list[Tree]]:
    trees = []

    for line in input_file.splitlines():
        row = []
        for height in line:
            row.append(Tree(int(height)))
        trees.append(row)

    return trees


def evaluate_visibility(trees: list[list[Tree]]) -> None:
    n = len(trees)

    for i in range(n):
        # Scan left to right
        max_height = -1
        for j in range(n):
            if trees[i][j].height > max_height:
                trees[i][j].is_visible = True
                max_height = trees[i][j].height

        # Scan right to left
        max_height = -1
        for j in range(n):
            if trees[i][-j - 1].height > max_height:
                trees[i][-j - 1].is_visible = True
                max_height = trees[i][-j - 1].height

        # Scan top to bottom
        max_height = -1
        for j in range(n):
            if trees[j][i].height > max_height:
                trees[j][i].is_visible = True
                max_height = trees[j][i].height

        # Scan bottom to top
        max_height = -1
        for j in range(n):
            if trees[-j - 1][i].height > max_height:
                trees[-j - 1][i].is_visible = True
                max_height = trees[-j - 1][i].height


def result(input_file: str) -> int:
    trees = read_trees(input_file)
    evaluate_visibility(trees)
    visible_count = sum(t.is_visible for row in trees for t in row)
    return visible_count


def solve(input_file: str) -> str:
    return f"There are {result(input_file)} visible trees"
