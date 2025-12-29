use crate::requirement::Requirement;
use std::collections::HashMap;

#[derive(Debug)]
pub enum DiffOp {
    Add(Requirement),
    Remove(String), // name only
    Update(Requirement),
}

pub fn differ(old: &[Requirement], new: &[Requirement]) -> Vec<DiffOp> {
    let old_index = index_requirements(old);
    let new_index = index_requirements(new);
    let mut diff = Vec::new();

    // Find additions and updates
    for req in new {
        let Some(name) = &req.name else { continue };

        if let Some(old_req) = old_index.get(name.as_str()) {
            if req.line != old_req.line {
                diff.push(DiffOp::Update(req.clone()));
            }
        } else {
            diff.push(DiffOp::Add(req.clone()));
        }
    }

    // Find removals
    for req in old {
        let Some(name) = &req.name else { continue };

        if !new_index.contains_key(name.as_str()) {
            diff.push(DiffOp::Remove(name.clone()));
        }
    }

    diff
}

fn index_requirements(reqs: &[Requirement]) -> HashMap<String, &Requirement> {
    reqs.iter()
        .filter_map(|r| r.name.clone().map(|n| (n, r)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::requirement::parse_requirements;

    #[test]
    fn test_add() {
        let old = parse_requirements("");
        let new = parse_requirements("new-dep");
        let diff = differ(&old, &new);

        assert_eq!(diff.len(), 1);
        assert!(matches!(&diff[0], DiffOp::Add(r) if r.name.as_deref() == Some("new-dep")));
    }

    #[test]
    fn test_remove() {
        let old = parse_requirements("old-dep");
        let new = parse_requirements("");
        let diff = differ(&old, &new);

        assert_eq!(diff.len(), 1);
        assert!(matches!(&diff[0], DiffOp::Remove(name) if name == "old-dep"));
    }

    #[test]
    fn test_update() {
        let old = parse_requirements("dep==1.0");
        let new = parse_requirements("dep==2.0");
        let diff = differ(&old, &new);

        assert_eq!(diff.len(), 1);
        assert!(matches!(&diff[0], DiffOp::Update(r) if r.version.as_deref() == Some("2.0")));
    }

    #[test]
    fn test_no_change() {
        let old = parse_requirements("dep==1.0");
        let new = parse_requirements("dep==1.0");
        let diff = differ(&old, &new);

        assert!(diff.is_empty());
    }
}
