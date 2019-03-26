use std::env;
use std::process;

use dice_roller;
use dice_roller::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = dice_roller::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

mod tests {
    extern crate assert_cmd;

    use std::process::Command;
    use assert_cmd::prelude::*;

    #[test]
    fn config_valid_args() {
        let mut cmd = Command::cargo_bin("dice_roller").unwrap();
        cmd
            .arg("2d8")
            .assert()
            .success();
    }

    #[test]
    fn config_no_args() {
        let mut cmd = Command::cargo_bin("dice_roller").unwrap();
        cmd
            .assert()
            .failure()
            .stderr("Problem parsing arguments: Didn't get a dice roll\n");
    }

    #[test]
    fn config_bad_args() {
        let mut cmd = Command::cargo_bin("dice_roller").unwrap();
        cmd
            .arg("test")
            .assert()
            .failure()
            .stderr("\
                Problem parsing arguments: Dice roll must be of the \
                format: <num_of_rolls>d<sides_of_die>\n"
            );
    }
}
