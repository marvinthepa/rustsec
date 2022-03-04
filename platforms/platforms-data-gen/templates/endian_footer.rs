
impl fmt::Display for Endian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl Serialize for Endian {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Endian {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = <&str>::deserialize(deserializer)?;
        if cfg!(feature = "std") {
            Ok(string.parse().map_err(|_| D::Error::custom(std::format!("Unrecognized value '{}' for target_endian", string)))?)
        } else {
            Ok(string.parse().map_err(|_| D::Error::custom("Unrecognized value for target_endian"))?)
        }
    }
}