use std::{collections::HashMap, io::stdin, path::Path};

use icons::Icon;

mod icons;

fn get_icon<'a>(
    path: &Path,
    icons_by_filename: &'a HashMap<&str, Icon>,
    icons_by_extension: &'a HashMap<&str, Icon>,
) -> (&'a str, Option<(u8, u8, u8)>) {
    if let Some(filename) = path.file_name() {
        if let Some(icon) = icons_by_filename.get(filename.to_str().unwrap()) {
            return (
                icon.icon,
                Some((icon.color_red, icon.color_green, icon.color_blue)),
            );
        }
    }

    if let Some(extension) = path.extension() {
        if let Some(icon) = icons_by_extension.get(extension.to_str().unwrap()) {
            return (
                icon.icon,
                Some((icon.color_red, icon.color_green, icon.color_blue)),
            );
        }
    }

    (icons::DEFAULT_ICON, None)
}

fn main() {
    let icons_by_filename = icons::ICONS_BY_FILENAME.lock().unwrap();
    let icons_by_extension = icons::ICONS_BY_FILE_EXTENSION.lock().unwrap();

    let default_icon_color = "6d8086";
    let (default_r, default_g, default_b) = (
        u8::from_str_radix(&default_icon_color[0..2], 16).unwrap(),
        u8::from_str_radix(&default_icon_color[2..4], 16).unwrap(),
        u8::from_str_radix(&default_icon_color[4..6], 16).unwrap(),
    );

    // https://github.com/ibhagwan/fzf-lua/blob/2fa4913c7db0c22e02c003c6f09b7ebb2d0ed367/lua/fzf-lua/utils.lua#L40
    // Using the non-breaking space as a separator makes `fzf-lua` detect the
    // icon and the path as two separate columns.
    let non_breaking_space = '\u{2002}';

    let input = stdin();
    let mut buffer = String::new();
    while input.read_line(&mut buffer).unwrap() > 0 {
        let trimmed_lowercase = buffer.trim().to_lowercase();
        let path = Path::new(&trimmed_lowercase);
        let (icon, color) = get_icon(path, &icons_by_filename, &icons_by_extension);
        let (r, g, b) = color.unwrap_or((default_r, default_g, default_b));

        print!("\x1b[38;2;{r};{g};{b}m{icon}\x1b[0m{non_breaking_space}{buffer}");

        buffer.clear();
    }
}
