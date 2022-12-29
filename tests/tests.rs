#![allow(clippy::box_collection)]

use base64::{engine::general_purpose::STANDARD, Engine};
use base64_serde::base64_serde_type;
use std::cell::RefCell;

base64_serde_type!(Base64Standard, STANDARD);

mod some_other_mod {
    use base64::engine::general_purpose::STANDARD;
    use base64_serde::base64_serde_type;

    base64_serde_type!(pub Base64StandardInModule, STANDARD);
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct ByteHolder {
    #[serde(with = "Base64Standard")]
    bytes: Vec<u8>,
}

#[derive(Debug, PartialEq, serde::Deserialize)]
struct BoxVecHolder {
    #[serde(with = "Base64Standard")]
    bytes: Box<Vec<u8>>,
}

#[derive(Debug, PartialEq, serde::Deserialize)]
struct RefCellVecHolder {
    #[serde(with = "Base64Standard")]
    bytes: RefCell<Vec<u8>>,
}

#[derive(Debug, PartialEq, serde::Serialize)]
struct SliceHolder<'a> {
    #[serde(with = "Base64Standard")]
    bytes: &'a [u8],
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct ByteHolderHelperInMod {
    #[serde(with = "some_other_mod::Base64StandardInModule")]
    bytes: Vec<u8>,
}

#[test]
fn serde_with_type() {
    let b = ByteHolder {
        bytes: vec![0x00, 0x77, 0xFF],
    };

    let s = serde_json::to_string(&b).unwrap();
    let expected = format!("{{\"bytes\":\"{}\"}}", STANDARD.encode(&b.bytes));
    assert_eq!(expected, s);

    let b2 = serde_json::from_str(&s).unwrap();
    assert_eq!(b, b2);
}

#[test]
fn deserialize_with_box_vec() {
    let b = BoxVecHolder {
        bytes: Box::new(vec![0x00, 0x77, 0xFF]),
    };

    let json = format!("{{\"bytes\":\"{}\"}}", STANDARD.encode(b.bytes.as_slice()));

    let b2 = serde_json::from_str(&json).unwrap();
    assert_eq!(b, b2);
}

#[test]
fn deserialize_with_refcell() {
    let b = RefCellVecHolder {
        bytes: RefCell::new(vec![0x00, 0x77, 0xFF]),
    };

    let expected = format!(
        "{{\"bytes\":\"{}\"}}",
        STANDARD.encode(b.bytes.borrow().as_slice())
    );

    let b2 = serde_json::from_str(&expected).unwrap();
    assert_eq!(b, b2);
}

#[test]
fn serialize_with_u8_slice() {
    let bytes = vec![0x00, 0x77, 0xFF];
    let b = SliceHolder { bytes: &bytes };

    let s = serde_json::to_string(&b).unwrap();
    let expected = format!("{{\"bytes\":\"{}\"}}", STANDARD.encode(b.bytes));
    assert_eq!(expected, s);
}

#[test]
fn serde_with_type_using_public_helper() {
    let b = ByteHolderHelperInMod {
        bytes: vec![0x00, 0x77, 0xFF],
    };

    let s = serde_json::to_string(&b).unwrap();
    let expected = format!("{{\"bytes\":\"{}\"}}", STANDARD.encode(&b.bytes));
    assert_eq!(expected, s);

    let b2 = serde_json::from_str(&s).unwrap();
    assert_eq!(b, b2);
}

#[test]
fn fails_nicely_on_inappropriate_input() {
    assert_eq!(
        "invalid type: integer `1234`, expected base64 ASCII text at line 1 column 13",
        format!(
            "{}",
            serde_json::from_str::<ByteHolder>("{\"bytes\":1234}").unwrap_err()
        )
    );
}
