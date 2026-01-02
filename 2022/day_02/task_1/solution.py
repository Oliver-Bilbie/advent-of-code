from enum import IntEnum
from typing import Self


class Choice(IntEnum):
    ROCK = 1
    PAPER = 2
    SCISSORS = 3

    @classmethod
    def from_opponent(cls, c: str) -> Self:
        match c:
            case "A":
                return cls.ROCK
            case "B":
                return cls.PAPER
            case "C":
                return cls.SCISSORS
            case _:
                raise ValueError(f"{c} is not a valid choice")

    @classmethod
    def from_player(cls, c: str) -> Self:
        match c:
            case "X":
                return cls.ROCK
            case "Y":
                return cls.PAPER
            case "Z":
                return cls.SCISSORS
            case _:
                raise ValueError(f"{c} is not a valid choice")


class Outcome(IntEnum):
    WIN = 6
    DRAW = 3
    LOSE = 0

    @classmethod
    def from_round(cls, player: Choice, opponent: Choice) -> Self:
        if opponent == player:
            return cls.DRAW

        match player:
            case Choice.ROCK:
                if opponent == Choice.SCISSORS:
                    return cls.WIN
            case Choice.PAPER:
                if opponent == Choice.ROCK:
                    return cls.WIN
            case Choice.SCISSORS:
                if opponent == Choice.PAPER:
                    return cls.WIN

        return cls.LOSE


def result(input_file: str) -> int:
    total = 0

    for line in input_file.splitlines():
        player = Choice.from_player(line[2])
        opponent = Choice.from_opponent(line[0])
        outcome = Outcome.from_round(player, opponent)
        total += player + outcome

    return total


def solve(input_file: str) -> str:
    return f"The final score is: {result(input_file)}"
