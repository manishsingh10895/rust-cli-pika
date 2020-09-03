use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn run_with_defaults() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("pikacli")
        .expect("binary doesn't exists")
        .assert()
        .stdout(predicate::str::contains("Created by manishsingh10895"))
        .success();

    Ok(())
 }

 #[test]
 fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>> {
     Command::cargo_bin("pikacli")
        .expect("binary doesn't exists")
        .args(&["-f", "no/dasjdh/adaks.txt"])
        .assert()
        .failure();

        Ok(())
 }


 