use std::fmt;

use serde::{de, Deserializer};

header! { (XPlexProduct, "X-Plex-Product") => [String] }
header! { (XPlexVersion, "X-Plex-Version") => [String] }
header! { (XPlexClientIdentifier, "X-Plex-Client-Identifier") => [String] }
header! { (XPlexToken, "X-Plex-Token") => [String] }

pub(crate) fn deserialize_xml_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct XmlBool();

    impl<'de> de::Visitor<'de> for XmlBool {
        type Value = bool;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("\"0\" (false) or \"1\" (true)")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match s {
                "0" => Ok(false),
                "1" => Ok(true),
                _ => Err(de::Error::invalid_value(de::Unexpected::Str(s), &self)),
            }
        }
    }

    let visitor = XmlBool();
    deserializer.deserialize_any(visitor)
}
