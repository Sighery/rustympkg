use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn json_with_custom_simple_pkgbuild_should_work() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rustympkg")?;

    cmd.arg("-p").arg("tests/PKGBUILD").arg("json");

    let output = String::from_utf8(cmd.assert().success().get_output().stdout.clone())?;

    let result_json: serde_json::Value = serde_json::from_str(&output)?;

    assert!(result_json.is_object());

    let object = result_json.as_object().unwrap();

    let expected_pkgname = serde_json::json!(["terraform-provider-njalla"]);
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

    assert_eq!(object.get("pkgbase"), Some(&serde_json::json!(null)));
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
fn json_with_custom_complex_pkgbuild_should_work() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rustympkg")?;

    cmd.arg("-p").arg("tests/PKGBUILD.complex").arg("json");

    let output = String::from_utf8(cmd.assert().success().get_output().stdout.clone())?;

    let result_json: serde_json::Value = serde_json::from_str(&output)?;

    assert!(result_json.is_object());

    let object = result_json.as_object().unwrap();

    let expected_pkgbase = serde_json::json!("droidcam");
    assert_eq!(object.get("pkgbase"), Some(&expected_pkgbase));

    let expected_pkgname = serde_json::json!(["droidcam", "v4l2loopback-dc-dkms"]);
    assert_eq!(object.get("pkgname"), Some(&expected_pkgname));

    let expected_pkgver = serde_json::json!("1.7.3");
    assert_eq!(object.get("pkgver"), Some(&expected_pkgver));

    let expected_pkgrel = serde_json::json!(1);
    assert_eq!(object.get("pkgrel"), Some(&expected_pkgrel));

    let expected_epoch = serde_json::json!(1);
    assert_eq!(object.get("epoch"), Some(&expected_epoch));

    let expected_pkgdesc =
        serde_json::json!("A tool for using your android device as a wireless/usb webcam");
    assert_eq!(object.get("pkgdesc"), Some(&expected_pkgdesc));

    let expected_url = serde_json::json!("https://github.com/aramg/${pkgbase}");
    assert_eq!(object.get("url"), Some(&expected_url));

    let expected_arch = serde_json::json!(["x86_64"]);
    assert_eq!(object.get("arch"), Some(&expected_arch));

    let expected_license = serde_json::json!(["GPL"]);
    assert_eq!(object.get("license"), Some(&expected_license));

    let expected_makedepends = serde_json::json!(["gtk3", "ffmpeg", "libusbmuxd"]);
    assert_eq!(object.get("makedepends"), Some(&expected_makedepends));

    let expected_source = serde_json::json!([
        "${pkgbase}.desktop",
        "dkms.conf",
        "${pkgbase}.conf",
        "${pkgbase}-${pkgver}.zip::${url}/archive/v${pkgver}.zip"
    ]);
    assert_eq!(object.get("source"), Some(&expected_source));

    let expected_sha512sums = serde_json::json!([
        "72d21aa2d7eecc9bb070aaf7059a671246feb22f9c39b934a5463a4839f9347050de00754e5031dbc44f78eb2731f58f0cd2fcf781bc241f6fbd1abb4308b7ee",
        "27848dc6825c965c0aaac8e86220c3916ba20df6d941f5f05caecbf9c329ee744ee883bd2638ba58fe0dc3f40a8ae804dafbfbbe2efc23237e2b5450606cb78d",
        "74415b349bf8b2d1bb8181906f4254416d6223c5c42951185051bf3dd3e2f780db3441078ebff4a670eb0ffc76cc08f3b36851e0824c55a7f70136ce4d0240bc",
        "3934033dac931277a2f8ff348bcaa39b0cfe3e73885acd28f34b4b4efd8ce0b8606f23493b92206b5a7d3a2e1a2e1726d1d9ec33cd3f1876d1e6806dfb59c74f"
    ]);
    assert_eq!(object.get("sha512sums"), Some(&expected_sha512sums));

    assert_eq!(object.get("depends"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("optdepends"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("checkdepends"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("provides"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("conflicts"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("replaces"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("validpgpkeys"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("md5sums"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("sha224sums"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("sha256sums"), Some(&serde_json::json!(null)));
    assert_eq!(object.get("sha384sums"), Some(&serde_json::json!(null)));
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
