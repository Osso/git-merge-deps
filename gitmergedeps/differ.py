from packaging import version


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


def merger(reqs, diff):
    indexed_requirements = index_requirements(reqs)

    to_add = set()
    to_remove = set()
    for change in diff:
        if isinstance(change, UpdateVersion):
            change.apply(indexed_requirements[change.req.name])
        elif isinstance(change, AddDep):
            to_add.add(change.req)
        elif isinstance(change, RemoveDep):
            to_remove.add(change.req.name)

    reqs.extend([el for el in to_add if el.name not in indexed_requirements])
    # reqs = sort_requirements([req for req in reqs if req.name not in to_remove])
    reqs = [req for req in reqs if req.name not in to_remove]

    return "\n".join([req2line(req) for req in reqs])


def index_requirements(reqs):
    return {req.name: req for req in reqs}


def sort_requirements(reqs):
    return sorted(reqs, key=lambda r: r.name)


def req2line(req):
    return req.line


class DiffEl:
    def __init__(self, req):
        self.req = req


class UpdateVersion(DiffEl):
    def __repr__(self):
        return "UpdateVersion(req=%r)" % self.req

    def apply(self, req):
        if self.req.revision:
            if req.revision:
                if version.parse(req.revision) < version.parse(self.req.revision):
                    req.revision = self.req.revision
                req.line = req.line.replace(f"@{req.revision}",
                                            f"@{self.req.revision}")
            else:
                req.revision = self.req.revision
                req.line = self.req.line

        elif self.req.specs:
            if req.specs:
                other_constraint, other_version = req.specs[0]
                self_constraint, self_version = self.req.specs[0]
                if version.parse(other_version) < version.parse(self_version):
                    req.specs[0] = (self_constraint, self_version)
                    req.line = req.line.replace(f"{other_constraint}{other_version}",
                                                f"{self_constraint}{self_version}")
            else:
                req.specs[0] = self.req.specs[0]
                req.line = self.req.line
        else:
            assert False


class RemoveDep(DiffEl):
    def __repr__(self):
        return "RemoveDep(req=%r)" % self.req


class AddDep(DiffEl):
    def __repr__(self):
        return "AddDep(req=%r)" % self.req
