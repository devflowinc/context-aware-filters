use openai_dive::v1::{
    api::Client,
    resources::chat::{
        ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage,
        ChatMessageContent, JsonSchemaBuilder,
    },
};
use serde::{Deserialize, Serialize};

use tests::*;

mod tests;

#[derive(Deserialize, Serialize, Debug)]
pub struct FilterOutput {
    selected_tag: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RepeatSearch {
    search_catagory: bool,
}

pub async fn make_search(convo: &ExampleConvo) -> RepeatSearch {
    let mut messages = convo.previous_messages.clone();
    messages.push(ChatMessage::User {
        content: ChatMessageContent::Text(format!(
            "Given the following messages, should a search be conducted to gather more context?"
        )),
        name: Some("search_expert".to_string()),
    });

    let parameters = ChatCompletionParametersBuilder::default()
        .model("gpt-4o")
        .messages(messages)
        .response_format(ChatCompletionResponseFormat::JsonSchema(
            JsonSchemaBuilder::default()
                .name("math_reasoning")
                .schema(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "search_catagory": { "type": "boolean"}
                    },
                    "required": ["search_catagory"],
                    "additionalProperties": false
                }))
                .strict(true)
                .build()
                .expect("failed to build schema"),
        ))
        .build()
        .expect("failed to build parameters");

    let client = Client {
        headers: None,
        project: None,
        api_key: std::env::var("OPENAI_API_KEY").expect("Please add an api key"),
        http_client: reqwest::Client::new(),
        base_url: "https://api.openai.com/v1".to_string(),
        organization: None,
    };

    let result = client.chat().create(parameters).await.unwrap();

    match result.choices.first().unwrap().message.clone() {
        ChatMessage::Assistant {
            content: Some(ChatMessageContent::Text(content)),
            ..
        } => serde_json::from_str(&content).unwrap(),
        _ => unreachable!(),
    }
}

pub async fn make_filter(available_tags: Vec<String>, convo: &ExampleConvo) -> FilterOutput {
    let mut messages = convo.previous_messages.clone();
    messages.push(
        ChatMessage::User {
            content: ChatMessageContent::Text(format!("Based on the above conversastion, which of the following tags should I filter by? {:?}? Only give me 1", available_tags)),
            name: Some("search_expert".to_string())
        }
    );

    let parameters = ChatCompletionParametersBuilder::default()
        .model("gpt-4o")
        .messages(messages)
        .response_format(ChatCompletionResponseFormat::JsonSchema(
            JsonSchemaBuilder::default()
                .name("math_reasoning")
                .schema(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "selected_tag": { "type": "string"}
                    },
                    "required": ["selected_tag"],
                    "additionalProperties": false
                }))
                .strict(true)
                .build()
                .expect("failed to build schema"),
        ))
        .build()
        .expect("failed to build parameters");

    let client = Client {
        headers: None,
        project: None,
        api_key: std::env::var("OPENAI_API_KEY").expect("Please add an api key"),
        http_client: reqwest::Client::new(),
        base_url: "https://api.openai.com/v1".to_string(),
        organization: None,
    };

    let result = client.chat().create(parameters).await.unwrap();

    match result.choices.first().unwrap().message.clone() {
        ChatMessage::Assistant {
            content: Some(ChatMessageContent::Text(content)),
            ..
        } => serde_json::from_str(&content).unwrap(),
        _ => FilterOutput {
            selected_tag: "".to_string(),
        },
    }
}

pub async fn create_message(conversastion: ExampleConvo) {
    println!(
        "------------------------- Running conversastion for {} -------------------------------",
        conversastion.name
    );
    let mut tag_filters: Vec<String> = vec![];
    if let Some(tags) = conversastion.available_tags.clone() {
        let results = make_filter(tags.clone(), &conversastion).await;
        tag_filters.push(results.selected_tag);
    }

    let should_search = make_search(&conversastion).await;
    println!("tag_set filter of {:?}", tag_filters);
    println!("Should search {:?}", should_search);

    println!("\n\n");
}

#[derive(Default, Clone)]
pub struct ExampleConvo {
    pub name: String,
    pub previous_messages: Vec<ChatMessage>,
    pub available_tags: Option<Vec<String>>,
}

#[tokio::main]
pub async fn main() {
    dotenvy::dotenv().expect("failed to load env vars");
    let examples = vec![
        filtered_search(),
        image_search(),
        recomend_flooring(),
        just_chat(),
    ];

    for example in examples {
        create_message(example).await;
    }
}
