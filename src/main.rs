use std::io::{self, Write};

fn fail(error: impl std::fmt::Display) -> ! {
    eprintln!("json_error:{error}");
    std::process::exit(1)
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let result = match args.as_slice() {
        [command, text] if command == "parse" => adamantium_json::pretty(text),
        [command, text] if command == "compact" => adamantium_json::compact(text),
        [command, text] if command == "type-of" => {
            adamantium_json::type_of(text).map(str::to_owned)
        }
        [command, text] if command == "is-valid" => {
            println!("{}", adamantium_json::parse(text).is_ok());
            return;
        }
        [command, path] if command == "read" => {
            print!("{}", adamantium_json::read(path).unwrap_or_else(|e| fail(e)));
            return;
        }
        [command, path, text] if command == "write" => {
            adamantium_json::write(path, text).unwrap_or_else(|e| fail(e));
            return;
        }
        _ => {
            eprintln!("Usage: adamantium-json <parse|compact|type-of|is-valid|read|write> <value> [json]");
            std::process::exit(2);
        }
    };
    writeln!(io::stdout(), "{}", result.unwrap_or_else(|e| fail(e))).unwrap_or_else(|e| fail(e));
}
