use std::vec::Vec;

use git2::Oid;
use git2::Repository;

use crate::calver::CalVer;
use crate::calver::ToCalVer;
use crate::repo::RepositoryExt;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Release {
    pub version: CalVer,
    pub commit_id: Oid,
}

impl Release {
    pub fn zero() -> Release {
        Release {
            commit_id: Oid::zero(),
            version: CalVer::zero(),
        }
    }
}

pub trait RepositoryWithRelease {
    fn find_releases(&self) -> Vec<Release>;
    fn find_latest_release(&self) -> Option<Release>;
}

impl RepositoryWithRelease for Repository {
    fn find_releases(&self) -> Vec<Release> {
        let tag_names = self
            .tag_names(None)
            .ok()
            .map(|a| {
                a.iter()
                    .flatten()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_else(|| vec![]);
        let all_releases = tag_names
            .iter()
            .flat_map(|t| t.calver())
            .flat_map(|v| {
                self.resolve_reference_from_short_name(v.to_string().as_str())
                    .ok()
                    .map(|r| r.peel_to_commit().ok())
                    .flatten()
                    .map(|c| Release {
                        commit_id: c.id(),
                        version: v,
                    })
            })
            .collect::<Vec<_>>();
        if let Some(head) = self.head().ok().map(|r| r.peel_to_commit().ok()).flatten() {
            let mut releases = self
                .iter_commit_id(&head)
                .flat_map(|cid| {
                    all_releases
                        .iter()
                        .filter(|r| cid == r.commit_id)
                        .map(|&r| r)
                        .next()
                })
                .collect::<Vec<_>>();
            releases.sort();
            releases.reverse();
            releases
        } else {
            vec![]
        }
    }

    fn find_latest_release(&self) -> Option<Release> {
        self.find_releases().into_iter().next()
    }
}
