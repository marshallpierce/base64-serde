extern crate base64;
extern crate serde;
#[cfg(test)]
#[macro_use]
extern crate serde_derive;

#[doc(hidden)]
pub use serde::{Deserializer, de, Serializer};
#[doc(hidden)]
pub use base64::{encode_config, decode_config};

macro_rules! base64_serde_type {
    ($typename:ident, $config:expr) => {
        struct $typename {}

        impl $typename {
            pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
                where S: $crate::Serializer {
                serializer.serialize_str(&$crate::encode_config(bytes, $config))
            }

            pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
                where D: $crate::Deserializer<'de> {
                let s = <&str>::deserialize(deserializer)?;
                $crate::decode_config(s, $config).map_err($crate::de::Error::custom)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::serde::Deserialize;
    use super::base64::STANDARD;

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
                               super::base64::encode_config(&b.bytes, STANDARD));
        assert_eq!(expected, s);

        let b2 = serde_json::from_str(&s).unwrap();
        assert_eq!(b, b2);
    }
}
