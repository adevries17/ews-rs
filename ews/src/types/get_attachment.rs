/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use ews_proc_macros::operation_response;
use serde::Deserialize;
use xml_struct::XmlSerialize;

use crate::{AttachmentId, AttachmentShape, Attachments, MESSAGES_NS_URI};

/// The GetAttachment element is the root element in a request to get an attachment from the Exchange store.
///
/// See <https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/getattachment>.
#[derive(Clone, Debug, XmlSerialize)]
#[xml_struct(default_ns = MESSAGES_NS_URI)]
#[operation_response(GetAttachmentResponseMessage)]
pub struct GetAttachment {
    pub attachment_shape: AttachmentShape,

    pub attachment_ids: AttachmentIds,
}

#[derive(Clone, Debug, XmlSerialize)]
pub struct AttachmentIds {
    #[xml_struct(flatten)]
    attachment_id: Vec<AttachmentId>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct GetAttachmentResponseMessage {
    pub attachments: Attachments,
}

#[cfg(test)]
mod tests {
    use crate::{
        test_utils::{assert_deserialized_content, assert_serialized_content},
        Attachment, ResponseClass, ResponseMessages,
    };

    use super::*;

    #[test]
    fn test_serialize_get_attachment() {
        let data = GetAttachment {
            attachment_shape: AttachmentShape {
                include_mime_content: None,
                body_type: None,
                filter_html_content: None,
                additional_properties: None,
            },
            attachment_ids: AttachmentIds {
                attachment_id: vec![AttachmentId {
                    id: "AAAtAEFkbWluaX".to_string(),
                    root_item_id: Some("AAADDDFFF".to_string()),
                    root_item_change_key: Some("AASDFASDF".to_string()),
                }],
            },
        };

        let expected_xml_content = r#"<GetAttachment xmlns="http://schemas.microsoft.com/exchange/services/2006/messages"><AttachmentShape></AttachmentShape><AttachmentIds><AttachmentId Id="AAAtAEFkbWluaX"/></AttachmentIds></GetAttachment>"#;

        assert_serialized_content(&data, "GetAttachment", expected_xml_content);
    }

    #[test]
    fn test_deserialize_get_attachment_response() {
        let content = r#"<GetAttachmentResponse xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages"
            xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
            xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
             <m:ResponseMessages>
               <m:GetAttachmentResponseMessage ResponseClass="Success">
                 <m:ResponseCode>NoError</m:ResponseCode>
                 <m:Attachments>
                   <t:FileAttachment>
                     <t:AttachmentId Id="AAAtAEFkbWluaX"/>
                     <t:Name>SomeFile</t:Name>
                     <t:Content>AQIDBAU=</t:Content>
                     <t:ContentType>HTML</t:ContentType>
                   </t:FileAttachment>
                 </m:Attachments>
               </m:GetAttachmentResponseMessage>
             </m:ResponseMessages>
           </GetAttachmentResponse>"#;

        let expected = GetAttachmentResponse {
            response_messages: ResponseMessages {
                response_messages: vec![ResponseClass::Success(GetAttachmentResponseMessage {
                    attachments: Attachments {
                        inner: vec![Attachment::FileAttachment {
                            attachment_id: AttachmentId {
                                id: "AAAtAEFkbWluaX".to_string(),
                                root_item_id: None,
                                root_item_change_key: None,
                            },
                            name: "SomeFile".to_string(),
                            content_type: "HTML".to_string(),
                            content_id: None,
                            content_location: None,
                            size: None,
                            last_modified_time: None,
                            is_inline: None,
                            is_contact_photo: None,
                            content: Some("AQIDBAU=".to_string()),
                        }],
                    },
                })],
            },
        };
        assert_deserialized_content(content, expected);
    }
}
