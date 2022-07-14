use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use gtk::glib;
use gtk::Align;

use crate::lib::wine::Group;
use super::wine_row::WineRow;

#[derive(Debug, Clone)]
pub struct WineGroup {
    pub group: Group,
    pub version_components: Vec<WineRow>,

    pub expander_row: adw::ExpanderRow
}

impl WineGroup {
    pub fn new(group: Group) -> Self {
        let expander_row = adw::ExpanderRow::new();

        expander_row.set_title(&group.title);
        expander_row.set_subtitle(group.subtitle.as_ref().unwrap_or(&String::new()));

        let mut version_components = Vec::new();

        for version in &group.runners {
            let component = WineRow::new(version.clone());

            expander_row.add_row(&component.row);

            version_components.push(component);
        }

        Self {
            group,
            version_components,
            expander_row
        }
    }
}
