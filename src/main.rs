extern crate git2;

use git2::{Repository, FetchOptions, RemoteCallbacks};

fn main() -> Result<(), git2::Error> {
    let repo_url = "https://github.com/Thepigcat76/citadel";
    let target_dir = "C:/Users/Admin/rust/nexus/matrix/git_test_target";

    let repo = Repository::open(target_dir)?;

    fn fetch_origin_main(repo: git2::Repository) -> Result<(), git2::Error> {
        repo.find_remote("origin")?.fetch(&["master"], None, None)
    }
    
    fetch_origin_main(repo).unwrap();

    Ok(())
}
