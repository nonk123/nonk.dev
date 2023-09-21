#[macro_use]
extern crate log;

use data::{Project, RepoOwner};
use seed::{prelude::*, *};

mod data;

fn main() -> eyre::Result<()> {
    console_log::init()?;
    console_error_panic_hook::set_once();

    info!("Hey, no peeking!"); // TODO: issue a warning instead and only when opening the dev console
    App::start("root", init, update, view);

    Ok(())
}

pub struct Model {
    projects: Vec<Project>,
}

pub enum Msg {}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    let this_website = Project {
        owner: RepoOwner::Nonk,
        repo: "nonk.dev".to_string(),
        thumbnail: "/static/logo.png".to_string(),
        title: "This Website".to_string(),
        description:
            r#"Not much to say about it except that it's made in Rust. It looks great though."#
                .to_string(),
    };

    let schwungus = Project {
        owner: RepoOwner::SchwungusSoftware,
        repo: "schwungus.software".to_string(),
        thumbnail: "/static/schwung.png".to_string(),
        title: "Schwungus Software Homepage".to_string(),
        description: r#"Comes with a landing page and a whole documentation section!"#.to_string(),
    };

    let catspeak_compiler = Project {
        owner: RepoOwner::SchwungusSoftware,
        repo: "catspeak-compiler".to_string(),
        thumbnail: "https://github.com/katsaii/catspeak-lang/raw/main/catspeak-logo-dark.svg".to_string(),
        title: "Catspeak Compiler".to_string(),
        description: r#"A work-in-progress compiler for the Catspeak modding language. Compiles to Catspeak IR, which can be loaded into your project to obfuscate production code and skip directly to codegen."#.to_string(),
    };

    let projects = vec![this_website, schwungus, catspeak_compiler];

    Model { projects }
}

fn update(_: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {}

fn view(model: &Model) -> impl IntoNodes<Msg> {
    let projects: Vec<_> = model.projects.iter().map(project).collect();

    vec![
        div![C!["background"]],
        nav![
            div![
                img![
                    attrs! {
                        At::Id => "logo",
                        At::Src => "/static/logo.png",
                        At::Alt => "nonk's meh face",
                    }
                ],
            ],
            div![
                id!("links"),
                link("brands", "github", "nonk123", "https://github.com/nonk123"),
                link("brands", "discord", "adhfjlkadshfljk", "https://discord.com/users/268677450144153611"),
                link("regular", "envelope", "me@nonk.dev", "mailto:me@nonk.dev"),
            ],
            div![
                p!["Hi! I'm nonk."],
                p!["I spend too much time exploring questionable ideas and implementing them in Rust."],
                p!["Feel free to check out my projects."],
                p!["By the way, I am part of ", a![attrs! { At::Href => "https://schwungus.software" }, "Schwungus Software"], "."],
            ],
        ],
        main![
            h1!["My Projects"],
            div![
                projects
            ],
        ],
    ]
}

fn project(project: &Project) -> Node<Msg> {
    article![
        C!["project"],
        div![
            C!["description"],
            span![
                a![
                    C!["link"],
                    attrs! { At::Href => project.repo_link() },
                    icon("brands", "github")
                ],
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

fn link(icon_group: &str, icon_name: &str, caption: &str, url: &str) -> Node<Msg> {
    a![
        C!["link"],
        attrs! { At::Href => url },
        icon(icon_group, icon_name),
        " ",
        span![caption],
    ]
}

fn icon(group: &str, name: &str) -> Node<Msg> {
    i![C!["icon", format!("fa-{group}"), format!("fa-{name}")]]
}
