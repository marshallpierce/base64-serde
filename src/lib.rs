extern crate base64;
extern crate serde;

#[doc(hidden)]
pub use serde::{de, Deserializer, Serializer};

/// Create a type with appropriate `serialize` and `deserialize` functions to use with
/// serde when specifying how to serialize a particular field.
///
/// Once the type is defined, you can use `#[serde(with = "YourTypeNameHere")]` on a `Vec<u8>`
/// field that you wished to serialize to base64 or deserialize from base64.
///
/// If you want to change resulting type's visibility, prefix the desired type
/// name with appropiate visibility, for example:
///
/// ```
/// use base64_serde::base64_serde_type;
///
/// base64_serde_type!(pub IWillBeAPublicType, base64::engine::general_purpose::STANDARD);
/// base64_serde_type!(pub(crate) IWillBeACrateType, base64::engine::general_purpose::STANDARD);
/// ```
///
/// # Examples
///
/// Existing engine:
///
/// ```
/// use base64_serde::base64_serde_type;
///
/// base64_serde_type!(Base64Standard, base64::engine::general_purpose::STANDARD);
///
/// #[derive(serde::Serialize, serde::Deserialize)]
/// struct ByteHolder {
///     #[serde(with = "Base64Standard")]
///     bytes: Vec<u8>,
/// }
/// ```
///
/// Custom engine:
///
/// ```
/// use base64_serde::base64_serde_type;
///
/// const BCRYPT_NO_PAD: base64::engine::GeneralPurpose =
///     base64::engine::GeneralPurpose::new(
///         &base64::alphabet::BCRYPT,
///         base64::engine::general_purpose::NO_PAD
///     );
///
/// base64_serde_type!(Base64BcryptNoPad, BCRYPT_NO_PAD);
/// ```
#[macro_export]
macro_rules! base64_serde_type {
    ($visibility:vis $typename:ident, $engine:expr) => {
        $visibility enum $typename {}
        base64_serde_type!(impl_only, $typename, $engine);
    };
    (impl_only, $typename:ident, $engine:expr) => {
        impl $typename {
            pub fn serialize<S, Input>(
                bytes: Input,
                serializer: S,
            ) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: $crate::Serializer,
                Input: AsRef<[u8]>,
            {
                use base64::Engine as _;
                serializer.serialize_str(&$engine.encode(bytes.as_ref()))
            }

            pub fn deserialize<'de, D, Output>(
                deserializer: D,
            ) -> ::std::result::Result<Output, D::Error>
            where
                D: $crate::Deserializer<'de>,
                Output: From<Vec<u8>>,
            {
                struct Base64Visitor;

                impl<'de> $crate::de::Visitor<'de> for Base64Visitor {
                    type Value = Vec<u8>;

                    fn expecting(
                        &self,
                        formatter: &mut ::std::fmt::Formatter,
                    ) -> ::std::fmt::Result {
                        write!(formatter, "base64 ASCII text")
                    }

                    fn visit_str<E>(self, v: &str) -> ::std::result::Result<Self::Value, E>
                    where
                        E: $crate::de::Error,
                    {
                        use base64::Engine as _;
                        $engine.decode(v).map_err($crate::de::Error::custom)
                    }
                }

                deserializer
                    .deserialize_str(Base64Visitor)
                    .map(|vec| Output::from(vec))
            }
        }
    };
}
