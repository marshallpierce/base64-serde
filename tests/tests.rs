#[macro_use]
extern crate base64_serde;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate base64;

use base64::{encode_config, STANDARD};
use serde::de::{Deserialize, Expected, IntoDeserializer, Unexpected};
use std::fmt::{self, Display};

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
    let expected = format!("{{\"bytes\":\"{}\"}}", encode_config(&b.bytes, STANDARD));
    assert_eq!(expected, s);

    let b2 = serde_json::from_str(&s).unwrap();
    assert_eq!(b, b2);
}

#[test]
fn fails_nicely_on_inappropriate_input() {
    #[derive(Debug)]
    struct CheckError;

    impl Display for CheckError {
        fn fmt(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
            unimplemented!()
        }
    }

    impl std::error::Error for CheckError {
        fn description(&self) -> &str {
            unimplemented!()
        }
    }

    impl serde::de::Error for CheckError {
        fn custom<T: Display>(_msg: T) -> Self {
            unimplemented!()
        }

        fn invalid_type(unexp: Unexpected, exp: &Expected) -> Self {
            assert_eq!(unexp, Unexpected::Signed(1234));
            assert_eq!(exp.to_string(), "base64 ASCII text");
            CheckError
        }
    }

    // The only way Serde could have constructed an error of type CheckError is
    // through the invalid_type constructor, where we assert that the unexpected
    // and expected values are what we intended.
    let de = vec![1234].into_deserializer();
    let _: CheckError = ByteHolder::deserialize(de).unwrap_err();
}
