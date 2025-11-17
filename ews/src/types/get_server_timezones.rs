use ews_proc_macros::operation_response;
use serde::Deserialize;
use xml_struct::XmlSerialize;

use crate::{
    timezone::{TimeZoneDefinitions, TimeZoneId},
    MESSAGES_NS_URI,
};

#[derive(Clone, Debug, Deserialize, XmlSerialize, PartialEq, Eq)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
#[operation_response(TimeZoneDefinitions)]
pub struct GetServerTimeZones {
    #[xml_struct(attribute)]
    pub return_full_time_zone_data: Option<bool>,

    pub ids: Option<Vec<TimeZoneId>>,
}

#[cfg(test)]
mod tests {
    use crate::{
        test_utils::{assert_deserialized_content, assert_serialized_content},
        timezone::TimeZoneDefinition,
        ResponseClass, ResponseMessages,
    };

    use super::*;

    #[test]
    fn serialize_get_server_time_zones_request() {
        let get_server_timezones = GetServerTimeZones {
            return_full_time_zone_data: Some(true),
            ids: Some(vec![TimeZoneId {
                id: "Eastern Standard Time".to_string(),
            }]),
        };

        //let expected = r#"<m:GetServerTimeZones ReturnFullTimeZoneData="true"><m:Ids><t:Id>Eastern Standard Time</Id></m:Ids></m:GetServerTimeZones>"#;
        let expected = r#"<GetServerTimeZones xmlns="http://schemas.microsoft.com/exchange/services/2006/messages" ReturnFullTimeZoneData="true"><Ids><Id>Eastern Standard Time</Id></Ids></GetServerTimeZones>"#;

        assert_serialized_content(&get_server_timezones, "GetServerTimeZones", expected);
    }

    #[test]
    fn deserialize_get_server_time_zones_response() {
        let content = r#"<GetServerTimeZonesResponse xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
                                        xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages">
              <m:ResponseMessages>
                <m:GetServerTimeZonesResponseMessage ResponseClass="Success">
                  <m:ResponseCode>NoError</m:ResponseCode>
                  <m:TimeZoneDefinitions>
                    <t:TimeZoneDefinition Id="Eastern Standard Time" Name="(GMT-05:00) Eastern Time (US &amp;amp; Canada)" />
                    <t:TimeZoneDefinition Id="Pacific Standard Time" Name="(GMT-08:00) Pacific Time (US &amp;amp; Canada)" />
                  </m:TimeZoneDefinitions>
                </m:GetServerTimeZonesResponseMessage>
              </m:ResponseMessages>
            </m:GetServerTimeZonesResponse>"#;

        let expected = GetServerTimeZonesResponse {
            response_messages: ResponseMessages {
                response_messages: vec![ResponseClass::Success(TimeZoneDefinitions {
                    inner: vec![
                        TimeZoneDefinition {
                            id: "Eastern Standard Time".to_string(),
                            name: Some(
                                r#"(GMT-05:00) Eastern Time (US &amp;amp; Canada)"#.to_string(),
                            ),
                        },
                        TimeZoneDefinition {
                            id: "Pacific Standard Time".to_string(),
                            name: Some(
                                r#"(GMT-08:00) Pacific Time (US &amp;amp; Canada)"#.to_string(),
                            ),
                        },
                    ],
                })],
            },
        };

        assert_deserialized_content(content, expected);
    }
}
