use crate::differ::DiffOp;
use crate::requirement::Requirement;
use std::collections::HashMap;

/// Compare version strings, handling both semver and simple numeric versions
fn version_greater(a: &str, b: &str) -> bool {
    // Try semver first
    if let (Ok(va), Ok(vb)) = (semver::Version::parse(a), semver::Version::parse(b)) {
        return va > vb;
    }

    // Fall back to simple numeric comparison (e.g., "1.0" vs "2.0")
    let parse_parts = |s: &str| -> Vec<u64> {
        s.split('.')
            .filter_map(|p| p.parse().ok())
            .collect()
    };

    let parts_a = parse_parts(a);
    let parts_b = parse_parts(b);

    for (a, b) in parts_a.iter().zip(parts_b.iter()) {
        match a.cmp(b) {
            std::cmp::Ordering::Greater => return true,
            std::cmp::Ordering::Less => return false,
            std::cmp::Ordering::Equal => continue,
        }
    }

    parts_a.len() > parts_b.len()
}

pub fn merge(current: &mut Vec<Requirement>, diff: Vec<DiffOp>) {
    let index: HashMap<String, usize> = current
        .iter()
        .enumerate()
        .filter_map(|(i, r)| r.name.clone().map(|n| (n, i)))
        .collect();

    let mut to_remove: Vec<String> = Vec::new();
    let mut to_add: Vec<Requirement> = Vec::new();

    for op in diff {
        match op {
            DiffOp::Update(new_req) => {
                let Some(name) = &new_req.name else { continue };
                let Some(&idx) = index.get(name) else {
                    continue;
                };

                let current_req = &mut current[idx];
                apply_update(current_req, &new_req);
            }
            DiffOp::Add(req) => {
                if let Some(name) = &req.name {
                    if !index.contains_key(name) {
                        to_add.push(req);
                    }
                }
            }
            DiffOp::Remove(name) => {
                to_remove.push(name);
            }
        }
    }

    // Remove dependencies
    current.retain(|r| {
        r.name
            .as_ref()
            .map(|n| !to_remove.contains(n))
            .unwrap_or(true)
    });

    // Add new dependencies
    current.extend(to_add);
}

fn apply_update(current: &mut Requirement, new: &Requirement) {
    // Update constraint if present in new
    if let Some(new_constraint) = &new.constraint {
        current.set_constraint(Some(new_constraint));
    }

    // Handle revision (VCS)
    if let Some(new_rev) = &new.revision {
        // Only update if new revision is greater
        let should_update = current
            .revision
            .as_ref()
            .map(|cur| version_greater(new_rev, cur))
            .unwrap_or(true);

        if should_update {
            current.set_revision(Some(new_rev));
        }
    }
    // Handle version
    else if let Some(new_ver) = &new.version {
        let should_update = current
            .version
            .as_ref()
            .map(|cur| version_greater(new_ver, cur))
            .unwrap_or(true);

        if should_update {
            current.set_version(Some(new_ver));
        }
    }
    // New has no version/revision - remove constraint
    else {
        current.set_constraint(None);
        current.set_revision(None);
        current.set_version(None);
    }
}

pub fn format_requirements(reqs: &[Requirement]) -> String {
    reqs.iter()
        .map(|r| r.line.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::differ::differ;
    use crate::requirement::parse_requirements;

    fn merge_test(base: &str, other: &str) -> String {
        let base_reqs = parse_requirements(base);
        let other_reqs = parse_requirements(other);
        let mut current = parse_requirements(base);

        let diff = differ(&base_reqs, &other_reqs);
        merge(&mut current, diff);
        format_requirements(&current)
    }

    #[test]
    fn test_add_dep() {
        assert_eq!(merge_test("", "new-dep"), "new-dep");
    }

    #[test]
    fn test_remove_dep() {
        assert_eq!(merge_test("new-dep", ""), "");
    }

    #[test]
    fn test_upgrade_version() {
        assert_eq!(merge_test("new-dep==0.1", "new-dep==0.2"), "new-dep==0.2");
    }

    #[test]
    fn test_keep_higher_version() {
        // Downgrade attempt should keep higher version
        assert_eq!(merge_test("new-dep==0.2", "new-dep==0.1"), "new-dep==0.2");
    }

    #[test]
    fn test_change_constraint() {
        assert_eq!(merge_test("new-dep<=0.1", "new-dep==0.2"), "new-dep==0.2");
    }

    #[test]
    fn test_remove_version() {
        assert_eq!(merge_test("new-dep==0.1", "new-dep"), "new-dep");
    }

    #[test]
    fn test_three_part_version() {
        assert_eq!(merge_test("dep==1.1.2", "dep==1.1.0"), "dep==1.1.2");
    }

    #[test]
    fn test_preserve_comments() {
        assert_eq!(
            merge_test("# comment\ndep==1 # c2", "# comment\ndep==2 # c2"),
            "# comment\ndep==2 # c2"
        );
    }

    #[test]
    fn test_extras() {
        assert_eq!(
            merge_test("dep[opt]==0.1", "dep[opt]==0.2"),
            "dep[opt]==0.2"
        );
    }

    #[test]
    fn test_vcs_revision() {
        assert_eq!(
            merge_test(
                "git+ssh://git@url.git#egg=fragment",
                "git+ssh://git@url.git@1.0#egg=fragment"
            ),
            "git+ssh://git@url.git@1.0#egg=fragment"
        );
    }
}
