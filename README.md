## and itertools


### `all` function.


```python
from and_itertools import all as itertools_all


def test_ok():
    assert itertools_all([1, 2, 3], lambda x: x) is True


def test_ng():
    assert itertools_all([1, 2, 3], lambda x: isinstance(x, (str, bool))) is False


def test_any():
    assert itertools_all([1, "2", False, None], lambda x: isinstance(x, (str,))) is False
```


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
