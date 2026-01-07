#[derive(Clone)]
pub struct AppEntry {
    pub name: String,
    pub exec: String,
}

pub fn default_apps() -> Vec<AppEntry> {
    vec![
        AppEntry {
            name: "Files".to_string(),
            exec: "nautilus".to_string(),
        },
        AppEntry {
            name: "Terminal".to_string(),
            exec: "gnome-terminal".to_string(),
        },
        AppEntry {
            name: "Web".to_string(),
            exec: "firefox".to_string(),
        },
    ]
}
