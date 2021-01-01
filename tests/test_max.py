import pytest

from and_itertools import max as itertools_max


def test_not_iterable():
    with pytest.raises(TypeError):
        itertools_max(["", 1])


def test_ok():
    assert itertools_max([1, 2, 3]) == 3


def test_ng():
    assert itertools_max([0, 1, 2, False]) == 2


def test_iterator_ok():
    assert itertools_max(iter([1, 2, 3, 4])) == 4


def test_iterator_ng():
    assert itertools_max(iter([0, 1, 2, 3, 4])) == 4


def test_generator_ok():
    assert itertools_max(range(1, 10001)) == 10000


def test_generator_ng():
    assert itertools_max(range(0, 10001)) == 10000
