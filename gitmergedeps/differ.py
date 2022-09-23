from packaging import version

from .tools import index_requirements


def differ(old_requirements, new_requirements):
    diff = []
    indexed_old_requirements = index_requirements(old_requirements)
    indexed_new_requirements = index_requirements(new_requirements)

    for req in new_requirements:
        if not req.name:
            continue
        if req.name in indexed_old_requirements:
            if req.line == indexed_old_requirements[req.name].line:
                continue
            diff += [UpdateVersion(req)]
        else:
            diff += [AddDep(req)]

    for req in old_requirements:
        if req.name not in indexed_new_requirements:
            diff += [RemoveDep(req)]

    return diff


class DiffEl:
    def __init__(self, req):
        self.req = req


class UpdateVersion(DiffEl):
    def __repr__(self):
        return "UpdateVersion(req=%r)" % self.req

    def apply(self, req):
        if self.req.constraint:
            req.constraint = self.req.constraint

        if self.req.revision:
            assert not req.version
            if req.revision is None or version.parse(req.revision) < version.parse(self.req.revision):
                req.revision = self.req.revision

        elif self.req.version:
            if req.version is None or version.parse(req.version) < version.parse(self.req.version):
                req.version = self.req.version

        else:
            req.constraint = None
            req.revision = None
            req.version = None


class RemoveDep(DiffEl):
    def __repr__(self):
        return "RemoveDep(req=%r)" % self.req


class AddDep(DiffEl):
    def __repr__(self):
        return "AddDep(req=%r)" % self.req
