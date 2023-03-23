use log::warn;

use crate::azure::{
    endpoint::{request_get_endpoint, SpeechServiceEndpoint},
    types::tts::Voice,
};

pub struct Speech {}

impl Speech {
    pub async fn get_voice_list() -> Result<Vec<Voice>, Box<dyn std::error::Error>> {
        let text = request_get_endpoint(&SpeechServiceEndpoint::Get_List_of_Voices).await?;
        match serde_json::from_str::<Vec<Voice>>(&text) {
            Ok(voices) => Ok(voices),
            Err(e) => {
                warn!(target: "azure", "Error parsing response: {:?}", e);
                Err("Unable to parse voice list, check log for details".into())
            }
        }
    }
}
