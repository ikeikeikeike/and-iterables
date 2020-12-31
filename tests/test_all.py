import operator

import pytest

from and_itertools import all as itertools_all


def test_not_iterable():
    with pytest.raises(TypeError):
        itertools_all(None, operator.truth)


def test_ok():
    assert itertools_all([1, 2, 3], lambda x: x) is True


def test_ng():
    assert itertools_all([0, 1, 2, False], operator.truth) is False


def test_iterator_ok():
    assert itertools_all(iter([1, 2, 3, 4]), operator.truth) is True


def test_iterator_ng():
    assert itertools_all(iter([0, 1, 2, 3, 4]), operator.truth) is False


def test_generator_ok():
    assert itertools_all(range(1, 3), operator.truth) is True


def test_generator_ng():
    assert itertools_all(range(0, 3), operator.truth) is False


def test_generator_types():
    assert itertools_all((i for i in [1, "2", False, None]), lambda x: isinstance(x, (str,))) is False


def test_builtin():
    assert itertools_all([1, 2, 3], operator.truth) is True


def test_predicate():
    assert itertools_all([1, 2, 3], lambda x: isinstance(x, (str, bool))) is False


def test_types():
    assert itertools_all([1, "2", False, None], lambda x: isinstance(x, (str,))) is False
