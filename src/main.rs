use dstopic;
use std::fs;
use std::io;

fn main() {
    dstopic::parse_args::parse_args().get_matches();

    // echo command

    let mut cmd = String::new();

    io::stdin().read_line(&mut cmd)
        .expect("Failed to read command");

    print!("{}", cmd);
}
