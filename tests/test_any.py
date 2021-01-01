import operator

import pytest

from and_itertools import any as itertools_any


def test_not_iterable():
    with pytest.raises(TypeError):
        itertools_any(None, operator.truth)


def test_ok():
    assert itertools_any([0, 1, 2, 3, False], lambda x: x) is True


def test_ng():
    assert itertools_any([0, None, "", False], operator.truth) is False
    assert itertools_any([0, None, "", False], operator.truth) == any([0, None, "", False])


def test_iterator_ok():
    assert itertools_any(iter([1, 2, 3, 4]), operator.truth) is True
    assert itertools_any(iter([1, 2, 3, 4]), operator.truth) == any(iter([1, 2, 3, 4]))


def test_iterator_ng():
    assert itertools_any(iter([0, None, "", False]), operator.truth) is False
    assert itertools_any(iter([0, None, "", False]), operator.truth) == any(iter([0, None, "", False]))


def test_generator_ok():
    assert itertools_any(range(1, 3), operator.truth) is True
    assert itertools_any(range(1, 3), operator.truth) == any(range(1, 3))


def test_generator_ng():
    assert itertools_any((i for i in [0, "", False, None]), operator.truth) is False
    assert itertools_any((i for i in [0, "", False, None]), operator.truth) == any((i for i in [0, "", False, None]))


def test_generator_types():
    assert itertools_any((i for i in [1, "2", False, None]), lambda x: isinstance(x, (str,))) is True


def test_builtin():
    assert itertools_any([1, 2, 3], operator.truth) is True
    assert itertools_any([1, 2, 3], operator.truth) == any([1, 2, 3])


def test_predicate():
    assert itertools_any([1, 2, 3], lambda x: isinstance(x, (str, bool))) is False
    assert itertools_any([1, 2, 3], lambda x: isinstance(x, (str, bool))) is False


def test_types():
    assert itertools_any([1, "2", False, None], lambda x: isinstance(x, (str,))) is True
    assert itertools_any([1, "2", False, None], lambda x: isinstance(x, (str,))) is True
