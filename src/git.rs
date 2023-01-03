// This is a git adapter for the git2 crate. it is used to create a git branch
// in the current repository

use git2::Repository;

fn git_repository_open_from_workdir() -> Repository {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    return repo;
}

// Create a git branch in the current repository
pub fn branch_create(branch_name: std::string::String) {
    let repo = git_repository_open_from_workdir();
    let head = repo.head().unwrap();
    let head_commit = head.peel_to_commit().unwrap();

    let branch = repo.branch(&branch_name, &head_commit, false).unwrap();
    let branch_ref = branch.into_reference();
    let branch_ref_name = branch_ref.name().unwrap();

    repo.set_head(branch_ref_name).unwrap();
    repo.checkout_head(Some(git2::build::CheckoutBuilder::default().safe()))
        .unwrap();
}
