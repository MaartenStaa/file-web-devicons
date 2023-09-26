use std::borrow::Cow;

enum FilepathState<'a> {
    /// Result should just be the entire input (trimmed) input.
    Simple,
    /// Result should be a slice from this offset in the (trimmed) input.
    FromOffset(usize),
    /// Result should be this buffer, or use this buffer as a starting point when
    /// needing to allocate.
    Buffer(&'a str),
    /// Result is this allocated string. Any other characters found should be
    /// appended to this.
    Allocated(String),
}

// NOTE: This function is a bit convoluted to try to avoid allocating a new string
// if not needed (e.g. path is entirely lowercase, no escape codes in the middle
// of the path).
pub(crate) fn extract_filepath(input: &str) -> Cow<str> {
    let input = input.trim();

    // Are we currently in an escape code.
    let mut in_escape = false;
    let mut state = FilepathState::Simple;

    for (i, char) in input.trim().chars().enumerate() {
        match char {
            '\x1B' if !in_escape => {
                match state {
                    FilepathState::Simple if i > 0 => state = FilepathState::Buffer(&input[..i]),
                    FilepathState::FromOffset(j) if i == j => {
                        // Two (or more) consecutive escape codes at the beginning of the string.
                        // The offset will be saved again at the end of this escape code.
                        state = FilepathState::Simple
                    }
                    FilepathState::FromOffset(j) => state = FilepathState::Buffer(&input[j..i]),
                    _ => {}
                }

                in_escape = true;
            }
            'm' | 'G' | 'K' if in_escape => {
                in_escape = false;

                match state {
                    FilepathState::Simple => state = FilepathState::FromOffset(i + 1),
                    FilepathState::FromOffset(_) => unreachable!(
                        "FromOffset is always replaced when stargin an escape sequence"
                    ),
                    FilepathState::Buffer(_) | FilepathState::Allocated(_) => {}
                }
            }
            // FIXME: This may break Windows support if the input contains absolute file paths.
            ':' if !in_escape => break,
            char if !in_escape => match &mut state {
                FilepathState::Simple if char.is_uppercase() => {
                    let mut s = String::with_capacity(input.len());
                    s.push_str(&input[..i]);
                    s.extend(char.to_lowercase());
                    state = FilepathState::Allocated(s)
                }
                FilepathState::FromOffset(x) if char.is_uppercase() => {
                    let mut s = String::with_capacity(input.len());
                    s.push_str(&input[*x..i]);
                    s.extend(char.to_lowercase());
                    state = FilepathState::Allocated(s)
                }
                FilepathState::Buffer(b) => {
                    let mut s = String::with_capacity(input.len());
                    s.push_str(b);
                    s.extend(char.to_lowercase());
                    state = FilepathState::Allocated(s)
                }
                FilepathState::Allocated(s) => s.extend(char.to_lowercase()),
                FilepathState::Simple | FilepathState::FromOffset(_) => {}
            },
            _ => {}
        }
    }

    match state {
        FilepathState::Simple => Cow::Borrowed(input),
        FilepathState::FromOffset(i) => Cow::Borrowed(&input[i..]),
        FilepathState::Buffer(b) => Cow::Borrowed(b),
        FilepathState::Allocated(s) => Cow::Owned(s),
    }
}
