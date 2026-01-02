from dataclasses import dataclass
from typing import Self, Optional


MAX_SIZE = 100000


@dataclass
class File:
    name: str
    size: int


class Directory:
    def __init__(self, name: str):
        self.name: str = name
        self.size: Optional[int] = None
        self.children: list[Self | File] = []

    def get_child(self, target: str) -> Self | File:
        for d in self.children:
            if d.name == target:
                return d
        raise KeyError("Target directory does not exist")


def read_fs(input_file: str) -> Directory:
    root = Directory("root")
    path = [root]

    for line in input_file.splitlines():
        if line == "$ ls":
            continue

        if line.startswith("$ cd"):
            target = line[5:]
            match target:
                case "/":
                    path = [root]
                case "..":
                    path.pop()
                case _:
                    path.append(path[-1].get_child(target))

        else:
            info, name = line.split(maxsplit=1)
            if info == "dir":
                path[-1].children.append(Directory(name))
            else:
                path[-1].children.append(File(name, int(info)))

    return root


def evaluate_sizes(item: Directory | File) -> int:
    if isinstance(item, File):
        return 0

    size = 0
    directory_sum = 0

    for c in item.children:
        directory_sum += evaluate_sizes(c)
        size += c.size

    item.size = size
    if size <= MAX_SIZE:
        directory_sum += size

    return directory_sum


def result(input_file: str) -> int:
    root = read_fs(input_file)
    return evaluate_sizes(root)


def solve(input_file: str) -> str:
    return f"The sum of the sizes of the directories is: {result(input_file)}"
