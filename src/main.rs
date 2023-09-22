#[macro_use]
extern crate log;

use rand::prelude::*;
use seed::{prelude::*, *};
use tab::Tab;

use crate::utils::icon_link;

mod data;
mod tab;
mod utils;

fn main() -> eyre::Result<()> {
    console_error_panic_hook::set_once();
    console_log::init()?;

    warn!("Hey, no peeking!");
    App::start("root", init, update, view);

    Ok(())
}

pub struct Model {
    current_tab: Tab,
    anger: f64,
    mouse_inside_logo: bool,
    angry_face: ElRef<web_sys::HtmlImageElement>,
    anger_last_shown: f64,
}

impl Model {
    fn render_angry_face(&mut self, render_info: &RenderInfo) {
        let dt = render_info.timestamp_delta.unwrap_or(0.0) / 1000.0;

        // TODO: replace magic numbers with constants.
        if self.mouse_inside_logo {
            self.anger += 0.55 * dt
        } else {
            self.anger -= 0.83 * dt;
        }

        // TODO: move limits to dedicated constants.
        self.anger = self.anger.clamp(0.5, 5.0);

        let elapsed = render_info.timestamp - self.anger_last_shown;
        let delay = 300.0 / self.anger as f64;

        if elapsed >= delay {
            self.anger_last_shown = render_info.timestamp;
        } else {
            return;
        }

        self.angry_face.get().map(|angry_face| {
            let direction = (rand::thread_rng().gen::<f64>() - 0.5) * 2.0;

            let distance = rand::thread_rng().gen::<f64>().max(0.4);
            let distance = distance * 50.0;

            let left = direction * distance;
            let top = (1.0 - direction.powi(2)) * distance;

            // TODO: use the CSS description struct.
            let style = format!(
                r#"
left: {left}px;
top: {top}px;
transition: left {delay}ms, top {delay}ms;"#
            );

            angry_face.set_attribute("style", &style).unwrap();
        });
    }
}

pub enum Msg {
    UrlChanged(subs::UrlChanged),
    Render(RenderInfo),
    MouseEnteredLogo,
    MouseLeftLogo,
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .subscribe(Msg::UrlChanged)
        .notify(subs::UrlChanged(url));

    orders.after_next_render(Msg::Render);

    Model {
        current_tab: Tab::Projects,
        anger: 0.0,
        mouse_inside_logo: false,
        angry_face: Default::default(),
        anger_last_shown: 0.0,
    }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(mut url)) => {
            model.current_tab = Tab::from_url(&mut url);
        }
        Msg::Render(render_info) => {
            model.render_angry_face(&render_info);
            orders.after_next_render(Msg::Render);
        }
        Msg::MouseEnteredLogo => {
            model.mouse_inside_logo = true;
        }
        Msg::MouseLeftLogo => {
            model.mouse_inside_logo = false;
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
        div![id!("background")],
        nav![
            div![img![
                el_ref(&model.angry_face),
                mouse_ev(Ev::MouseEnter, |_| Msg::MouseEnteredLogo),
                mouse_ev(Ev::MouseLeave, |_| Msg::MouseLeftLogo),
                attrs! {
                    At::Id => "logo",
                    At::Src => "/static/logo.png",
                    At::Alt => "nonk's meh face",
                    At::Draggable => false,
                }
            ],],
            div![
                id!("bio"),
                p!["Hi! I'm nonk."],
                p!["I love exploring unconventional ideas and implementing them in Rust."],
                p!["Feel free to check out my projects."],
                p![
                    "By the way, I am part of ",
                    a![
                        attrs! {
                            At::Href => "https://schwungus.software",
                            At::Target => "_blank",
                        },
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
