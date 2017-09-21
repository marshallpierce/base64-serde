extern crate base64;
extern crate serde;
#[cfg(test)]
#[macro_use]
extern crate serde_derive;

macro_rules! base64_serde_type {
    ($typename:ident, $config:expr) => {
        struct $typename {}

        impl $typename {
            pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
                where S: Serializer {
                serializer.serialize_str(&base64::encode_config(bytes, $config))
            }

            pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
                where D: Deserializer<'de> {
                let s = <&str>::deserialize(deserializer)?;
                base64::decode_config(s, $config).map_err(de::Error::custom)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::base64;
    use super::serde::{Deserialize, Deserializer, de, Serializer};

    base64_serde_type!(Base64Standard, base64::STANDARD);

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
                               base64::encode_config(&b.bytes, base64::STANDARD));
        assert_eq!(expected, s);

        let b2 = serde_json::from_str(&s).unwrap();
        assert_eq!(b, b2);
    }
}
