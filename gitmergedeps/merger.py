from .differ import (AddDep,
                     RemoveDep,
                     UpdateVersion)
from .tools import index_requirements


def req2line(req):
    return req.line


def merger(reqs, diff):
    indexed_requirements = index_requirements(reqs)

    to_add = set()
    to_remove = set()
    for change in diff:
        if isinstance(change, UpdateVersion) and change.req.name in indexed_requirements:
            change.apply(indexed_requirements[change.req.name])
        elif isinstance(change, AddDep):
            to_add.add(change.req)
        elif isinstance(change, RemoveDep):
            to_remove.add(change.req.name)

    reqs.extend([el for el in to_add if el.name not in indexed_requirements])
    # reqs = sort_requirements([req for req in reqs if req.name not in to_remove])
    reqs = [req for req in reqs if req.name not in to_remove]

    return "\n".join([req2line(req) for req in reqs])
