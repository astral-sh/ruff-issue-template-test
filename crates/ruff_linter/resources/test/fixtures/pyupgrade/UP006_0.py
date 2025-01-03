import typing


def f(x: typing.List[str]) -> None:
    ...


from typing import List


def f(x: List[str]) -> None:
    ...


import typing as t


def f(x: t.List[str]) -> None:
    ...


from typing import List as IList


def f(x: IList[str]) -> None:
    ...


def f(x: "List[str]") -> None:
    ...


def f(x: r"List[str]") -> None:
    ...


def f(x: "List[str]") -> None:
    ...


def f(x: """List[str]""") -> None:
    ...


def f(x: "Li" "st[str]") -> None:
    ...


def f(x: "List['List[str]']") -> None:
    ...


def f(x: "List['Li' 'st[str]']") -> None:
    ...


def f(x: "Li" "st['List[str]']") -> None:
    ...


def f(x: typing.Deque[str]) -> None:
    ...


def f(x: typing.DefaultDict[str, str]) -> None:
    ...


def f(x: typing.AbstractSet[str]) -> None:
    ...


def f(x: typing.Pattern[str]) -> None:
    ...


def f(x: typing.Sequence[str]) -> None:
    ...


from typing import Collection


def f(x: typing.Collection[str]) -> None:
    ...
