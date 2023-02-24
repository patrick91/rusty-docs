from typing import overload


@overload
def f(x: int) -> int:
    ...


@overload
def f(x: str) -> str:
    ...


def f(x: str | int) -> str | int:
    """
    This is a docstring

    Args:
        x: This is a parameter

    Returns:
        This is a return value
    """

    return x
