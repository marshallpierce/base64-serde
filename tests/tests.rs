#[macro_use]
extern crate base64_serde;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate base64;

use serde::Deserialize;
use base64::{encode_config, STANDARD};

base64_serde_type!(Base64Standard, STANDARD);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ByteHolder {
    #[serde(with = "Base64Standard")]
    bytes: Vec<u8>,
}

#[test]
fn serde_with_type() {
    let b = ByteHolder { bytes: vec![0x00, 0x77, 0xFF] };

    let s = serde_json::to_string(&b).unwrap();
    let expected = format!("{{\"bytes\":\"{}\"}}",
                           encode_config(&b.bytes, STANDARD));
    assert_eq!(expected, s);

    let b2 = serde_json::from_str(&s).unwrap();
    assert_eq!(b, b2);
}
