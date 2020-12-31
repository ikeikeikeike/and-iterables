from and_itertools import all as itertools_all


def test_ok():
    assert itertools_all([1, 2, 3], lambda x: x) is True


def test_ng():
    assert itertools_all([1, 2, 3], lambda x: isinstance(x, (str, bool))) is False


def test_any():
    assert itertools_all([1, "2", False, None], lambda x: isinstance(x, (str,))) is False
