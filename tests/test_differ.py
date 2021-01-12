from gitmergedeps.differ import (AddDep,
                                 RemoveDep,
                                 UpdateVersion,
                                 differ)
import requirements


def test_add():
    old = list(requirements.parse(""))
    new = list(requirements.parse("new-dep"))
    diff = differ(old, new)
    assert len(diff) == 1
    assert isinstance(diff[0], AddDep)
    assert diff[0].req.name == "new-dep"


def test_remove():
    old = list(requirements.parse("new-dep"))
    new = list(requirements.parse(""))
    diff = differ(old, new)
    assert len(diff) == 1
    assert isinstance(diff[0], RemoveDep)
    assert diff[0].req.name == "new-dep"


def test_update():
    old = list(requirements.parse("new-dep==0.1"))
    new = list(requirements.parse("new-dep==0.2"))
    diff = differ(old, new)
    assert len(diff) == 1
    assert isinstance(diff[0], UpdateVersion)
    assert diff[0].req.name == "new-dep"


def test_update_older():
    old = list(requirements.parse("new-dep==0.2"))
    new = list(requirements.parse("new-dep==0.1"))
    diff = differ(old, new)
    assert len(diff) == 1
    assert isinstance(diff[0], UpdateVersion)
    assert diff[0].req.name == "new-dep"
