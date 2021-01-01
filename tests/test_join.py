import pytest

from and_itertools import join


def test_not_iterable():
    with pytest.raises(TypeError):
        join(["1", 1], ", ")


def test_ok():
    assert join(map(str, [1, 2, 3]), ", ") == str.join(", ", map(str, [1, 2, 3]))


def test_ng():
    assert join(map(str, [0, 1, 2, False]), ", ") == str.join(", ", map(str, [0, 1, 2, False]))


def test_iterator_ok():
    assert join(iter(map(str, [1, 2, 3, 4])), ", ") == str.join(", ", iter(map(str, [1, 2, 3, 4])))


def test_iterator_ng():
    assert join(iter(map(str, [0, 1, 2, 3, 4])), ", ") == str.join(", ", iter(map(str, [0, 1, 2, 3, 4])))


def test_generator_ok():
    assert join(map(str, range(1, 10001)), ", ") == str.join(", ", map(str, range(1, 10001)))


def test_generator_ng():
    assert join(map(str, range(0, 10001)), ", ") == str.join(", ", map(str, range(0, 10001)))
