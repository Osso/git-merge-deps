def index_requirements(reqs):
    return {req.name: req for req in reqs}


def sort_requirements(reqs):
    return sorted(reqs, key=lambda r: r.name)
