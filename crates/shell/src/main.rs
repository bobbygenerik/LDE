use gtk4::prelude::*;
use gtk4::glib;
use gtk4::glib::clone;
use kevyt_shared::ipc::{DBUS_NAME_SETTINGS, DBUS_PATH_SETTINGS};
use kevyt_shared::settings::{KEY_ANIMATIONS, KEY_DOCK_AUTOHIDE, KEY_THEME};
use kevyt_shared::SettingValue;
use zbus::blocking::Connection;
use chrono::Local;
use std::collections::HashMap;
use std::process::Command;
use std::fs;
use std::thread;
use std::time::Duration;

mod apps;

fn build_css(theme: &str) -> String {
    let (base, surface, text) = match theme {
        "dim" => ("#eeece7", "#f6f5f2", "#1f2328"),
        _ => ("#f6f5f2", "#ffffff", "#1f2328"),
    };
    format!(
        "\n        * {{ font-family: 'IBM Plex Sans', 'Source Sans 3', sans-serif; }}\n        .kevyt-panel {{ background: {base}; color: {text}; padding: 6px 12px; }}\n        .kevyt-panel .title {{ font-weight: 600; }}\n        .kevyt-panel entry {{ background: {surface}; border-radius: 8px; padding: 4px 8px; }}\n        .kevyt-dock {{ background: {surface}; border-radius: 12px; padding: 8px 10px; }}\n        .kevyt-dock button {{ background: #f9f8f6; border-radius: 10px; }}\n        \"
    )
}

fn fetch_settings(connection: &Connection) -> HashMap<String, SettingValue> {
    let reply = connection.call_method(
        Some(DBUS_NAME_SETTINGS),
        DBUS_PATH_SETTINGS,
        Some(DBUS_NAME_SETTINGS),
        "list",
        &(),
    );

    if let Ok(message) = reply {
        if let Ok(values) = message.body::<Vec<(String, SettingValue)>>() {
            return values.into_iter().collect();
        }
    }

    HashMap::new()
}

fn apply_settings(
    settings: &HashMap<String, SettingValue>,
    provider: &gtk4::CssProvider,
    dock: &gtk4::Box,
) {
    let theme = match settings.get(KEY_THEME) {
        Some(SettingValue::Text(value)) => value.as_str(),
        _ => "light",
    };

    let css = build_css(theme);
    provider.load_from_data(css.as_bytes());

    let _autohide = matches!(settings.get(KEY_DOCK_AUTOHIDE), Some(SettingValue::Bool(true)));
    dock.set_visible(true);
    let _ = settings.get(KEY_ANIMATIONS);
}

fn read_battery() -> String {
    let entries = match fs::read_dir("/sys/class/power_supply") {
        Ok(entries) => entries,
        Err(_) => return "AC".to_string(),
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let kind = fs::read_to_string(path.join("type")).unwrap_or_default();
        if kind.trim() != "Battery" {
            continue;
        }
        let capacity = fs::read_to_string(path.join("capacity")).unwrap_or_default();
        let status = fs::read_to_string(path.join("status")).unwrap_or_default();
        let cap = capacity.trim().to_string();
        let status = status.trim().to_string();
        if cap.is_empty() {
            return "BAT".to_string();
        }
        return format!("{cap}% {status}");
    }

    "AC".to_string()
}

fn read_network() -> String {
    let entries = match fs::read_dir("/sys/class/net") {
        Ok(entries) => entries,
        Err(_) => return "Net".to_string(),
    };

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name == "lo" {
            continue;
        }
        let state = fs::read_to_string(entry.path().join("operstate")).unwrap_or_default();
        if state.trim() == "up" {
            return format!("{name}");
        }
    }

    "Offline".to_string()
}

fn refresh_status(net: &gtk4::Label, bat: &gtk4::Label) {
    net.set_text(&read_network());
    bat.set_text(&read_battery());
}

fn clear_results(list: &gtk4::ListBox) {
    let mut child = list.first_child();
    while let Some(widget) = child {
        child = widget.next_sibling();
        if let Ok(row) = widget.downcast::<gtk4::ListBoxRow>() {
            list.remove(&row);
        }
    }
}

fn main() {
    let app = gtk4::Application::new(Some("org.kevyt.Shell"), Default::default());
    app.connect_activate(|app| {
        let launcher_window = gtk4::Window::builder()
            .application(app)
            .title("Launcher")
            .default_width(480)
            .default_height(240)
            .build();
        launcher_window.set_hide_on_close(true);
        let launcher_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
        launcher_box.set_margin_top(12);
        launcher_box.set_margin_bottom(12);
        launcher_box.set_margin_start(12);
        launcher_box.set_margin_end(12);
        let results = gtk4::ListBox::new();
        launcher_box.append(&results);
        launcher_window.set_child(Some(&launcher_box));

        let provider = gtk4::CssProvider::new();
        provider.load_from_data(build_css("light").as_bytes());
        gtk4::StyleContext::add_provider_for_display(
            &gtk4::gdk::Display::default().expect("display"),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let window = gtk4::ApplicationWindow::builder()
            .application(app)
            .title("Kevyt Shell")
            .default_width(1200)
            .default_height(120)
            .build();

        let root = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
        root.set_margin_top(8);
        root.set_margin_bottom(8);
        root.set_margin_start(12);
        root.set_margin_end(12);

        let panel = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        panel.add_css_class("kevyt-panel");
        panel.set_hexpand(true);

        let title = gtk4::Label::new(Some("Kevyt"));
        title.add_css_class("title");
        let launcher = gtk4::Entry::new();
        launcher.set_placeholder_text(Some("Search"));
        launcher.set_width_chars(20);
        let apps = apps::default_apps();
        let apps_clone = apps.clone();
        let update_results = clone!(@strong results, @strong launcher_window, @strong apps_clone => move |entry: &gtk4::Entry| {
            let query = entry.text().to_string().to_lowercase();
            clear_results(&results);

            for app in &apps_clone {
                if app.name.to_lowercase().contains(&query) {
                    let row = gtk4::ListBoxRow::new();
                    let label = gtk4::Label::new(Some(&app.name));
                    label.set_xalign(0.0);
                    row.set_child(Some(&label));
                    row.set_activatable(true);
                    row.set_selectable(false);
                    row.set_widget_name(&app.exec);
                    results.append(&row);
                }
            }
            launcher_window.present();
        });
        launcher.connect_changed(update_results.clone());
        launcher.connect_activate(move |entry| {
            update_results(entry);
        });

        results.connect_row_activated(clone!(@strong launcher_window => move |_, row| {
            let exec = row.widget_name();
            if !exec.is_empty() {
                let _ = Command::new(exec).spawn();
            }
            launcher_window.hide();
        }));
            launcher_window.present();
        }));

        let spacer = gtk4::Label::new(None);
        spacer.set_hexpand(true);

        let status = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
        let net = gtk4::Label::new(None);
        let vol = gtk4::Label::new(Some("Vol"));
        let bat = gtk4::Label::new(None);
        let clock = gtk4::Label::new(None);
        status.append(&net);
        status.append(&vol);
        status.append(&bat);
        status.append(&clock);

        panel.append(&title);
        panel.append(&launcher);
        panel.append(&spacer);
        panel.append(&status);

        let dock = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        dock.add_css_class("kevyt-dock");
        dock.set_halign(gtk4::Align::Center);
        dock.set_hexpand(true);

        for app in &apps {
            let button = gtk4::Button::with_label(&app.name);
            let exec = app.exec.clone();
            button.connect_clicked(move |_| {
                let _ = Command::new(&exec).spawn();
            });
            dock.append(&button);
        }

        root.append(&panel);
        root.append(&dock);
        window.set_child(Some(&root));

        if let Ok(connection) = Connection::session() {
            let settings = fetch_settings(&connection);
            apply_settings(&settings, &provider, &dock);
        }

        refresh_status(&net, &bat);
        glib::timeout_add_seconds_local(5, clone!(@strong net, @strong bat => move || {
            refresh_status(&net, &bat);
            glib::ControlFlow::Continue
        }));

        glib::timeout_add_seconds_local(1, move || {
            let now = Local::now();
            clock.set_text(&now.format("%H:%M").to_string());
            glib::ControlFlow::Continue
        });

        let provider_refresh = provider.clone();
        let dock_refresh = dock.clone();
        let (settings_tx, settings_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        thread::spawn(move || loop {
            if let Ok(connection) = Connection::session() {
                let settings = fetch_settings(&connection);
                let _ = settings_tx.send(settings);
            }
            thread::sleep(Duration::from_secs(5));
        });
        settings_rx.attach(None, move |settings| {
            apply_settings(&settings, &provider_refresh, &dock_refresh);
            glib::ControlFlow::Continue
        });

        window.show();
    });

    app.run();
}
