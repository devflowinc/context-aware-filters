use openai_dive::v1::resources::chat::{
    ChatMessage, ChatMessageContent, ChatMessageContentPart, ChatMessageImageContentPart,
    ImageUrlType,
};

use crate::ExampleConvo;

pub fn filtered_search() -> ExampleConvo {
    ExampleConvo {
        name: "filtered_search".to_string(),
        available_tags: Some(vec![
            "whiskey".to_string(),
            "beer".to_string(),
            "gin".to_string(),
        ]),
        previous_messages: vec![
            ChatMessage::System {
                content: ChatMessageContent::Text(
                    "Always respond no matter what. You are a sales assistant for DesignShop. Your goal is to help self DesignShop's products. Be professional, brief, and to the point. Always use more words than just 'here' for your links. Answer questions to the best of your ability and avoid listing. Try to stick to only highlighting 1 or 2 products.".to_string()
                ),
                name: None,
            },
            ChatMessage::User {
                content: ChatMessageContent::Text("What are some good whiskey's for beginners".to_string()),
                name: None,
            },
        ],
        ..Default::default()
    }
}

pub fn just_chat() -> ExampleConvo {
    ExampleConvo {
        name: "Just Chat".to_string(),
        available_tags: Some(vec![
            "whiskey".to_string(),
            "beer".to_string(),
            "gin".to_string(),
        ]),
        previous_messages: vec![
            ChatMessage::System {
                content: ChatMessageContent::Text(
                    "Always respond no matter what. You are a sales assistant for DesignShop. Your goal is to help self DesignShop's products. Be professional, brief, and to the point. Always use more words than just 'here' for your links. Answer questions to the best of your ability and avoid listing. Try to stick to only highlighting 1 or 2 products.".to_string()
                ),
                name: None,
            },
            ChatMessage::User {
                content: ChatMessageContent::Text("What are some good whiskey's for beginners".to_string()),
                name: None,
            },
            ChatMessage::Assistant {
                content: Some(ChatMessageContent::Text("I reccomend the Jameson for $30 or the or the Crown Royal Canadian Whisky for $29.".to_string())),
                name: None,
                refusal: None,
                audio: None,
                tool_calls: None,
                reasoning_content: None
            },
            ChatMessage::User {
                content: ChatMessageContent::Text("Why did you choose these?".to_string()),
                name: None,
            },
        ],
        ..Default::default()
    }
}

pub fn image_search() -> ExampleConvo {
    ExampleConvo {
        name: "Pick Hardwood Flooring".to_string(),
        available_tags: Some(vec![
            "Laminate Flooring".to_string(),
            "Vinyl Flooring".to_string(),
            "Carpet Tiles".to_string(),
            "Unfinished Hardwood Flooring".to_string(),
        ]),
        previous_messages: vec![
            ChatMessage::System {
                content: ChatMessageContent::Text(
                    "Always respond no matter what. You are a sales assistant for DesignShop. Your goal is to help self DesignShop's products. Be professional, brief, and to the point. Always use more words than just 'here' for your links. Answer questions to the best of your ability and avoid listing. Try to stick to only highlighting 1 or 2 products.".to_string()
                ),
                name: None,
            },
            ChatMessage::User {
                content: ChatMessageContent::ContentPart(vec![
                    ChatMessageContentPart::Image(ChatMessageImageContentPart {
                        r#type: "image_url".to_string(),
                        image_url: ImageUrlType {
                            url: "https://images.ctfassets.net/zkiqcy76d1dl/3PEVFibbe2TWLQyttg2Spu/0527c9af7d3a0398a87c1e7effaf58a3/new-image-3e2b1cf7-dc34-44a3-8ca8-039031a329e3?q=80&w=2800&fm=webp".to_string(),
                            detail: None,
                        },
                    }),
                ]),
                name: None,
            },
            ChatMessage::User {
                content: ChatMessageContent::Text("Help me find a paint similar to this color".to_string()),
                name: None
            }
        ],
        ..Default::default()
    }
}

pub fn recomend_flooring() -> ExampleConvo {
    ExampleConvo {
        name: "Pick Carpet flooring".to_string(),
        available_tags: Some(vec![
            "Laminate Flooring".to_string(),
            "Vinyl Flooring".to_string(),
            "Carpet Tiles".to_string(),
            "Unfinished Hardwood Flooring".to_string(),
            "Carpet".to_string(),
        ]),
        previous_messages: vec![
            ChatMessage::System {
                content: ChatMessageContent::Text(
                    "Always respond no matter what. You are a sales assistant for DesignShop. Your goal is to help self DesignShop's products. Be professional, brief, and to the point. Always use more words than just 'here' for your links. Answer questions to the best of your ability and avoid listing. Try to stick to only highlighting 1 or 2 products.".to_string()
                ),
                name: None,
            },
            ChatMessage::User {
                content: ChatMessageContent::ContentPart(vec![
                    ChatMessageContentPart::Image(ChatMessageImageContentPart {
                        r#type: "image_url".to_string(),
                        image_url: ImageUrlType {
                            url: "https://dma-render-stage.global.ssl.fastly.net/scenes/117/1727084965709_day.png?quality=80&width=816&format=webp".to_string(),
                            detail: None,
                        },
                    }),
                ]),
                name: None,
            },
            ChatMessage::User {
                content: ChatMessageContent::Text("I like how this flooring looks, please show me similar floors".to_string()),
                name: None
            }
        ],
        ..Default::default()
    }
}
