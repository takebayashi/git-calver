use git2::Error;
use git2::Oid;
use git2::Repository;

use crate::calver::CalVer;
use crate::repo_release::Release;
use crate::repo_release::RepositoryWithRelease;

pub struct Releaser<'a> {
    repo: &'a Repository,
}

impl<'a> Releaser<'a> {
    pub fn new(repo: &Repository) -> Releaser {
        Releaser { repo }
    }

    pub fn last_release(&self) -> Release {
        self.repo
            .find_latest_release()
            .unwrap_or_else(Release::zero)
    }

    pub fn next_version(&self) -> CalVer {
        self.last_release().version.next_version()
    }

    pub fn is_releasable(&self) -> bool {
        let last_id = self.last_release().commit_id;
        let head_commit = self
            .repo
            .head()
            .ok()
            .and_then(|head| head.peel_to_commit().ok());
        if let Some(commit) = head_commit {
            commit.id() != last_id || last_id == Oid::zero()
        } else {
            false
        }
    }

    pub fn bump(&self, message: &str, lightweight: bool) -> Result<CalVer, Error> {
        let v = self.next_version();
        self.bump_to(v, message, lightweight)
    }

    pub fn bump_to(
        &self,
        version: CalVer,
        message: &str,
        lightweight: bool,
    ) -> Result<CalVer, Error> {
        let result = if lightweight {
            self.repo.tag_lightweight(
                format!("{}", version).as_str(),
                self.repo.head()?.peel_to_commit()?.as_object(),
                false,
            )
        } else {
            self.repo.tag(
                format!("{}", version).as_str(),
                self.repo.head()?.peel_to_commit()?.as_object(),
                &self.repo.signature()?,
                message,
                false,
            )
        };
        match result {
            Ok(_) => Ok(version),
            Err(err) => Err(err),
        }
    }
}
