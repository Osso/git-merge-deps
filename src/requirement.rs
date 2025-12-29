use regex::Regex;
use std::sync::LazyLock;

static NAME_VERSION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<name>[\w_-]+(?:\[\w+\])?) *(?P<constraint>[<>=]=) *(?P<version>[\d.]+)")
        .unwrap()
});

static VCS_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?P<scheme>git|git\+https|git\+ssh|git\+git)://(?:(?P<login>[^/@]+)@)?(?P<path>[^#@]+)(?:@(?P<revision>[^#]+))?(?:#(?P<fragment>\S+))?",
    )
    .unwrap()
});

static NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?P<name>[\w_-]+)").unwrap());

#[derive(Debug, Clone)]
pub struct Requirement {
    pub line: String,
    pub name: Option<String>,
    pub constraint: Option<String>,
    pub version: Option<String>,
    pub revision: Option<String>,
    pub fragment: Option<String>,
}

impl Requirement {
    pub fn parse(line: &str) -> Self {
        let mut req = Requirement {
            line: line.to_string(),
            name: None,
            constraint: None,
            version: None,
            revision: None,
            fragment: None,
        };

        // Try VCS URL first
        if let Some(caps) = VCS_REGEX.captures(line) {
            let scheme = caps.name("scheme").unwrap().as_str();
            let path = caps.name("path").unwrap().as_str();
            req.name = Some(format!("{scheme}://{path}"));
            req.revision = caps.name("revision").map(|m| m.as_str().to_string());
            req.fragment = caps.name("fragment").map(|m| m.as_str().to_string());
            return req;
        }

        // Try versioned requirement
        if let Some(caps) = NAME_VERSION_REGEX.captures(line) {
            req.name = Some(caps.name("name").unwrap().as_str().to_string());
            req.constraint = Some(caps.name("constraint").unwrap().as_str().to_string());
            req.version = Some(caps.name("version").unwrap().as_str().to_string());
            return req;
        }

        // Try simple name
        if let Some(caps) = NAME_REGEX.captures(line) {
            req.name = Some(caps.name("name").unwrap().as_str().to_string());
        }

        req
    }

    pub fn set_version(&mut self, new_version: Option<&str>) {
        match (&self.version, new_version) {
            (Some(old), Some(new)) => {
                self.line = self.line.replace(old, new);
                self.version = Some(new.to_string());
            }
            (Some(old), None) => {
                self.line = self.line.replace(old, "");
                self.version = None;
            }
            (None, Some(new)) => {
                self.line.push_str(new);
                self.version = Some(new.to_string());
            }
            (None, None) => {}
        }
    }

    pub fn set_revision(&mut self, new_revision: Option<&str>) {
        match (&self.revision, &self.fragment, new_revision) {
            (Some(old), _, Some(new)) => {
                self.line = self.line.replace(old, new);
                self.revision = Some(new.to_string());
            }
            (None, Some(frag), Some(new)) => {
                let old_frag = format!("#{frag}");
                let new_part = format!("@{new}#{frag}");
                self.line = self.line.replace(&old_frag, &new_part);
                self.revision = Some(new.to_string());
            }
            (None, None, Some(new)) => {
                self.line.push_str(&format!("@{new}"));
                self.revision = Some(new.to_string());
            }
            (_, _, None) => {
                self.revision = None;
            }
        }
    }

    pub fn set_constraint(&mut self, new_constraint: Option<&str>) {
        match (&self.constraint, new_constraint) {
            (Some(old), Some(new)) => {
                self.line = self.line.replace(old, new);
                self.constraint = Some(new.to_string());
            }
            (Some(old), None) => {
                self.line = self.line.replace(old, "");
                self.constraint = None;
            }
            (None, Some(new)) => {
                self.line.push_str(new);
                self.constraint = Some(new.to_string());
            }
            (None, None) => {}
        }
    }
}

pub fn parse_requirements(content: &str) -> Vec<Requirement> {
    content.lines().map(Requirement::parse).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple() {
        let req = Requirement::parse("requests");
        assert_eq!(req.name.as_deref(), Some("requests"));
        assert_eq!(req.version, None);
    }

    #[test]
    fn parse_versioned() {
        let req = Requirement::parse("requests==2.28.0");
        assert_eq!(req.name.as_deref(), Some("requests"));
        assert_eq!(req.constraint.as_deref(), Some("=="));
        assert_eq!(req.version.as_deref(), Some("2.28.0"));
    }

    #[test]
    fn parse_with_extras() {
        let req = Requirement::parse("requests[security]==2.28.0");
        assert_eq!(req.name.as_deref(), Some("requests[security]"));
        assert_eq!(req.version.as_deref(), Some("2.28.0"));
    }

    #[test]
    fn parse_vcs() {
        let req = Requirement::parse("git+ssh://git@github.com/user/repo.git@1.0.0#egg=mypackage");
        assert_eq!(
            req.name.as_deref(),
            Some("git+ssh://github.com/user/repo.git")
        );
        assert_eq!(req.revision.as_deref(), Some("1.0.0"));
        assert_eq!(req.fragment.as_deref(), Some("egg=mypackage"));
    }

    #[test]
    fn parse_vcs_no_revision() {
        let req = Requirement::parse("git+ssh://git@github.com/user/repo.git#egg=mypackage");
        assert_eq!(
            req.name.as_deref(),
            Some("git+ssh://github.com/user/repo.git")
        );
        assert_eq!(req.revision, None);
        assert_eq!(req.fragment.as_deref(), Some("egg=mypackage"));
    }

    #[test]
    fn set_version() {
        let mut req = Requirement::parse("requests==2.28.0");
        req.set_version(Some("2.29.0"));
        assert_eq!(req.line, "requests==2.29.0");
        assert_eq!(req.version.as_deref(), Some("2.29.0"));
    }

    #[test]
    fn set_revision() {
        let mut req = Requirement::parse("git+ssh://git@url.git#egg=fragment");
        req.set_revision(Some("1.0"));
        assert_eq!(req.line, "git+ssh://git@url.git@1.0#egg=fragment");
        assert_eq!(req.revision.as_deref(), Some("1.0"));
    }
}
