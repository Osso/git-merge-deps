import pytest

from gitmergedeps.differ import (differ,
                                 merger)
import requirements


@pytest.mark.parametrize("old,new,expected",
                        [("", "new-dep", "new-dep"),
                         ("new-dep", "", ""),
                         ("new-dep==0.1", "new-dep==0.2", "new-dep==0.2"),
                         ("new-dep==0.2", "new-dep==0.1", "new-dep==0.2"),
                         ("new-dep<=0.1", "new-dep==0.2", "new-dep==0.2"),
                         ("dep==1.1.2", "dep==1.1.0", "dep==1.1.2")])
def test_merge(old, new, expected):
    old_req = list(requirements.parse(old))
    new_req = list(requirements.parse(new))
    diff = differ(old_req, new_req)
    merged = merger(old_req, diff)
    assert merged == expected
