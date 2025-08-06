use std::error::Error;
use git2::{Repository, Oid, Revwalk};
use crate::cli::Args;

pub struct CommitIterator<'a> {
    revwalk: Revwalk<'a>,
    repo: &'a Repository,
    total_commits: usize,
    commits_processed_in_page: usize,
    args: &'a Args,
}

impl<'a> CommitIterator<'a> {
    pub fn new(repo: &'a Repository, args: &'a Args, total_commits: usize) -> Result<Self, Box<dyn Error>> {
        let mut revwalk = repo.revwalk()?;

        if let Some(start_commit_hash) = &args.start_commit {
            let oid = Oid::from_str(start_commit_hash)?;
            revwalk.push(oid)?;
        } else {
            revwalk.push_head()?;
        }

        Ok(Self {
            revwalk,
            repo,
            total_commits,
            commits_processed_in_page: 0,
            args,
        })
    }

    pub fn commits_processed(&self) -> usize {
        self.commits_processed_in_page
    }
}

impl<'a> Iterator for CommitIterator<'a> {
    type Item = Result<(git2::Commit<'a>, usize, f64), Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(limit) = self.args.commits_per_page {
            if self.commits_processed_in_page >= limit {
                return None;
            }
        }

        match self.revwalk.next() {
            Some(Ok(oid)) => {
                self.commits_processed_in_page += 1;
                let percentage = (self.commits_processed_in_page as f64 / self.total_commits as f64) * 100.0;
                let commit = self.repo.find_commit(oid);
                match commit {
                    Ok(c) => Some(Ok((c, self.commits_processed_in_page, percentage))),
                    Err(e) => Some(Err(Box::new(e))),
                }
            },
            Some(Err(e)) => Some(Err(Box::new(e))),
            None => None,
        }
    }
}