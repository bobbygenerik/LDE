use std::collections::HashMap;

use crate::SettingValue;

pub const KEY_THEME: &str = "appearance.theme";
pub const KEY_ANIMATIONS: &str = "appearance.animations";
pub const KEY_SCALE: &str = "display.scale";
pub const KEY_TAP_TO_CLICK: &str = "input.tap_to_click";
pub const KEY_DOCK_AUTOHIDE: &str = "dock.autohide";

pub fn default_settings() -> HashMap<String, SettingValue> {
    let mut values = HashMap::new();
    values.insert(KEY_THEME.to_string(), SettingValue::Text("light".to_string()));
    values.insert(KEY_ANIMATIONS.to_string(), SettingValue::Bool(true));
    values.insert(KEY_SCALE.to_string(), SettingValue::Float(1.0));
    values.insert(KEY_TAP_TO_CLICK.to_string(), SettingValue::Bool(true));
    values.insert(KEY_DOCK_AUTOHIDE.to_string(), SettingValue::Bool(true));
    values
}
