use openai_dive::v1::{
    api::Client,
    resources::chat::{
        ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage,
        ChatMessageContent, JsonSchemaBuilder,
    },
};
use reqwest::ClientBuilder;
use serde::{Deserialize, Serialize};

use tests::*;

mod tests;

#[derive(Deserialize, Serialize)]
pub struct FilterOutput {
    selected_tags: Vec<String>,
}

pub async fn make_search() {}

pub async fn make_filter(available_tags: Vec<String>, convo: &ExampleConvo) -> FilterOutput {
    let mut messages = convo.previous_messages.clone();
    messages.push(
        ChatMessage::User {
            content: ChatMessageContent::Text(format!("Based on the above conversastion, which of the following tags should I filter by? {:?}?", available_tags)),
            name: Some("search_expert".to_string())
        }
    );

    let parameters = ChatCompletionParametersBuilder::default()
        .model("gpt-4o-mini")
        .messages(messages)
        .response_format(ChatCompletionResponseFormat::JsonSchema(
            JsonSchemaBuilder::default()
                .name("math_reasoning")
                .schema(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "selected_tags": { "type": "array", "items": { "type": "string"} }
                    },
                    "required": ["selected_tags"],
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

    let result = client
        .chat()
        .create(parameters)
        .await
        .expect("failed to run chat");

    match result.choices.first().expect("HI").message.clone() {
        ChatMessage::Assistant {
            content: Some(ChatMessageContent::Text(content)),
            ..
        } => serde_json::from_str(&content).expect(""),
        _ => FilterOutput {
            selected_tags: vec![],
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
        println!("Applied filters {:?}", results.selected_tags);
        tag_filters = results.selected_tags;
    }

    println!("OUTPUT: \n\n");

    let latest_user_message = match conversastion
        .previous_messages
        .clone()
        .last()
        .expect("it is here")
    {
        ChatMessage::User {
            content: ChatMessageContent::Text(content),
            ..
        } => content.clone(),
        _ => unreachable!(),
    };

    // Search with Trieve
    let search_req_payload = serde_json::json!({
        "query": latest_user_message,
        "search_type": "hybrid",
        "filters": {
            "must": [
                {
                    "field": "tag_set",
                    "match_any": tag_filters
                }
            ]
        }
    });

    let reqwest_client = ClientBuilder::new()
        .build()
        .expect("Failed to create reqwest client");

    let mut header_map: reqwest::header::HeaderMap = reqwest::header::HeaderMap::new();
    header_map.insert(
        "Content-Type",
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    header_map.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(
            std::env::var("TRIEVE_API_KEY")
                .expect("TRIEVE_API_KEY is not set")
                .as_str(),
        )
        .unwrap(),
    );
    header_map.insert(
        "TR-Dataset",
        reqwest::header::HeaderValue::from_str(
            std::env::var("TRIEVE_DATASET_ID")
                .expect("TRIEVE_DATASET_ID is not set")
                .as_str(),
        )
        .unwrap(),
    );
    header_map.insert(
        "X-API-Version",
        reqwest::header::HeaderValue::from_static("V2"),
    );

    let search_req_resp = reqwest_client
        .post("https://api.trieve.ai/api/chunk/search")
        .headers(header_map)
        .body(serde_json::to_string(&search_req_payload).unwrap())
        .send()
        .await
        .expect("Failed to search")
        .text()
        .await
        .expect("Failed to get body");

    println!("search resp {:?}", search_req_resp);

    // let last_message = ChatMessageContent::Text(format!(
    //     "Here's my prompt: {} \n\n {} {}",
    //     user_message.clone(),
    //     rag_prompt,  // prompt
    //     rag_content, // chunks
    // ));
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
        make_a_search(),
        filtered_search(),
        just_respond(),
        image_search(),
        recomend_flooring()
    ];

    for example in examples {
        create_message(example).await;
    }
}
