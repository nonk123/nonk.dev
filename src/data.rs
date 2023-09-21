pub struct Project {
    pub owner: RepoOwner,
    pub repo: String,
    pub title: String,
    pub description: String,
    pub thumbnail: String,
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
