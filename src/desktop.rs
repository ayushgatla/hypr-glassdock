use std::fs;

pub fn get_icon(name: &str) -> String {

    let path =
        format!("/usr/share/applications/{}.desktop", name);

    let text =
        fs::read_to_string(path).unwrap_or_default();

    for line in text.lines() {
        if let Some(icon) =
            line.strip_prefix("Icon=")
        {
            return icon.to_string();
        }
    }

    name.to_string()
}
