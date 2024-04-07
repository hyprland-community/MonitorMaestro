#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        app::tui::App,
        workspaces::{Monitor, State, WorkSpace},
    };

    #[test]
    fn serde_app() {
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
}
