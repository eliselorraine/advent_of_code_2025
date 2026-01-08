use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use fixtures::fixtures;

// Macros run at compile time. Means they cannot accept dynamic arguments.
// Functions run at runtime. 

fn command() -> std::process::Command {
    Command::new(assert_cmd::cargo::cargo_bin!("day_two"))
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = command();
    cmd.arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Couldn't read file."));

    Ok(())
}

#[fixtures(["tests/fixtures/puzzle_input.txt"])]
#[test]
fn find_content_in_file(path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = command();
    cmd.arg(path);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("33"));

    Ok(())
}
