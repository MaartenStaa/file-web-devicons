use std::{collections::HashMap, path::Path};

use filepaths::extract_filepath;

pub(crate) mod filepaths;
pub(crate) mod icons;
#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Icon {
    pub icon: &'static str,
    pub color_red: u8,
    pub color_green: u8,
    pub color_blue: u8,
}

impl Icon {
    const fn new(icon: &'static str, color: u32) -> Icon {
        Icon {
            icon,
            color_red: ((color >> 16) & 0xFF) as u8,
            color_green: ((color >> 8) & 0xFF) as u8,
            color_blue: (color & 0xFF) as u8,
        }
    }
}

/// Take a line read from stdin, find an appropriate icon for it, and return
/// a line to write to stdout including the icon in the right color (using
/// terminal escape codes).
pub fn handle_input_line(line: &str) -> String {
    // https://github.com/ibhagwan/fzf-lua/blob/2fa4913c7db0c22e02c003c6f09b7ebb2d0ed367/lua/fzf-lua/utils.lua#L40
    // Using the non-breaking space as a separator makes `fzf-lua` detect the
    // icon and the path as two separate columns.
    let non_breaking_space = '\u{2002}';

    let filepath = extract_filepath(line);
    let path = Path::new(filepath.as_ref());
    let icon = get_icon(
        path,
        &icons::ICONS_BY_FILENAME,
        &icons::ICONS_BY_FILE_EXTENSION,
        &icons::DEFAULT_ICON,
    );
    let (r, g, b) = (icon.color_red, icon.color_green, icon.color_blue);
    let icon = icon.icon;

    format!("\x1b[38;2;{r};{g};{b}m{icon}\x1b[0m{non_breaking_space}{line}")
}

/// Find an appropriate icon in `icons_by_filename` or `icons_by_extension` for
/// the given `path`. Falls back to returning the `default_icon` otherwise.
pub fn get_icon<'a>(
    path: &Path,
    icons_by_filename: &'a HashMap<&str, Icon>,
    icons_by_extension: &'a HashMap<&str, Icon>,
    default_icon: &'a Icon,
) -> &'a Icon {
    if let Some(filename) = path.file_name() {
        if let Some(icon) = icons_by_filename.get(filename.to_str().unwrap()) {
            return icon;
        }
    }

    if let Some(extension) = path.extension() {
        if let Some(icon) = icons_by_extension.get(extension.to_str().unwrap()) {
            return icon;
        }
    }

    default_icon
}
