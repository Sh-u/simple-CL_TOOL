use std::env;

use CL_TOOL::{get_input_args, open_file_and_read};
fn main() {
    let mut args: Vec<String> = env::args().collect();

    let mut iteration = 1;
    loop {


        let input = match get_input_args(&mut args, iteration) {
            Ok(inp) => inp,
            Err(err) => {
                eprintln!("open file and read: {err}");
                continue;
            }
        };

        let mut written_file = String::new();
        iteration += 1;
        let result = match open_file_and_read(
            input.ignore_case,
            &mut written_file,
            input.filename.as_str(),
            input.query.as_str(),
        ) {
            Ok(s) => s,
            Err(err) => {
                eprintln!("open file and read: {err}");
                continue;
            }
        };

        eprintln!("Searching for {}", input.query);
        eprintln!("In file {}", input.filename);

        eprintln!("result: {:?}", result);
        break;
    }
}
