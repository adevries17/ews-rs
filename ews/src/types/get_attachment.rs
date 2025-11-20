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
    pub attachment_shape: Option<AttachmentShape>,

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
    use crate::test_utils::assert_serialized_content;

    use super::*;

    #[test]
    fn test_serialize_get_attachment_request() {
        let attach = AttachmentId {
            id: "AAAtAEFkbWluaX".to_string(),
            root_item_id: None,
            root_item_change_key: None,
        };
        let get_attachment = GetAttachment {
            attachment_shape: None,
            attachment_ids: vec![attach],
        };

        let expected = r#"<GetAttachment xmlns="http://schemas.microsoft.com/exchange/services/2006/messages"><AttachmentIds><t:AttachmentId Id="AAAtAEFkbWluaX"/></AttachmentIds></GetAttachment>"#;

        assert_serialized_content(&get_attachment, "GetAttachment", expected);
    }

    #[test]
    fn test_deserialize_get_attachment_response() {}
}
