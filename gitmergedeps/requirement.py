import re

VCS_SCHEMES = [
    'git',
    'git+https',
    'git+ssh',
    'git+git',
]

NAME_REQ_REGEX = re.compile(
    r'^(?P<name>[\w_-]+)'
)
NAME_VERSION_REQ_REGEX = re.compile(
    r'^(?P<name>[\w_-]+(\[\w+\])?) *'
    r'(?P<constraint>[<>=]=)'
    r' *(?P<version>[\d.]+)'
)
VCS_REGEX = re.compile(
    r'^(?P<scheme>{0})://'.format(r'|'.join(
        [scheme.replace('+', r'\+') for scheme in VCS_SCHEMES])) +
    r'((?P<login>[^/@]+)@)?'
    r'(?P<path>[^#@]+)'
    r'(@(?P<revision>[^#]+))?'
    r'(#(?P<fragment>\S+))?'
)


class Requirement:
    def __init__(self, req_line):
        self.line = req_line
        self.name = None
        self._constraint = None
        self._version = None
        self._revision = None
        self.fragment = None

        matches = re.match(NAME_REQ_REGEX, self.line)
        if matches:
            self.name = matches["name"]

        matches = re.match(NAME_VERSION_REQ_REGEX, self.line)
        if matches:
            self.name = matches["name"]
            self._constraint = matches['constraint']
            self._version = matches['version']

        matches = re.match(VCS_REGEX, self.line)
        if matches:
            self.name = f"{matches['scheme']}://{matches['path']}"
            self._revision = matches['revision']
            self.fragment = matches['fragment']

    def __repr__(self):
        return self.line

    @property
    def version(self):
        return self._version

    @version.setter
    def version(self, new_value):
        if new_value is None and self.version:
            self.line = self.line.replace(self.version, "")
        else:
            self.line = self.line.replace(self.version, new_value)
        self._version = new_value

    @property
    def revision(self):
        return self._revision

    @revision.setter
    def revision(self, new_value):
        if self.revision:
            self.line = self.line.replace(self.revision, new_value)
        elif self.fragment:
            version_and_fragment = f"@{new_value}#{self.fragment}"
            self.line = self.line.replace(f"#{self.fragment}", version_and_fragment)
        elif new_value:
            self.line += f"@{new_value}"

        self._revision = new_value

    @property
    def constraint(self):
        return self._constraint

    @constraint.setter
    def constraint(self, new_value):
        if self.constraint:
            self.line = self.line.replace(self.constraint, new_value if new_value else "")
        else:
            self.line = self.line.replace(self.name, new_value)
        self._constraint = new_value


def parse_requirements(raw_req):
    return [parse_requirement(req_line) for req_line in raw_req.splitlines()]


def parse_requirement(line):
    # if line.startswith("#"):
    #     return line

    return Requirement(line)
