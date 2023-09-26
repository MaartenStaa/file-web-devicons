use std::{borrow::Cow, path::Path};

use crate::{extract_filepath, get_icon, icons};

#[test]
fn matches_by_filename() {
    let icons_by_filename = &icons::ICONS_BY_FILENAME;
    for (k, v) in icons_by_filename.iter() {
        let path = format!("/usr/local/project/{k}");
        let path = Path::new(&path);

        assert_eq!(
            get_icon(
                path,
                icons_by_filename,
                &icons::ICONS_BY_FILE_EXTENSION,
                &icons::DEFAULT_ICON
            ),
            v
        );
    }
}

#[test]
fn matches_by_extension() {
    let icons_by_extension = &icons::ICONS_BY_FILE_EXTENSION;
    for (k, v) in icons_by_extension.iter() {
        let path = format!("/usr/local/project/filename.{k}");
        let path = Path::new(&path);

        assert_eq!(
            get_icon(
                path,
                &icons::ICONS_BY_FILENAME,
                icons_by_extension,
                &icons::DEFAULT_ICON
            ),
            v,
            "should get the correct icon for extension {k} (path = {})",
            path.display()
        );
    }
}

#[test]
fn falls_back_to_default_icon() {
    assert_eq!(
        get_icon(
            Path::new("/usr/local/project/madeup.extensionzzz"),
            &icons::ICONS_BY_FILENAME,
            &icons::ICONS_BY_FILE_EXTENSION,
            &icons::DEFAULT_ICON
        ),
        &*icons::DEFAULT_ICON,
        "should fall back to default icon",
    );
}

#[test]
fn extracts_filepaths() {
    assert_eq!(
        Cow::Borrowed("/foo/bar/baz.txt"),
        extract_filepath("/foo/bar/baz.txt\n"),
        "returns unmodified lowercase trimmed paths"
    );
    assert_eq!(
        Cow::Owned::<str>("/foo/bar/baz.txt".to_string()),
        extract_filepath("/fOo/Bar/baz.TXT\n"),
        "returns owned lowercase trimmed version of path"
    );
    assert_eq!(
        Cow::Owned::<str>("/users/me/scripts/update-icons.sh".to_string()),
        extract_filepath(
            "\x1b38;5;81m/Users/me/scripts/\x1b0m\x1b1;38;5;203mupdate-icons.sh\x1b0m"
        ),
        "skips over escape codes"
    );
    assert_eq!(
        Cow::Borrowed("/foo/bar/baz.txt"),
        extract_filepath(
            "\x1b[0m\x1b[35m/foo/bar/baz.txt\x1b[0m:\x1b[32m159\x1b[0m:the rest of the string"
        ),
        "can extract uninterupdated already lowercase path"
    );
}
