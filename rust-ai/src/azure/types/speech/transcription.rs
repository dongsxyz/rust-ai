use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::azure::Locale;
use lazy_static::lazy_static;
use regex::Regex;

use super::{
    diarization::DiarizationProperties,
    entity::{EntityError, EntityReference},
    filter::FilterOperator,
    language::LanguageIdentificationProperties,
    mode::{ProfanityFilterMode, PunctuationMode},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transcription {
    /// Audio files to transcript
    ///
    /// Either the `contentContainerUrl` or `contentUrls` property must be set.
    ///
    /// A list of content urls to get audio files to transcribe. Up to 1000
    /// urls are allowed.
    ///
    /// This property will not be returned in a response.
    #[serde(skip_serializing_if = "Option::is_none", rename = "contentUrls")]
    pub content_urls: Option<Vec<String>>,

    /// Container to audio files to transcript
    ///
    /// Either the `contentContainerUrl` or `contentUrls` property must be set.
    ///
    /// A URL for an Azure blob container that contains the audio files. A
    /// container is allowed to have a maximum size of 5GB and a maximum number
    /// of 10000 blobs.
    ///
    /// The maximum size for a blob is 2.5GB.
    ///
    /// Container SAS should contain 'r' (read) and 'l' (list) permissions.
    ///
    /// This property will not be returned in a response.
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "contentContainerUrl"
    )]
    pub content_container_url: Option<String>,

    /// The locale of the contained data. If Language Identification is used,
    /// this locale is used to transcribe speech for which no language could be
    /// detected.
    pub locale: Locale,

    /// The display name of the object.
    #[serde(rename = "displayName")]
    pub display_name: String,

    /// TranscriptionLinks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<TranscriptionLinks>,

    /// The location to get all files of this entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<TranscriptionProperties>,

    /// The location of the referenced entity.
    ///
    /// Must be a valid URI string.
    #[serde(rename = "self")]
    pub _self: Option<String>,

    /// EntityReference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<EntityReference>,

    /// EntityReference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<EntityReference>,

    /// EntityReference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<EntityReference>,

    /// The description of the object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The custom properties of this entity. The maximum allowed key length is
    /// 64 characters, the maximum
    ///
    /// allowed value length is 256 characters and the count of allowed entries
    /// is 10.
    #[serde(rename = "customProperties", skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<HashMap<String, String>>,

    /// The time-stamp when the current status was entered.
    ///
    /// The time stamp is encoded as ISO 8601 date and time format
    /// ("YYYY-MM-DDThh:mm:ssZ", see https://en.wikipedia.org/wiki/ISO_8601#Combined_date_and_time_representations).
    ///
    /// Must be a valid date-time string
    #[serde(rename = "lastActionDateTime", skip_serializing_if = "Option::is_none")]
    pub last_action_date_time: Option<String>,

    /// Describe the current state of the API
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,

    /// The time-stamp when the object was created.
    ///
    /// The time stamp is encoded as ISO 8601 date and time format
    /// ("YYYY-MM-DDThh:mm:ssZ", see https://en.wikipedia.org/wiki/ISO_8601#Combined_date_and_time_representations).
    ///
    /// Must be a valid date-time string
    #[serde(rename = "createdDateTime", skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,

    /// Format - int32. Number of datasets that will be skipped.
    #[serde(skip)]
    pub skip: Option<usize>,

    /// Format - int32. Number of datasets that will be skipped.
    #[serde(skip)]
    pub top: Option<usize>,

    /// A filtering expression for selecting a subset of the available files.
    ///
    /// -   **Supported properties**: name, createdDateTime, kind.
    /// -   **Operators**:
    ///     -   eq, ne are supported for all properties.
    ///     -   gt, ge, lt, le are supported for createdDateTime.
    ///     -   and, or, not are supported.
    /// -   **Example**: `filter=name eq 'myaudio.wav.json' and kind eq 'Transcription'`
    #[serde(skip)]
    pub filter: Option<FilterOperator>,

    /// Format - int32. The duration in seconds that an SAS url should be
    /// valid. The default duration is 12 hours. When using BYOS (https://docs.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-encryption-of-data-at-rest#bring-your-own-storage-byos-for-customization-and-logging):
    /// A value of 0 means that a plain blob URI without SAS token will be
    /// generated.
    #[serde(skip)]
    pub sas_validity_in_seconds: Option<u32>,
}

impl Into<String> for Transcription {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Default for Transcription {
    fn default() -> Self {
        Self {
            content_urls: None,
            content_container_url: None,
            locale: Locale::en_US,
            display_name: String::new(),
            links: None,
            properties: None,
            _self: None,
            model: None,
            project: None,
            dataset: None,
            description: None,
            custom_properties: None,
            last_action_date_time: None,
            status: None,
            created_date_time: None,
            skip: None,
            top: None,
            filter: None,
            sas_validity_in_seconds: None,
        }
    }
}

lazy_static! {
    static ref RE_TRANS_ID: Regex = Regex::new(r"/(?P<trans_id>[\da-z-]+)$").unwrap();
}

impl Transcription {
    /// Get transcription ID from a batch creation job.
    pub fn transcription_id(&self) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(_self) = self._self.clone() {
            if let Some(captures) = RE_TRANS_ID.captures(&_self) {
                if let Some(trans_id) = captures.name("trans_id") {
                    Ok(trans_id.as_str().into())
                } else {
                    Err(format!("Incorrect format: `{}`", _self).into())
                }
            } else {
                Err(format!("Incorrect format: `{}`", _self).into())
            }
        } else {
            Err("You must submit the batch create transcriptions request to obtain transcription ID".into())
        }
    }
}

/// Describe the current state of the API
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Status {
    NotStarted,
    Running,
    Succeeded,
    Failed,
}

impl Into<String> for Status {
    fn into(self) -> String {
        (match self {
            Self::NotStarted => "NotStarted",
            Self::Running => "Running",
            Self::Succeeded => "Succeeded",
            Self::Failed => "Failed",
        })
        .into()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TranscriptionLinks {
    /// The location to get all files of this entity.
    ///
    /// Should be a valid URI
    pub files: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TranscriptionProperties {
    /// A value indicating whether diarization (speaker identification) is
    /// requested. The default value is `false`.
    ///
    /// If this field is set to true and the improved diarization system is
    /// configured by specifying `DiarizationProperties`, the improved
    /// diarization system will provide diarization for a configurable range of
    /// speakers.
    ///
    /// If this field is set to true and the improved diarization system is not
    /// enabled (not specifying `DiarizationProperties`), the basic diarization
    /// system will distinguish between up to two speakers.
    ///
    /// No extra charges are applied for the basic diarization.
    ///
    /// The basic diarization system is deprecated and will be removed in the
    /// next major version of the API.
    ///
    /// This `diarizationEnabled` setting will also be removed.
    #[serde(rename = "diarizationEnabled", skip_serializing_if = "Option::is_none")]
    pub diarization_enabled: Option<bool>,

    /// A value indicating whether word level timestamps are requested. The
    /// default value is `false`.
    #[serde(
        rename = "wordLevelTimestampsEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub word_level_timestamps_enabled: Option<bool>,

    /// A value indicating whether word level timestamps for the display form
    /// are requested. The default value is `false`.
    #[serde(
        rename = "displayFormWordLevelTimestampsEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_form_word_level_timestamps_enabled: Option<bool>,

    /// The duration of the transcription. The duration is encoded as ISO 8601
    /// duration ("PnYnMnDTnHnMnS", see https://en.wikipedia.org/wiki/ISO_8601#Durations).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,

    /// A collection of the requested channel numbers. In the default case, the
    /// channels 0 and 1 are considered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<usize>>,

    /// The requested destination container.
    ///
    /// ### Remarks
    ///
    /// When a destination container is used in combination with a
    /// `timeToLive`, the metadata of a transcription will be deleted normally,
    /// but the data stored in the destination container, including
    /// transcription results, will remain untouched, because no delete
    /// permissions are required for this container.  
    ///
    /// To support automatic cleanup, either configure blob lifetimes on the
    /// container, or use "Bring your own Storage (BYOS)" instead of
    /// `destinationContainerUrl`, where blobs can be cleaned up.
    #[serde(
        rename = "destinationContainerUrl",
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_container_url: Option<String>,

    /// The mode used for punctuation.
    #[serde(rename = "punctuationMode", skip_serializing_if = "Option::is_none")]
    pub punctuation_mode: Option<PunctuationMode>,

    /// Mode of profanity filtering.
    #[serde(
        rename = "profanityFilterMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub profanity_filter_mode: Option<ProfanityFilterMode>,

    /// How long the transcription will be kept in the system after it has
    /// completed. Once the transcription reaches the time to live after
    /// completion (successful or failed) it will be automatically deleted. Not
    /// setting this value or setting it to 0 will disable automatic deletion.
    /// The longest supported duration is 31 days.
    ///
    /// The duration is encoded as ISO 8601 duration ("PnYnMnDTnHnMnS", see https://en.wikipedia.org/wiki/ISO_8601#Durations).
    #[serde(rename = "timeToLive", skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<String>,

    /// DiarizationProperties
    pub diariaztion: Option<DiarizationProperties>,

    /// LanguageIdentificationProperties
    #[serde(
        rename = "languageIdentification",
        skip_serializing_if = "Option::is_none"
    )]
    pub language_identification: Option<LanguageIdentificationProperties>,

    /// The email address to send email notifications to in case the operation
    /// completes.
    ///
    /// The value will be removed after successfully sending the email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<EntityError>,
}

impl Default for TranscriptionProperties {
    fn default() -> Self {
        Self {
            diarization_enabled: None,
            word_level_timestamps_enabled: None,
            display_form_word_level_timestamps_enabled: None,
            duration: None,
            channels: None,
            destination_container_url: None,
            punctuation_mode: None,
            profanity_filter_mode: None,
            time_to_live: None,
            diariaztion: None,
            language_identification: None,
            email: None,
            error: None,
        }
    }
}
