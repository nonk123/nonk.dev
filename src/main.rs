#[macro_use]
extern crate log;

use seed::{prelude::*, *};
use tab::Tab;

use crate::utils::icon_link;

mod data;
mod tab;
mod utils;

fn main() -> eyre::Result<()> {
    console_log::init()?;
    console_error_panic_hook::set_once();

    info!("Hey, no peeking!"); // TODO: issue a warning instead and only when opening the dev console
    App::start("root", init, update, view);

    Ok(())
}

pub struct Model {
    current_tab: Tab,
}

pub enum Msg {
    UrlChanged(subs::UrlChanged),
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .subscribe(Msg::UrlChanged)
        .notify(subs::UrlChanged(url));

    Model {
        current_tab: Tab::Projects,
    }
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(mut url)) => {
            model.current_tab = Tab::from_url(&mut url);
        }
    }
}

fn view(model: &Model) -> impl IntoNodes<Msg> {
    let tabs: Vec<_> = enum_iterator::all::<Tab>()
        .map(|tab| {
            a![
                C![IF!(tab == model.current_tab => "selected")],
                attrs! { At::Href => tab.href() },
                tab.name()
            ]
        })
        .collect();

    vec![
        div![C!["background"]],
        nav![
            div![img![attrs! {
                At::Id => "logo",
                At::Src => "/static/logo.png",
                At::Alt => "nonk's meh face",
            }],],
            div![
                p!["Hi! I'm nonk."],
                p!["I love exploring unconventional ideas and implementing them in Rust."],
                p!["Feel free to check out my projects."],
                p![
                    "By the way, I am part of ",
                    a![
                        attrs! { At::Href => "https://schwungus.software" },
                        "Schwungus Software"
                    ],
                    "."
                ],
            ],
            div![
                id!("links"),
                icon_link("brands", "github", "https://github.com/nonk123"),
                icon_link(
                    "brands",
                    "discord",
                    "https://discord.com/users/268677450144153611"
                ),
                icon_link("regular", "envelope", "mailto:me@nonk.dev"),
            ],
        ],
        div![
            id!("container"),
            div![id!("tabs"), tabs],
            main![model.current_tab.view(model)],
        ],
    ]
}
