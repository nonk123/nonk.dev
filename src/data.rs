pub struct Project {
    pub owner: RepoOwner,
    pub repo: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub thumbnail: &'static str,
}

pub enum RepoOwner {
    Nonk,
    SchwungusSoftware,
}

impl Project {
    pub fn repo_link(&self) -> String {
        let owner = match self.owner {
            RepoOwner::Nonk => "nonk123",
            RepoOwner::SchwungusSoftware => "Schwungus-Software",
        };

        let repo = &self.repo;

        format!("https://github.com/{owner}/{repo}")
    }
}
