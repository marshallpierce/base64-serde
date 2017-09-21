extern crate base64;
extern crate serde;

#[doc(hidden)]
pub use serde::{Deserializer, de, Serializer};
#[doc(hidden)]
pub use base64::{encode_config, decode_config};

/// Create a module with appropriate `serialize` and `deserialize` functions to use with
/// serde when specifying how to serialize a particular field, as in `#[serde(with=modname)]`.
#[macro_export]
macro_rules! base64_serde_type {
    ($typename:ident, $config:expr) => {
        enum $typename {}
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
