use std::error::Error;

fn try_main() -> Result<(), Box<Error>> {
    Ok(())
}

fn main() {
    if let Err(err) = try_main() {
        panic!("{}", err.description());
    }
}
