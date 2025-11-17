use serde::Deserialize;
use xml_struct::XmlSerialize;

#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
pub struct TimeZoneDefinitions {
    #[xml_struct(flatten)]
    pub inner: Vec<TimeZoneDefinition>,
}

#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
pub struct TimeZoneDefinition {
    /// Represents the unique identifier of the time zone.
    #[xml_struct(attribute)]
    pub id: String,

    /// Represents the descriptive name of the time zone.
    #[xml_struct(attribute)]
    pub name: Option<String>,
}

/// Identifies a single time zone definition.
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
pub struct TimeZoneId {
    pub id: String,
}
