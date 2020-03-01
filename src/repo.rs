use git2::Commit;
use git2::Oid;
use git2::Repository;

pub trait RepositoryExt {
    fn walk_history<C, T>(&self, commit_id: Oid, callback: C) -> Option<T>
    where
        C: Fn(Oid) -> Option<T>;

    fn iter_commit_id<'repo>(&'repo self, commit: &'repo Commit) -> CommitIdIter<'repo>;
}

impl RepositoryExt for Repository {
    fn walk_history<C, T>(&self, commit_id: Oid, callback: C) -> Option<T>
    where
        C: Fn(Oid) -> Option<T>,
    {
        if let Some(result) = callback(commit_id) {
            Some(result)
        } else {
            let parent_id = self.find_commit(commit_id).ok()?.parent(0).ok()?.id();
            self.walk_history(parent_id, callback)
        }
    }
    fn iter_commit_id<'repo>(&'repo self, commit: &'repo Commit) -> CommitIdIter<'repo> {
        CommitIdIter {
            repo: self,
            started: false,
            cursor: commit.id(),
        }
    }
}

pub struct CommitIdIter<'a> {
    repo: &'a Repository,
    started: bool,
    cursor: Oid,
}

impl<'a> Iterator for CommitIdIter<'a> {
    type Item = Oid;
    fn next(&mut self) -> Option<Oid> {
        if !self.started {
            self.started = true;
            Some(self.cursor)
        } else {
            let current = self.repo.find_commit(self.cursor).ok()?;
            if let Ok(next_commit) = current.parent(0) {
                self.cursor = next_commit.id();
                Some(self.cursor)
            } else {
                None
            }
        }
    }
}
