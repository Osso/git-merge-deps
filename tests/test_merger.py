import pytest

from gitmergedeps.differ import differ
from gitmergedeps.merger import merger
from gitmergedeps.requirement import parse_requirements


@pytest.mark.parametrize("old,new,expected",
                        [("", "new-dep", "new-dep"),
                         ("new-dep", "", ""),
                         ("new-dep==0.1", "new-dep==0.2", "new-dep==0.2"),
                         ("new-dep==0.2", "new-dep==0.1", "new-dep==0.2"),
                         ("new-dep<=0.1", "new-dep==0.2", "new-dep==0.2"),
                         ("dep==1.1.2", "dep==1.1.0", "dep==1.1.2"),
                         ("# comment\ndep==1 # c2", "# comment\ndep==2 # c2", "# comment\ndep==2 # c2"),
                         ("dep[opt]==0.1", "dep[opt]==0.2", "dep[opt]==0.2")])
def test_merge(old, new, expected):
    old_req = parse_requirements(old)
    new_req = parse_requirements(new)
    diff = differ(old_req, new_req)
    merged = merger(old_req, diff)
    assert merged == expected
