from rustapi_module.test_dict import DictSize
import pytest


@pytest.mark.parametrize(
    "size",
    [64, 128, 256, 1000000],
)
def test_size(size):
    d = {}
    for i in range(size):
        d[i] = str(i)
    assert DictSize(len(d)).iter_dict(d) == size
