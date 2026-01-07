use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use kevyt_shared::ipc::{DBUS_NAME_SETTINGS, DBUS_PATH_SETTINGS};
use kevyt_shared::settings::default_settings;
use kevyt_shared::SettingValue;
use zbus::blocking::Connection;
use zbus::interface;

#[derive(Default)]
struct SettingsStore {
    values: HashMap<String, SettingValue>,
}

struct SettingsService {
    store: Arc<Mutex<SettingsStore>>,
}

fn config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("kevyt");
    path.push("settings.json");
    path
}

fn load_store(path: &PathBuf) -> SettingsStore {
    let contents = fs::read_to_string(path).unwrap_or_default();
    let mut values: HashMap<String, SettingValue> = serde_json::from_str(&contents).unwrap_or_default();
    for (key, value) in default_settings() {
        values.entry(key).or_insert(value);
    }
    SettingsStore { values }
}

fn save_store(path: &PathBuf, store: &SettingsStore) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(data) = serde_json::to_string_pretty(&store.values) {
        let _ = fs::write(path, data);
    }
}

#[interface(name = "org.kevyt.Settings")]
impl SettingsService {
    fn get(&self, key: &str) -> SettingValue {
        let store = self.store.lock().expect("settings lock");
        store
            .values
            .get(key)
            .cloned()
            .unwrap_or(SettingValue::Text(String::new()))
    }

    fn set(&self, key: &str, value: SettingValue) {
        let mut store = self.store.lock().expect("settings lock");
        let changed = match store.values.get(key) {
            Some(existing) => existing != &value,
            None => true,
        };
        store.values.insert(key.to_string(), value);
        // Persist best-effort; avoid blocking the UI path.
        save_store(&config_path(), &store);
        if changed {
            let _ = self.changed(key, store.values.get(key).unwrap());
        }
    }

    fn list(&self) -> Vec<(String, SettingValue)> {
        let store = self.store.lock().expect("settings lock");
        store
            .values
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    #[signal]
    fn changed(&self, key: &str, value: &SettingValue) -> zbus::Result<()>;
}

fn main() -> zbus::Result<()> {
    let path = config_path();
    let store = Arc::new(Mutex::new(load_store(&path)));
    let service = SettingsService { store };

    let connection = Connection::session()?;
    connection.request_name(DBUS_NAME_SETTINGS)?;
    connection.object_server().at(DBUS_PATH_SETTINGS, service)?;

    println!("Kevyt settings daemon running on D-Bus");
    std::thread::park();
    Ok(())
}
