use std::io::{stdin, stdout, Write};

use file_web_devicon_lib::handle_input_line;

pub(crate) fn main() {
    let input = stdin();
    let mut output = stdout().lock();
    let mut buffer = String::new();
    let mut index = 0;
    while input.read_line(&mut buffer).unwrap() > 0 {
        let line_to_write = handle_input_line(&buffer);
        write!(&mut output, "{line_to_write}").unwrap();

        buffer.clear();

        index += 1;
        if index % 50 == 0 {
            output.flush().unwrap();
        }
    }

    output.flush().unwrap();
}
