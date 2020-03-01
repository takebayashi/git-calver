use chrono::prelude::Datelike;
use chrono::prelude::Local;
use chrono::DateTime;
use git2::Error;
use git2::Oid;
use git2::Repository;

use crate::calver::CalVer;
use crate::calver::ToCalVer;
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
        self.next_version_of(self.last_release().version)
    }

    pub fn next_version_of(&self, current: CalVer) -> CalVer {
        let t: DateTime<Local> = Local::now();
        let date_ver = format!("{}.{}", t.year() % 100, t.month());
        let mut i = 0;
        loop {
            let v = format!("{}.{}", date_ver, i)
                .calver()
                .unwrap_or_else(|| CalVer::new(i));
            if v > current {
                return v;
            }
            i += 1;
        }
    }
    pub fn is_releasable(&self) -> bool {
        let last_id = self.last_release().commit_id;
        let head_commit = self
            .repo
            .head()
            .ok()
            .map(|head| head.peel_to_commit().ok())
            .flatten();
        if let Some(commit) = head_commit {
            commit.id() != last_id || last_id == Oid::zero()
        } else {
            false
        }
    }

    pub fn bump(&self, message: &str) -> Result<CalVer, Error> {
        let v = self.next_version();
        self.bump_to(v, message)
    }

    pub fn bump_to(&self, version: CalVer, message: &str) -> Result<CalVer, Error> {
        match self.repo.tag(
            format!("{}", version).as_str(),
            self.repo.head()?.peel_to_commit()?.as_object(),
            &self.repo.signature()?,
            message,
            false,
        ) {
            Ok(_) => Ok(version),
            Err(err) => Err(err),
        }
    }
}
