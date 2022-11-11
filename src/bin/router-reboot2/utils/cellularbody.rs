use chrono;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::utils::Empty;
use crate::utils::Value;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct CellularSmsMessageEnvelop {
    #[serde(rename = "GetCellularSmsMessage")]
    pub sms_message: Empty,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct Inbox {
    #[serde(rename = "Index")]
    pub index: Value,
    #[serde(rename = "Read")]
    pub read: Value,
    #[serde(rename = "TimeStamp")]
    pub timestamp: Value,
    #[serde(rename = "PhoneNumber")]
    pub phone_number: Value,
    #[serde(rename = "Message")]
    pub message: Value,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct InboxList {
    #[serde(rename = "InboxList")]
    pub items: Vec<Inbox>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct SmsMessageSummary {
    #[serde(rename = "UnreadNumber")]
    pub unread_number: Value,
    #[serde(rename = "ReceivedNumber")]
    pub received_number: Value,
    #[serde(rename = "SentNumber")]
    pub sent_number: Value,
    #[serde(rename = "RemainNumber")]
    pub remain_number: Value,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct SmsMessage {
    #[serde(rename = "InterfaceIndex")]
    pub interface_index: Value,
    #[serde(rename = "Summary")]
    pub summary: SmsMessageSummary,
    #[serde(rename = "InboxList")]
    pub inbox_list: InboxList,
}

impl fmt::Display for SmsMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "InterfaceIndex: {}", self.interface_index.0)?;
        let summary = &self.summary;
        writeln!(
            f,
            "Unread: {}, Received: {}, Sent: {}, Remain: {}",
            summary.unread_number.0,
            summary.remain_number.0,
            summary.sent_number.0,
            summary.remain_number.0
        )?;

        for i in &self.inbox_list.items {
            writeln!(f, "index: {}", i.index.0)?;
            let timestamp: i64 = i.timestamp.0.parse().unwrap_or(0);
            let datetime = chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0);
            match datetime {
                Some(datetime) => writeln!(f, "time: {}", datetime.format("%Y-%m-%d %H:%M:%S")),
                None => writeln!(f, "time: invalid"),
            }?;
            writeln!(f, "phone: {}", i.phone_number.0)?;
            writeln!(f, "message: {}", i.message.0)?;
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct CellularSmsMessageResponse {
    #[serde(rename = "GetCellularSmsMessageResult")]
    pub result: Value,
    #[serde(rename = "SmsMessage")]
    pub sms_message: Option<SmsMessage>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct CellularSmsMessageResponseEnvelop {
    #[serde(rename = "GetCellularSmsMessageResponse")]
    pub response: CellularSmsMessageResponse,
}
