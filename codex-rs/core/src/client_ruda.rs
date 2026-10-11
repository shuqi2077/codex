use codex_protocol::models::ContentItem;
use codex_protocol::models::FunctionCallOutputBody;
use codex_protocol::models::FunctionCallOutputContentItem;
use codex_protocol::models::ImageDetail;
use codex_protocol::models::ResponseItem;

// RUDA selects image resolution with the actual checkpoint processor rather than
// OpenAI's high/low/original presets. Preserve all image bytes and other history.
pub(super) fn prepare_input(input: &mut [ResponseItem]) {
    for item in input {
        match item {
            ResponseItem::Message { content, .. } => {
                for part in content {
                    if let ContentItem::InputImage { detail, .. } = part {
                        *detail = Some(ImageDetail::Auto);
                    }
                }
            }
            ResponseItem::FunctionCallOutput { output, .. }
            | ResponseItem::CustomToolCallOutput { output, .. } => {
                if let FunctionCallOutputBody::ContentItems(parts) = &mut output.body {
                    for part in parts {
                        if let FunctionCallOutputContentItem::InputImage { detail, .. } = part {
                            *detail = Some(ImageDetail::Auto);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
