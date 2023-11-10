use crate::mytype::*;

pub fn transfer_single_message_to_gpt_request_body(message: String) -> GPTRequestBody {
    let gpt_message = GPTMessage::default(message);
    let messages = vec![gpt_message];
    let gpt_model = GPTModel::default();
    GPTRequestBody::new(gpt_model, messages)
}
