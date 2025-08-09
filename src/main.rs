use std::io::{StdoutLock, Write, stdout};

fn main() -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "Data Interchange").unwrap();

    return ();
}
