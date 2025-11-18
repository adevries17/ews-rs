use serde::Deserialize;
use xml_struct::XmlSerialize;

#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct TimeZoneDefinitions {
    #[serde(rename = "$value", default)]
    pub inner: Vec<TimeZoneDefinition>,
}

#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct TimeZoneDefinition {
    /// Represents the unique identifier of the time zone.
    #[xml_struct(attribute)]
    #[serde(rename = "@Id")]
    pub id: String,

    /// Represents the descriptive name of the time zone.
    #[xml_struct(attribute)]
    #[serde(rename = "@Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub periods: Option<Periods>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitions_groups: Option<TransitionsGroups>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Transitions>,
}

impl Default for TimeZoneDefinition {
    fn default() -> Self {
        Self {
            id: "UTC".to_string(),
            name: Default::default(),
            periods: Default::default(),
            transitions_groups: Default::default(),
            transitions: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
pub struct Periods {
    #[serde(rename = "$value", default)]
    pub period: Vec<Period>,
}

#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
pub struct Period {
    #[xml_struct(attribute)]
    #[serde(rename = "@Id")]
    pub id: String,

    #[xml_struct(attribute)]
    #[serde(rename = "@Bias")]
    pub bias: String,

    #[xml_struct(attribute)]
    #[serde(rename = "@Name")]
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct TransitionsGroups {
    #[xml_struct(attribute)]
    #[serde(rename = "@Id")]
    pub id: String,

    #[serde(rename = "$value")]
    pub transitions: Vec<Transitions>,
}

#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Transitions {
    #[serde(rename = "$value", default)]
    pub transitions: Vec<Transition>,
}

#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum Transition {
    AbsoluteDateTransition(AbsoluteDateTransition),
    RecurringDayTransition(RecurringDayTransition),
    RecurringDateTransition(RecurringDateTransition),
}

#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct AbsoluteDateTransition {
    pub to: String,

    pub date_time: String,
}

#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct RecurringDayTransition {
    pub to: String,

    pub time_offset: String,

    pub month: usize,

    pub day_of_week: String,

    pub occurrence: usize,
}

#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct RecurringDateTransition {
    pub to: String,

    pub time_offset: String,

    pub month: usize,

    pub day: usize,
}

/// Identifies a single time zone definition.
#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
pub struct TimeZoneId {
    #[xml_struct(ns_prefix = "t")]
    pub id: String,
}
