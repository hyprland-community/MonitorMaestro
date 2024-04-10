#![allow(unused)]

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        app::tui::App,
        workspaces::{Monitor, State, WorkSpace},
    };

    #[test]
    fn serde_json_app() {
        let m1 = Monitor::new("eDP-1", State::Disabled);
        let ms1 = vec![m1];
        let mut workspaces = HashMap::<String, WorkSpace>::new();
        let w1 = WorkSpace::new("Normal", ms1);
        workspaces.insert("Normal".to_string(), w1);

        let app = App::new(workspaces);
        dbg!(&app.ws_names);

        let ser = serde_json::to_string(&app).unwrap();
        println!("{}", ser);

        let des: App = serde_json::from_str(&ser).unwrap();
        dbg!(des);
    }

    #[test]
    fn toml_app() {
        let m1 = Monitor::new("eDP-1", State::Enabled { dimensions: (2560, 1600), position: (0, 0), rerfresh_rate: 165, scaling: 1.25 });
        let m2 = Monitor::new("HDMI-A-1", State::Enabled { dimensions: (100, 100), position: (100, 5), rerfresh_rate: 60, scaling: 1.0 });
        let ms1 = vec![m1, m2];
        let mut workspaces = HashMap::<String, WorkSpace>::new();
        let w1 = WorkSpace::new("Normal", ms1);
        workspaces.insert("Normal".to_string(), w1);
        
        let m3 = Monitor::new("HDMI-A-2", State::Disabled);
        let ms2 = vec![m3];
        let w2 = WorkSpace::new("Off", ms2);
        workspaces.insert("Off".to_string(), w2);

        let app = App::new(workspaces);

        let ser = toml::to_string(&app).unwrap();
        println!("{}", ser);

        let app: App = toml::from_str(&ser).unwrap();
        dbg!(app);
    }
}
