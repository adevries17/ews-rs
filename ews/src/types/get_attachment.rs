use ews_proc_macros::operation_response;
use serde::Deserialize;
use xml_struct::XmlSerialize;

use crate::{AttachmentId, AttachmentShape, Attachments, MESSAGES_NS_URI};

/// root element in a request to get an attachment from the Exchange store.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getattachment>
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
#[operation_response(GetAttachmentResponseMessage)]
pub struct GetAttachment {
    /// Identifies additional properties to return in a response to a `GetAttachment` request.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/attachmentshape>
    pub attachment_shape: AttachmentShape,

    /// Contains an array of attachment identifiers.
    ///
    /// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/attachmentids>
    pub attachment_ids: Vec<AttachmentId>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct GetAttachmentResponseMessage {
    pub attachments: Attachments,
}

#[cfg(test)]
mod tests {
    use time::{Date, Month, OffsetDateTime, Time};

    use crate::{
        test_utils::{assert_deserialized_content, assert_serialized_content},
        Attachment, DateTime, ResponseClass, ResponseMessages,
    };

    use super::*;

    #[test]
    fn test_serialize_get_attachment_request() {
        let get_attachment = GetAttachment {
            attachment_shape: AttachmentShape::default(),
            attachment_ids: vec![
                AttachmentId {
                    id: "AAAtAEFkbWluaX".to_string(),
                    root_item_id: None,
                    root_item_change_key: None,
                },
                AttachmentId {
                    id: "AASSDDFF".to_string(),
                    root_item_id: None,
                    root_item_change_key: None,
                },
            ],
        };

        let expected = r#"<GetAttachment xmlns="http://schemas.microsoft.com/exchange/services/2006/messages"><AttachmentShape/><AttachmentIds><AttachmentId Id="AAAtAEFkbWluaX"/><AttachmendId Id="AASSDDFF"/></AttachmentIds></GetAttachment>"#;

        assert_serialized_content(&get_attachment, "GetAttachment", expected);
    }

    #[test]
    fn test_deserialize_get_attachment_response() {
        let content = r##"
            <GetAttachmentResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
                                   xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
              <m:ResponseMessages>
                <m:GetAttachmentResponseMessage ResponseClass="Success">
                  <m:ResponseCode>NoError</m:ResponseCode>
                  <m:Attachments>
                  <t:FileAttachment>
                    <t:AttachmentId Id="AAMkADE1NjRiYT"/>
                      <t:Name>photo.png</t:Name>
                      <t:ContentType>application/octet-stream</t:ContentType>
                      <t:Size>51199</t:Size>
                      <t:LastModifiedTime>2025-08-11T11:38:22Z</t:LastModifiedTime>
                      <t:IsInline>false</t:IsInline>
                      <t:IsContactPhoto>false</t:IsContactPhoto>
                    </t:FileAttachment>
                  </m:Attachments>
                </m:GetAttachmentResponseMessage>
              </m:ResponseMessages>
            </GetAttachmentResponse>"##;

        let expected = GetAttachmentResponse {
            response_messages: ResponseMessages {
                response_messages: vec![ResponseClass::Success(GetAttachmentResponseMessage {
                    attachments: Attachments {
                        inner: vec![Attachment::FileAttachment {
                            attachment_id: AttachmentId {
                                id: "AAMkADE1NjRiYT".to_string(),
                                root_item_id: None,
                                root_item_change_key: None,
                            },
                            name: "photo.png".to_string(),
                            content: None,
                            content_type: "application/octet-stream".to_string(),
                            content_id: None,
                            content_location: None,
                            size: Some(51199),
                            last_modified_time: Some(DateTime(OffsetDateTime::new_utc(
                                Date::from_calendar_date(2025, Month::August, 11).unwrap(),
                                Time::from_hms(11, 38, 22).unwrap(),
                            ))),
                            is_inline: Some(false),
                            is_contact_photo: Some(false),
                        }],
                    },
                })],
            },
        };
        assert_deserialized_content(content, expected);
    }
}
