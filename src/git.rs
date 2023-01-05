// This is a git adapter for the git2 crate. it is used to create a git branch
// in the current repository

use error_stack::{Report, Result};
use git2::Repository;
use std::fmt;
use std::path::PathBuf;
use std::{error::Error, fmt::Display};

// Errors that can occur when creating a branch
#[derive(Debug)]
pub enum BranchError {
    NotAGitRepository,
    BranchAlreadyExists,
    BranchCreationError(git2::Error),
}

impl Display for BranchError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BranchError::NotAGitRepository => fmt.write_str("Not a git repository"),
            BranchError::BranchAlreadyExists => fmt.write_str("Branch already exists"),
            BranchError::BranchCreationError(e) => write!(fmt, "Error creating branch: {}", e),
        }
    }
}

impl Error for BranchError {}

// Find the root of the git repository
fn git_repository_open_from_workdir(path: PathBuf) -> Result<Repository, BranchError> {
    Repository::open(path).map_err(|_| Report::new(BranchError::NotAGitRepository))
}

// Create a git branch in the current repository
pub fn branch_create(repo_path: Option<PathBuf>, branch_name: String) -> Result<(), BranchError> {
    let path = repo_path.unwrap_or_else(|| PathBuf::from("."));
    let repo = git_repository_open_from_workdir(path)?;
    let head = repo
        .head()
        .map_err(|e| Report::new(BranchError::BranchCreationError(e)))?;
    let head_commit = head
        .peel_to_commit()
        .map_err(|e| Report::new(BranchError::BranchCreationError(e)))?;

    let branch = repo
        .branch(&branch_name, &head_commit, false)
        .map_err(|e| {
            if e.code() == git2::ErrorCode::Exists {
                Report::new(BranchError::BranchAlreadyExists)
            } else {
                Report::new(BranchError::BranchCreationError(e))
            }
        })?;
    let branch_ref = branch.into_reference();
    let branch_ref_name = branch_ref.name().ok_or_else(|| {
        Report::new(BranchError::BranchCreationError(git2::Error::from_str(
            "Branch name is not valid UTF-8",
        )))
    })?;

    repo.set_head(branch_ref_name)
        .map_err(|e| Report::new(BranchError::BranchCreationError(e)))?;
    repo.checkout_head(Some(git2::build::CheckoutBuilder::default().safe()))
        .map_err(|e| Report::new(BranchError::BranchCreationError(e)))
}
