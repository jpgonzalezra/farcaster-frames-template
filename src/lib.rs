use chrono::Utc;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::fmt;

pub fn get_character() -> String {
    let character = vec![
        "Luke Skywalker",
        "Leia Organa",
        "Han Solo",
        "Darth Vader",
        "Anakin Skywalker",
        "Obi-Wan Kenobi",
        "Yoda",
        "R2-D2",
        "C-3PO",
        "Chewbacca",
        "Palpatine",
        "Boba Fett",
        "Lando Calrissian",
        "Mace Windu",
        "Qui-Gon Jinn",
        "Padmé Amidala",
        "Ahsoka Tano",
        "Darth Maul",
        "Kylo Ren",
        "Rey",
        "Finn",
        "Poe Dameron",
        "BB-8",
        "Jabba the Hutt",
        "Wedge Antilles",
        "Greedo",
        "Jango Fett",
        "General Grievous",
        "Count Dooku",
        "Jar Jar Binks",
        "Kanan Jarrus",
        "Ezra Bridger",
        "Sabine Wren",
        "Hera Syndulla",
        "Chopper",
        "Zeb Orrelios",
        "Thrawn",
        "Rex",
        "Asajj Ventress",
        "Hondo Ohnaka",
        "Moff Gideon",
        "Cara Dune",
        "Grogu",
        "The Mandalorian (Din Djarin)",
        "IG-11",
        "Greef Karga",
    ];

    let starter = character.choose(&mut rand::thread_rng()).unwrap();
    starter.to_string()
}

#[derive(Debug, Serialize, Deserialize)]
struct PayloadTrustedData {
    #[serde(rename = "messageBytes")]
    message_bytes: String,
}

impl fmt::Display for PayloadTrustedData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Message Bytes: {}", self.message_bytes)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PayloadCastId {
    fid: i32,
    hash: String,
}

impl fmt::Display for PayloadCastId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FID: {}, Hash: {}", self.fid, self.hash)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PayloadUntrustedData {
    fid: i32,
    url: String,
    #[serde(rename = "messageHash")]
    message_hash: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    timestamp: chrono::DateTime<Utc>,
    network: i32,
    #[serde(rename = "buttonIndex")]
    button_index: i32,
    #[serde(rename = "castId")]
    cast_id: PayloadCastId,
    #[serde(rename = "inputText")]
    input_text: Option<String>,
}

impl fmt::Display for PayloadUntrustedData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FID: {}, URL: {}, Message Hash: {}, Timestamp: {}, Network: {}, Button Index: {}, Cast ID: [{}], Input Text: {}",
               self.fid, self.url, self.message_hash, self.timestamp, self.network, self.button_index, self.cast_id, self.input_text.clone().unwrap_or_default())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FrameActionPayload {
    #[serde(rename = "trustedData")]
    trusted_data: PayloadTrustedData,
    #[serde(rename = "untrustedData")]
    untrusted_data: PayloadUntrustedData,
}

impl fmt::Display for FrameActionPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Trusted Data: [{}], Untrusted Data: [{}]",
            self.trusted_data, self.untrusted_data
        )
    }
}

#[derive(Serialize)]
pub struct APIError {
    pub message: &'static str,
    pub code: &'static str,
}
