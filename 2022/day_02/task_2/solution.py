from enum import IntEnum
from typing import Self


class Outcome(IntEnum):
    WIN = 6
    DRAW = 3
    LOSE = 0

    @classmethod
    def from_guide(cls, c: str) -> Self:
        match c:
            case "X":
                return cls.LOSE
            case "Y":
                return cls.DRAW
            case "Z":
                return cls.WIN
            case _:
                raise ValueError(f"{c} is not a valid outcome")


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
    def from_outcome(cls, outcome: Outcome, opponent: Self) -> Self:
        if outcome == Outcome.DRAW:
            return opponent

        match opponent:
            case cls.ROCK:
                if outcome == Outcome.WIN:
                    return cls.PAPER
                return cls.SCISSORS
            case cls.PAPER:
                if outcome == Outcome.WIN:
                    return cls.SCISSORS
                return cls.ROCK
            case cls.SCISSORS:
                if outcome == Outcome.WIN:
                    return cls.ROCK
                return cls.PAPER


def result(input_file: str) -> int:
    total = 0

    for line in input_file.splitlines():
        opponent = Choice.from_opponent(line[0])
        outcome = Outcome.from_guide(line[2])
        player = Choice.from_outcome(outcome, opponent)
        total += player + outcome

    return total


def solve(input_file: str) -> str:
    return f"The final score is: {result(input_file)}"
