// https://developers.facebook.com/docs/messenger-platform/reference/webhook-events/messages
// https://developers.facebook.com/docs/messenger-platform/reference/webhook-events

use crate::core::Url;
use crate::errors::MessengerError;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub object: String,
    entry: Vec<EventEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventEntry {
    id: String,
    time: i64,
    messaging: Vec<Messaging>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Messaging {
    sender: MessageSender,
    recipient: MessageRecipient,
    message: Message,
    timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    mid: String,
    text: Option<String>,
    quick_reply: Option<QuickReply>,
    reply_to: Option<ReplyTo>,
    attachments: Option<Vec<Attachment>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuickReply {
    payload: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyTo {
    mid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageSender {
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(rename(deserialize = "type"))]
    attachment_type: String,
    payload: AttachmentPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentPayload {
    url: Url,
    title: Option<String>,
    sticker_id: Option<String>,
    #[serde(rename(deserialize = "coordinates.lat"))]
    coordinates_lat: Option<f64>,
    #[serde(rename(deserialize = "coordinates.long"))]
    coordinates_long: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageRecipient {
    id: String,
}

pub fn get_full_test_event() -> Event {
    Event {
            object: "page".to_owned(),
            entry: vec![EventEntry {
                id: "105265827748175".to_owned(),
                time: 1585340609078,
                messaging: vec![
                    Messaging {
                        sender: MessageSender {
                            id: "2854776677948847".to_owned(),
                        },
                        recipient: MessageRecipient {
                            id: "105265827748175".to_owned(),
                        },
                        timestamp: 1585340608887,
                        message: Message {
                            mid: "m_nsQsuEIA03t5MHUiKZbqqMV4k24EfYk_tE3C9TIH_MWdRN_dyfVCn6Y6fY_sJIuELdIvimLw4dg0Wyx36MGlJA".to_owned(),
                            text: None,
                            quick_reply: None,
                            reply_to: None,
                            attachments: Some(
                                vec![
                                    Attachment {
                                        attachment_type: "image".to_owned(),
                                        payload: AttachmentPayload {
                                            url: "https://data.jousse.org/autocomplete.png".to_owned(),
                                            title: None,
                                            sticker_id: None,
                                            coordinates_lat: None,
                                            coordinates_long: None,
                                        },
                                    },
                                    Attachment {
                                        attachment_type: "image".to_owned(),
                                        payload: AttachmentPayload {
                                            url: "https://scontent.xx.fbcdn.net/v/t1.15752-9/90985058_246558926513316_1639749626632339456_n.png?_nc_cat=111&_nc_sid=b96e70&_nc_ohc=eLeUQYlrvRMAX9a-Ann&_nc_ad=z-m&_nc_cid=0&_nc_zor=9&_nc_ht=scontent.xx&oh=06f1a949a0f28c1db01bb0edbc647c9d&oe=5EA2376B".to_owned(),
                                            title: None,
                                            sticker_id: None,
                                            coordinates_lat: None,
                                            coordinates_long: None,
                                        },
                                    },
                                    ]
                                )
                            }
                    }
                ]
            }
            ]
        }
}

pub fn parse_document(event: Event) -> Result<Vec<String>> {
    if event.object != "page" {
        Err(MessengerError {
            message: "Bad page type".to_owned(),
        })?
    }
    if event.entry.len() == 0 {
        Err(MessengerError {
            message: "No entry".to_owned(),
        })?
    }

    if event.entry[0].messaging.len() == 0 {
        Err(MessengerError {
            message: "No messaging entry".to_owned(),
        })?
    }

    match &event.entry[0].messaging[0].message.attachments {
        Some(attachments) => Ok(attachments.iter().map(|x| x.payload.url.clone()).collect()),
        _ => Ok(vec![]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bad_event() {
        let full_event = get_bad_object_event();
        assert_eq!(
            parse_document(full_event)
                .unwrap_err()
                .downcast_ref::<MessengerError>()
                .unwrap(),
            &MessengerError {
                message: "Bad page type".to_owned(),
            }
        );
    }

    #[test]
    fn test_full_event_is_ok() {
        let full_event = get_full_test_event();
        assert_eq!(parse_document(full_event).unwrap(),
        vec![
                "https://data.jousse.org/autocomplete.png".to_owned(),
                "https://scontent.xx.fbcdn.net/v/t1.15752-9/90985058_246558926513316_1639749626632339456_n.png?_nc_cat=111&_nc_sid=b96e70&_nc_ohc=eLeUQYlrvRMAX9a-Ann&_nc_ad=z-m&_nc_cid=0&_nc_zor=9&_nc_ht=scontent.xx&oh=06f1a949a0f28c1db01bb0edbc647c9d&oe=5EA2376B".to_owned()
        ])
    }

    fn get_bad_object_event() -> Event {
        let mut bad_event = get_full_test_event();
        bad_event.object = "bad".to_owned();
        bad_event
    }
}
