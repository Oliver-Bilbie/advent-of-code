from dataclasses import dataclass
from typing import Self, Optional


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


def evaluate_sizes(item: Directory | File) -> None:
    if isinstance(item, File):
        return

    item.size = 0
    for c in item.children:
        evaluate_sizes(c)
        item.size += c.size


def find_dir_to_delete(item: Directory, additional_space: int) -> Optional[Directory]:
    best_dir = item if item.size >= additional_space else None

    for c in item.children:
        if not isinstance(c, File):
            best_child = find_dir_to_delete(c, additional_space)

            if best_child is None:
                continue

            if best_dir is None:
                best_dir = best_child
            elif additional_space <= best_child.size < best_dir.size:
                best_dir = best_child

    return best_dir


def result(input_file: str) -> int:
    root = read_fs(input_file)
    evaluate_sizes(root)

    total_space = 70000000
    required_space = 30000000
    additional_space = required_space - (total_space - root.size)

    deleted_dir = find_dir_to_delete(root, additional_space)
    return deleted_dir.size


def solve(input_file: str) -> str:
    return f"The size of the smallest suitable directory is: {result(input_file)}"
