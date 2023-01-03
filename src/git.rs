// This is a git adapter for the git2 crate. it is used to create a git branch
// in the current repository

use git2::Repository;

fn git_repository_open_from_workdir(path: &str) -> Repository {
    match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    }
}

// Create a git branch in the current repository
pub fn branch_create(repo_path: Option<&str>, branch_name: std::string::String) {
    let path = repo_path.unwrap_or(".");
    let repo = git_repository_open_from_workdir(path);
    let head = repo.head().unwrap();
    let head_commit = head.peel_to_commit().unwrap();

    let branch = repo.branch(&branch_name, &head_commit, false).unwrap();
    let branch_ref = branch.into_reference();
    let branch_ref_name = branch_ref.name().unwrap();

    repo.set_head(branch_ref_name).unwrap();
    repo.checkout_head(Some(git2::build::CheckoutBuilder::default().safe()))
        .unwrap();
}
