use enum_iterator::Sequence;
use seed::{prelude::*, *};

use crate::{
    data::{Project, RepoOwner},
    utils::icon_link,
    Model, Msg,
};

const PROJECTS: &[Project] = {
    let this_website = Project {
        owner: RepoOwner::Nonk,
        repo: "nonk.dev",
        thumbnail: "/static/logo.png",
        title: "This Website",
        description: "Not much to say about it except that it's made in Rust. It looks great though. (MFW the website looks like \u{1f4a9})",
    };

    let schwungus = Project {
        owner: RepoOwner::SchwungusSoftware,
        repo: "schwungus.software",
        thumbnail: "/static/schwung.png",
        title: "Schwungus Software",
        description: r#"I contributed to the landing page and self-hosted it."#,
    };

    let catspeak_compiler = Project {
        owner: RepoOwner::SchwungusSoftware,
        repo: "catspeak-compiler",
        thumbnail: "https://github.com/katsaii/catspeak-lang/raw/main/catspeak-logo-dark.svg",
        title: "Catspeak Compiler",
        description: r#"A work-in-progress compiler for the GameMaker modding language Catspeak."#,
    };

    &[catspeak_compiler, this_website, schwungus]
};

#[derive(PartialEq, Eq, Sequence)]
pub enum Tab {
    Projects,
    Blog,
}

impl Tab {
    pub fn from_url(url: &mut Url) -> Self {
        match url.next_path_part() {
            Some("blog") => Self::Blog,
            _ => Self::Projects, // the default tab
        }
    }

    pub fn href(&self) -> &'static str {
        match self {
            Self::Projects => "/projects",
            Self::Blog => "/blog",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Projects => "My Projects",
            Self::Blog => "My Blog",
        }
    }

    pub fn view(&self, model: &Model) -> Vec<Node<Msg>> {
        match self {
            Self::Projects => {
                let projects: Vec<_> = PROJECTS.iter().map(project).collect();
                vec![div![id!("projects"), projects]]
            }
            Self::Blog => {
                vec![p!["Under construction!"]]
            }
        }
    }
}

fn project(project: &Project) -> Node<Msg> {
    article![
        C!["project"],
        div![
            C!["description"],
            span![
                icon_link("brands", "github", &project.repo_link()),
                h2![&project.title]
            ],
            p![&project.description],
        ],
        div![
            C!["thumbnail"],
            img![attrs! {
                At::Src => project.thumbnail,
                "loading" => "lazy",
            }]
        ],
    ]
}
