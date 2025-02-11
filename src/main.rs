use openai_dive::v1::resources::chat::{ChatMessage, ChatMessageContent};

pub async fn make_search() {
}

pub async fn make_filter() {
}

pub async fn create_message(conversastion: ExampleConvo) {
    println!("Running conversastion for {}", conversastion.name);

    let last_message = ChatMessageContent::Text(format!(
        "Here's my prompt: {} \n\n {} {}",
        user_message.clone(),
        rag_prompt, // prompt
        rag_content, // chunks
    ));
}

#[derive(Default)]
pub struct ExampleConvo {
    pub name: String,
    pub messages: Vec<ChatMessage>,
    pub tags: Option<String>,
}

#[tokio::main]
pub async fn main() {
    let examples = vec![make_a_search(), filtered_search(), just_respond()];

    for example in examples {
        create_message(example).await;
    }
}

fn make_a_search() -> ExampleConvo {
    ExampleConvo {
        name: "Make a search".to_string(),
        messages: vec![
            ChatMessage::System {
                content: ChatMessageContent::Text(
                    " You are a chat bot designed to make my life easier.".to_string(),
                ),
                name: None,
            },
            ChatMessage::User {
                content: ChatMessageContent::Text(
                    " You are a chat bot designed to make my life easier. ".to_string(),
                ),
                name: None,
            },
        ],
        ..Default::default()
    }
}

fn filtered_search() -> ExampleConvo {
    ExampleConvo {
        name: "filtered_search".to_string(),
        messages: vec![
            ChatMessage::System {
                content: ChatMessageContent::Text(
                    " You are a chat bot designed to make my life easier.".to_string(),
                ),
                name: None,
            },
            ChatMessage::User {
                content: ChatMessageContent::Text(
                    " You are a chat bot designed to make my life easier. ".to_string(),
                ),
                name: None,
            },
        ],
        ..Default::default()
    }
}

fn just_respond() -> ExampleConvo {
    ExampleConvo {
        name: "Just respond".to_string(),
        messages: vec![
            ChatMessage::System {
                content: ChatMessageContent::Text(
                    " You are a chat bot designed to make my life easier.".to_string(),
                ),
                name: None,
            },
            ChatMessage::User {
                content: ChatMessageContent::Text(
                    " You are a chat bot designed to make my life easier. ".to_string(),
                ),
                name: None,
            },
        ],
        ..Default::default()
    }
}
