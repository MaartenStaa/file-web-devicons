use std::borrow::Cow;

use crate::extract_filepath;

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
