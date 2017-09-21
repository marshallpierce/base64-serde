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
                struct Base64Visitor;

                impl<'de> $crate::de::Visitor<'de> for Base64Visitor {
                    type Value = Vec<u8>;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        write!(formatter, "base64 ASCII text")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
                            E: $crate::de::Error, {
                        $crate::decode_config(v, $config).map_err($crate::de::Error::custom)
                    }
                }

                deserializer.deserialize_str(Base64Visitor)
            }
        }
    }
}
