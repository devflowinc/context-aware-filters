use openai_dive::v1::{
    api::Client,
    resources::chat::{
        ChatCompletionFunction, ChatCompletionParametersBuilder, ChatCompletionTool,
        ChatCompletionToolType, ChatMessage, ChatMessageContent, ChatMessageContentPart, ToolCall,
    },
};

pub async fn get_tool_calls(message_content: Vec<ChatMessageContentPart>) -> Option<Vec<ToolCall>> {
    let client = Client {
        headers: None,
        project: None,
        api_key: std::env::var("OPENAI_API_KEY").expect("Please add an api key"),
        http_client: reqwest::Client::new(),
        base_url: std::env::var("OPENAI_BASE_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string()),
        organization: None,
    };

    let messages = vec![ChatMessage::User {
        content: ChatMessageContent::ContentPart(message_content),
        name: None,
    }];

    let parameters = ChatCompletionParametersBuilder::default()
        .model("openai/gpt-4o-mini")
        .messages(messages)
        .tools(vec![ChatCompletionTool {
            r#type: ChatCompletionToolType::Function,
            function: ChatCompletionFunction {
                name: "filter".to_string(),
                description: Some("Decide on which filters to apply to available catalog being used within the knowledge base to respond. Always get filters.".to_string()),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "paint": {"type": "boolean", "description": "Whether or not the material being asked about is paint."},
                        "paper": {"type": "boolean", "description": "Whether or not the material being asked about is made of paper. It can frequently be a wallpaper."},
                        "countertops": {"type": "boolean", "description": "Whether or not the material being asked about is a countertop."},
                        "ceramic_tile": {"type": "boolean", "description": "Whether or not the material being asked about is ceramic tile."},
                        "hardwood": {"type": "boolean", "description": "Whether or not the material being asked about is hardwood."},
                        "flooring": {"type": "boolean", "description": "Whether or not the material being asked about is flooring."},
                        "felts": {"type": "boolean", "description": "Whether or not the material being asked about is felt."},
                        "textile": {"type": "boolean", "description": "Whether or not the material being asked about is textile."},
                        "wood": {"type": "boolean", "description": "Whether or not the material being asked about is wood."},
                        "fixtures": {"type": "boolean", "description": "Whether or not the material being asked about is a fixture."},
                        "cabinets": {"type": "boolean", "description": "Whether or not the material being asked about is a cabinet."},
                        "decking": {"type": "boolean", "description": "Whether or not the material being asked about is decking."},
                        "carpet": {"type": "boolean", "description": "Whether or not the material being asked about is carpet."},
                        "blinds": {"type": "boolean", "description": "Whether or not the material being asked about is blinds."},
                    },
                    "required": ["paper", "countertops", "ceramic_tile", "hardwood", "flooring", "felts", "textile", "wood", "fixtures", "cabinets", "decking", "carpet", "blinds"],
                }),
            },
        }])
        .build()
        .expect("failed to build parameters");

    let result = client
        .chat()
        .create(parameters)
        .await
        .expect("failed to get result");

    let message = result.choices[0].message.clone();

    match message {
        ChatMessage::Assistant {
            tool_calls: Some(tool_calls),
            ..
        } => Some(tool_calls),
        _ => None,
    }
}

#[tokio::main]
pub async fn main() {
    dotenvy::dotenv().expect("failed to load env vars");

    // TODO: test search/no-search function call

    // TODO: test countertops

    // TODO: test Ceramic Tile

    // TODO: test Hardwood

    // TODO: test flooring

    // TODO: test felts

    // TODO: test textile

    // TODO: test wood

    // TODO: test fixtures

    // TODO: test cabinets

    // TODO: test decking

    // TODO: test carpet

    // TODO: test blinds
}

#[cfg(test)]
mod tests {
    use ctor::ctor;
    use openai_dive::v1::resources::chat::{
        ChatMessageContentPart, ChatMessageImageContentPart, ChatMessageTextContentPart,
        ImageUrlType,
    };

    use super::*;

    #[ctor]
    fn init() {
        dotenvy::dotenv().expect("failed to load env vars");
    }

    // image: https://designshop.com/img/pdp/pdp-paint-sample-2.webp?quality=80&width=1900&format=webp
    // text: I want to make my walls look like this
    /// Test paint with an image and text message
    #[tokio::test]
    async fn test_paint_image_text() {
        let message_content_parts = vec![
            ChatMessageContentPart::Image(ChatMessageImageContentPart {
                r#type: "image_url".to_string(),
                image_url: ImageUrlType {
                    url: "https://designshop.com/img/pdp/pdp-paint-sample-2.webp?quality=80&width=1900&format=webp".to_string(),
                    detail: None,
                },
            }
            ),
            ChatMessageContentPart::Text(
            ChatMessageTextContentPart {
                r#type: "text".to_string(),
                text: "Get filters for the following message: \n\nI want to make my walls look like this".to_string(),
            }
        )];

        let tool_calls = get_tool_calls(message_content_parts).await;
        assert!(tool_calls.is_some());
        let tool_calls = tool_calls.unwrap();
        assert_eq!(tool_calls.len(), 1);
        assert_eq!(tool_calls[0].function.name, "filter");

        let params: serde_json::Value =
            serde_json::from_str(&tool_calls[0].function.arguments).unwrap();
        assert_eq!(
            params,
            serde_json::json!(
                {
                    "paint": true,
                    "paper": false,
                    "countertops": false,
                    "ceramic_tile": false,
                    "hardwood": false,
                    "flooring": false,
                    "felts": false,
                    "textile": false,
                    "wood": false,
                    "fixtures": false,
                    "cabinets": false,
                    "decking": false,
                    "carpet": false,
                    "blinds": false
                }
            )
        );
    }

    // text: I want to paint my walls
    /// Test paint with a text message
    #[tokio::test]
    async fn test_paint_text() {
        let message_content_parts =
            vec![ChatMessageContentPart::Text(ChatMessageTextContentPart {
                r#type: "text".to_string(),
                text: "Get filters for the following message: \n\nI want to paint my walls"
                    .to_string(),
            })];

        let tool_calls = get_tool_calls(message_content_parts).await;
        assert!(tool_calls.is_some());
        let tool_calls = tool_calls.unwrap();
        assert_eq!(tool_calls.len(), 1);
        assert_eq!(tool_calls[0].function.name, "filter");

        let params: serde_json::Value =
            serde_json::from_str(&tool_calls[0].function.arguments).unwrap();
        assert_eq!(
            params,
            serde_json::json!(
                {
                    "paint": true,
                    "paper": false,
                    "countertops": false,
                    "ceramic_tile": false,
                    "hardwood": false,
                    "flooring": false,
                    "felts": false,
                    "textile": false,
                    "wood": false,
                    "fixtures": false,
                    "cabinets": false,
                    "decking": false,
                    "carpet": false,
                    "blinds": false
                }
            )
        );
    }

    // image: https://cdn.designshop.com/DAMRoot/Original/10811/300017614_111_CInstall_01.webp?quality=80&width=1900&format=webp
    // text: I'm interested in what's on the wall
    /// Test paper with an image and text message
    #[tokio::test]
    async fn test_paper_image_text() {
        let message_content_parts = vec![
            ChatMessageContentPart::Image(ChatMessageImageContentPart {
                r#type: "image_url".to_string(),
                image_url: ImageUrlType {
                    url: "https://cdn.designshop.com/DAMRoot/Original/10811/300017614_111_CInstall_01.webp?quality=80&width=1900&format=webp".to_string(),
                    detail: None,
                },
            }
            ),
            ChatMessageContentPart::Text(
            ChatMessageTextContentPart {
                r#type: "text".to_string(),
                text: "Get filters for the following message: \n\nI'm interested in what's on the wall".to_string(),
            }
        )];

        let tool_calls = get_tool_calls(message_content_parts).await;
        assert!(tool_calls.is_some());
        let tool_calls = tool_calls.unwrap();
        assert_eq!(tool_calls.len(), 1);
        assert_eq!(tool_calls[0].function.name, "filter");

        let params: serde_json::Value =
            serde_json::from_str(&tool_calls[0].function.arguments).unwrap();
        assert_eq!(
            params,
            serde_json::json!(
                {
                    "paint": false,
                    "paper": true,
                    "countertops": false,
                    "ceramic_tile": false,
                    "hardwood": false,
                    "flooring": false,
                    "felts": false,
                    "textile": false,
                    "wood": false,
                    "fixtures": false,
                    "cabinets": false,
                    "decking": false,
                    "carpet": false,
                    "blinds": false
                }
            )
        );
    }

    // text: I'm looking for something to put on my wall
    /// Test paint and paper with a text message
    #[tokio::test]
    async fn test_paper_text() {
        let message_content_parts = vec![ChatMessageContentPart::Text(
            ChatMessageTextContentPart {
                r#type: "text".to_string(),
                text: "Get filters for the following message: \n\nI'm looking for something to put on my wall".to_string(),
            }
        )];

        let tool_calls = get_tool_calls(message_content_parts).await;
        assert!(tool_calls.is_some());
        let tool_calls = tool_calls.unwrap();
        assert_eq!(tool_calls.len(), 1);
        assert_eq!(tool_calls[0].function.name, "filter");

        let params: serde_json::Value =
            serde_json::from_str(&tool_calls[0].function.arguments).unwrap();
        assert_eq!(
            params,
            serde_json::json!(
                {
                    "paint": true,
                    "paper": true,
                    "countertops": false,
                    "ceramic_tile": false,
                    "hardwood": false,
                    "flooring": false,
                    "felts": false,
                    "textile": false,
                    "wood": false,
                    "fixtures": false,
                    "cabinets": false,
                    "decking": false,
                    "carpet": false,
                    "blinds": false
                }
            )
        );
    }

    // image
}
