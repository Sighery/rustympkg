use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn json_with_custom_pkgbuild_should_work() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rustympkg")?;

    cmd.arg("-p").arg("tests/PKGBUILD").arg("json");

    let output = String::from_utf8(cmd.assert().success().get_output().stdout.clone())?;

    let result_json: serde_json::Value = serde_json::from_str(&output)?;

    assert!(result_json.is_object());

    let object = result_json.as_object().unwrap();

    let expected_pkgname = serde_json::json!("terraform-provider-njalla");
    assert_eq!(object.get("pkgname"), Some(&expected_pkgname));

    let expected_pkgver = serde_json::json!("0.7.0");
    assert_eq!(object.get("pkgver"), Some(&expected_pkgver));

    let expected_pkgrel = serde_json::json!(1);
    assert_eq!(object.get("pkgrel"), Some(&expected_pkgrel));

    let expected_pkgdesc = serde_json::json!("Unofficial Terraform Njalla provider plugin");
    assert_eq!(object.get("pkgdesc"), Some(&expected_pkgdesc));

    let expected_url = serde_json::json!("https://github.com/Sighery/terraform-provider-njalla");
    assert_eq!(object.get("url"), Some(&expected_url));

    let expected_arch = serde_json::json!(["x86_64"]);
    assert_eq!(object.get("arch"), Some(&expected_arch));

    let expected_license = serde_json::json!(["MIT"]);
    assert_eq!(object.get("license"), Some(&expected_license));

    let expected_makedepends = serde_json::json!(["go"]);
    assert_eq!(object.get("makedepends"), Some(&expected_makedepends));

    let expected_source =
        serde_json::json!(["$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz"]);
    assert_eq!(object.get("source"), Some(&expected_source));

    let expected_sha256sums =
        serde_json::json!(["29d5b4c94dcfe2260e0d217392e2aa935a6b81e7388f72305fde87f0b680189a"]);
    assert_eq!(object.get("sha256sums"), Some(&expected_sha256sums));

    assert_eq!(object.get("epoch"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("depends"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("optdepends"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("checkdepends"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("provides"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("conflicts"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("replaces"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("validpgpkeys"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("md5sums"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("sha224sums"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("sha384sums"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("sha512sums"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("b2sums"), Some(&serde_json::json!(null)));

    Ok(())
}

#[test]
fn json_with_custom_inexistent_pkgbuild_should_fail() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rustympkg")?;

    cmd.arg("-p").arg("tests/inexistent-PKGBUILD").arg("json");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}
