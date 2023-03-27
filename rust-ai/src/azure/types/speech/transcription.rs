use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::azure::Locale;
use lazy_static::lazy_static;
use regex::Regex;

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
        }
    }
}

lazy_static! {
    static ref RE_TRANS_ID: Regex = Regex::new(r"/(?P<trans_id>[\da-z-]+)$").unwrap();
}

impl Transcription {
    /// Get transcription ID from a batch creation job.
    pub fn transcription_id(self) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(_self) = self._self {
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

/// The mode used for punctuation.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PunctuationMode {
    /// No punctuation.
    None,

    /// Dictated punctuation marks only, i.e., explicit punctuation.
    Dictated,

    /// Automatic punctuation.
    Automatic,

    /// Dictated punctuation marks or automatic punctuation.
    DictatedAndAutomatic,
}

impl Into<String> for PunctuationMode {
    fn into(self) -> String {
        (match self {
            Self::None => "None",
            Self::Dictated => "Dictated",
            Self::Automatic => "Automatic",
            Self::DictatedAndAutomatic => "DictatedAndAutomatic",
        })
        .into()
    }
}

/// ProfanityFilterMode
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ProfanityFilterMode {
    /// Disable profanity filtering.
    None,

    /// Remove profanity.
    Removed,

    /// Add "profanity" XML tags</Profanity>
    Tags,

    /// Mask the profanity with * except of the first letter, e.g., f***
    Masked,
}

impl Into<String> for ProfanityFilterMode {
    fn into(self) -> String {
        (match self {
            Self::None => "None",
            Self::Removed => "Removed",
            Self::Tags => "Tags",
            Self::Masked => "Masked",
        })
        .into()
    }
}

/// DiarizationProperties
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DiarizationProperties {
    /// DiarizationSpeakersProperties
    pub speakers: DiarizationSpeakersProperties,
}

impl Default for DiarizationProperties {
    fn default() -> Self {
        Self {
            speakers: DiarizationSpeakersProperties::default(),
        }
    }
}

/// DiarizationSpeakersProperties
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DiarizationSpeakersProperties {
    /// A hint for the minimum number of speakers for diarization. Must be
    /// smaller than or equal to the maxSpeakers property.
    ///
    /// Minimal: 1
    #[serde(rename = "minCount")]
    pub min_count: usize,

    /// The maximum number of speakers for diarization. Must be less than 36 and
    /// larger than or equal to the minSpeakers property.
    ///
    /// Minimal: 1
    #[serde(rename = "maxCount")]
    pub max_count: usize,
}

impl Default for DiarizationSpeakersProperties {
    fn default() -> Self {
        Self {
            min_count: 1,
            max_count: 1,
        }
    }
}
/// LanguageIdentificationProperties
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LanguageIdentificationProperties {
    /// The candidate locales for language identification (example ["en-US",
    /// "de-DE", "es-ES"]). A minimum of 2 and a maximum of 10 candidate
    /// locales, including the main locale for the transcription, is supported.
    #[serde(rename = "candidateLocales")]
    pub candidate_locales: Vec<String>,

    /// An optional mapping of locales to speech model entities. If no model is
    /// given for a locale, the default base model is used.
    ///
    /// Keys must be locales contained in the candidate locales, values are
    /// entities for models of the respective locales.
    #[serde(rename = "speechModelMapping")]
    pub speech_model_mapping: HashMap<String, EntityReference>,
}

impl Default for LanguageIdentificationProperties {
    fn default() -> Self {
        Self {
            candidate_locales: vec![],
            speech_model_mapping: HashMap::<String, EntityReference>::new(),
        }
    }
}

/// An optional mapping of locales to speech model entities. If no model is
/// given for a locale, the default base model is used.
///
/// Keys must be locales contained in the candidate locales, values are
/// entities for models of the respective locales.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntityReference {
    /// The location of the referenced entity.
    ///
    /// Must be a valid URI string.
    #[serde(rename = "self")]
    pub _self: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntityError {
    /// The code of this error.
    pub code: String,

    /// The message for this error.
    pub description: String,
}

impl Default for EntityError {
    fn default() -> Self {
        Self {
            code: String::new(),
            description: String::new(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaginatedFiles {
    pub values: Vec<File>,

    /// A link to the next set of paginated results if there are more entities 
    /// available; otherwise null.
    #[serde(rename = "@nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    /// Type of data.
    pub kind: FileKind,

    /// FileLinks
    pub links: FileLinks,

    /// The creation time of this file.
    ///
    /// The time stamp is encoded as ISO 8601 date and time format
    /// ("YYYY-MM-DDThh:mm:ssZ", see https://en.wikipedia.org/wiki/ISO_8601#Combined_date_and_time_representations).
    ///
    /// Must be a valid date-time string
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,

    /// FileProperties
    pub properties: FileProperties,

    /// The name of this file.
    pub name: String,

    /// The location of this entity.
    ///
    /// Must be a valid URI string.
    #[serde(rename = "self")]
    pub _self: String,
}

/// Type of data.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum FileKind {
    /// Type of data is dataset report.
    DatasetReport,

    /// Type of data is audio.
    Audio,

    /// Type of data is language data.
    LanguageData,

    /// Type of data is pronunciation data.
    PronunciationData,

    /// Type of data is acoustic data archive.
    AcousticDataArchive,

    /// Type of data is acoustic data transcription v2.
    AcousticDataTranscriptionV2,

    /// Type of data is transcription.
    Transcription,

    /// Type of data is transcription report.
    TranscriptionReport,

    /// Type of data is evaluation details.
    EvaluationDetails,

    /// Type of data is model report.
    ModelReport,
}

impl Into<String> for FileKind {
    fn into(self) -> String {
        (match self {
            Self::DatasetReport => "DatasetReport",
            Self::Audio => "Audio",
            Self::LanguageData => "LanguageData",
            Self::PronunciationData => "PronunciationData",
            Self::AcousticDataArchive => "AcousticDataArchive",
            Self::AcousticDataTranscriptionV2 => "AcousticDataTranscriptionV2",
            Self::Transcription => "Transcription",
            Self::TranscriptionReport => "TranscriptionReport",
            Self::EvaluationDetails => "EvaluationDetails",
            Self::ModelReport => "ModelReport",
        })
        .into()
    }
}

/// FileLinks
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileLinks {
    /// The url to retrieve the content of this file.
    #[serde(rename = "contentUrl")]
    pub content_url: String,
}

/// FileProperties
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileProperties {
    /// The size of the data in bytes.
    pub size: u32,

    /// The duration in case this file is an audio file. The duration is encoded
    /// as ISO 8601 duration ("PnYnMnDTnHnMnS", see https://en.wikipedia.org/wiki/ISO_8601#Durations).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}
