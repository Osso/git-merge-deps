from gitmergedeps.differ import (AddDep,
                                 RemoveDep,
                                 UpdateVersion,
                                 differ)
from gitmergedeps.requirement import parse_requirements


def test_add():
    old = list(parse_requirements(""))
    new = list(parse_requirements("new-dep"))
    diff = differ(old, new)
    assert len(diff) == 1
    assert isinstance(diff[0], AddDep)
    assert diff[0].req.name == "new-dep"


def test_remove():
    old = list(parse_requirements("new-dep"))
    new = list(parse_requirements(""))
    diff = differ(old, new)
    assert len(diff) == 1
    assert isinstance(diff[0], RemoveDep)
    assert diff[0].req.name == "new-dep"


def test_update():
    old = list(parse_requirements("new-dep==0.1"))
    new = list(parse_requirements("new-dep==0.2"))
    diff = differ(old, new)
    assert len(diff) == 1
    assert isinstance(diff[0], UpdateVersion)
    assert diff[0].req.name == "new-dep"


def test_update_older():
    old = list(parse_requirements("new-dep==0.2"))
    new = list(parse_requirements("new-dep==0.1"))
    diff = differ(old, new)
    assert len(diff) == 1
    assert isinstance(diff[0], UpdateVersion)
    assert diff[0].req.name == "new-dep"
