use crate::Msg;

use seed::{prelude::*, *};

pub fn icon_link(icon_group: &str, icon_name: &str, url: &str) -> Node<Msg> {
    a![
        C!["link"],
        attrs! { At::Href => url, At::Target => "_blank", },
        icon(icon_group, icon_name),
    ]
}

pub fn icon(group: &str, name: &str) -> Node<Msg> {
    i![C!["icon", format!("fa-{group}"), format!("fa-{name}")]]
}
