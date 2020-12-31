import operator

from and_itertools import all as itertools_all


# def test_generator():
#     assert itertools_all(range(1, 3), operator.truth) is True


def test_builtin():
    assert itertools_all([1, 2, 3], operator.truth) is True


def test_ok():
    assert itertools_all([1, 2, 3], lambda x: x) is True


def test_ng():
    assert itertools_all([1, 2, 3], lambda x: isinstance(x, (str, bool))) is False


def test_any():
    assert itertools_all([1, "2", False, None], lambda x: isinstance(x, (str,))) is False
