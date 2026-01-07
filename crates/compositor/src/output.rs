use std::fs;

#[derive(Debug)]
pub struct OutputState {
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub scale: i32,
}

fn parse_mode(mode: &str) -> Option<(i32, i32)> {
    let mut parts = mode.trim().split('x');
    let w = parts.next()?.parse::<i32>().ok()?;
    let h = parts.next()?.parse::<i32>().ok()?;
    Some((w, h))
}

fn discover_sysfs_outputs() -> Vec<OutputState> {
    let mut outputs = Vec::new();
    let entries = match fs::read_dir("/sys/class/drm") {
        Ok(entries) => entries,
        Err(_) => return outputs,
    };

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.contains('-') {
            continue;
        }
        let status_path = entry.path().join("status");
        let status = fs::read_to_string(&status_path).unwrap_or_default();
        if status.trim() != "connected" {
            continue;
        }

        let modes_path = entry.path().join("modes");
        let modes = fs::read_to_string(&modes_path).unwrap_or_default();
        let mode = modes.lines().next().and_then(parse_mode);
        let (width, height) = mode.unwrap_or((1280, 720));

        outputs.push(OutputState {
            name,
            width,
            height,
            scale: 1,
        });
    }

    outputs
}

pub fn init_outputs() -> Vec<OutputState> {
    let outputs = discover_sysfs_outputs();
    if outputs.is_empty() {
        vec![OutputState {
            name: "default-output".to_string(),
            width: 1280,
            height: 720,
            scale: 1,
        }]
    } else {
        outputs
    }
}
