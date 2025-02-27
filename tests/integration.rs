// Copyright 2022 Adobe. All rights reserved.
// This file is licensed to you under the Apache License,
// Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
// or the MIT license (http://opensource.org/licenses/MIT),
// at your option.

// Unless required by applicable law or agreed to in writing,
// this software is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR REPRESENTATIONS OF ANY KIND, either express or
// implied. See the LICENSE-MIT and LICENSE-APACHE files for the
// specific language governing permissions and limitations under
// each license.

use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;
use std::path::PathBuf;
use std::process::Command;

const TEST_IMAGE: &str = "earth_apollo17.jpg";
//const TEST_IMAGE: &str = "libpng-test.png"; // save for png testing
//const TEST_IMAGE_WITH_MANIFEST: &str = "C.jpg"; // save for manifest tests

fn fixture_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fixtures");
    path.push(name);
    std::fs::canonicalize(path).expect("canonicalize")
}

fn temp_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    std::fs::create_dir_all(&path).ok();
    path.push(name);
    path
}

#[test]
fn tool_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("c2patool")?;
    cmd.arg("test/file/not.found");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("File not found"));
    Ok(())
}

#[test]
fn tool_jpeg_no_report() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("c2patool")?;
    cmd.arg(fixture_path(TEST_IMAGE));
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No claim found"));
    Ok(())
}

#[test]
fn tool_embed_jpeg_report() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("c2patool")?
        .arg("sample/test.json")
        .arg("-p")
        .arg(fixture_path(TEST_IMAGE))
        .arg("-o")
        .arg(temp_path("out.jpg"))
        .assert()
        .success() // should this be a failure?
        .stdout(predicate::str::contains("My Title"));
    Ok(())
}

/* remove this until the c2patool supports .c2pa write again
#[test]
fn tool_manifest_report() -> Result<(), Box<dyn std::error::Error>> {

    // first export a c2pa file
    Command::cargo_bin("c2patool")?
        .arg(fixture_path(TEST_IMAGE_WITH_MANIFEST))
        .arg("-o")
        .arg(temp_path("manifest.c2pa"))
        .assert()
        .success()
        .stdout(predicate::str::contains("C2PA Testing"));
    // then read it back in
    Command::cargo_bin("c2patool")?
        .arg(temp_path("manifest.c2pa"))
        .assert()
        .success()
        .stdout(predicate::str::contains("C2PA Testing"));
    Ok(())
}
*/
