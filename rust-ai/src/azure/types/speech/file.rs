use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

use super::transcription::{TranscriptionResult, TranscriptionReport};

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

    /// Format - int32. The duration in seconds that an SAS url should be valid.
    /// The default duration is 12 hours. When using BYOS (https://docs.microsoft.com/en-us/azure/cognitive-services/speech-service/speech-encryption-of-data-at-rest#bring-your-own-storage-byos-for-customization-and-logging):
    /// A value of 0 means that a plain blob URI without SAS token will be
    /// generated.
    #[serde(skip)]
    pub sas_validity_in_seconds: Option<u32>,
}

lazy_static! {
    static ref RE_ID_EXTRACT: Regex =
        Regex::new(r"/(?P<trans_id>[\da-z-]+)/files/(?P<file_id>[\da-z-]+)$").unwrap();
}

impl File {
    /// Get file ID from a File instance.
    pub fn file_id(&self) -> Result<(String, String), Box<dyn std::error::Error>> {
        if let Some(captures) = RE_ID_EXTRACT.captures(&self._self) {
            let error_message = match (captures.name("trans_id"), captures.name("file_id")) {
                (None, None) => "Neither transcription ID nor file ID found in `self`",
                (None, Some(_)) => "Transcription ID not found in `self`",
                (Some(_), None) => "File ID not found in `self`",
                (Some(trans_id), Some(file_id)) => {
                    return Ok((trans_id.as_str().into(), file_id.as_str().into()));
                }
            };
            Err(format!("{}: `{}`", error_message, self._self).into())
        } else {
            Err(format!("Incorrect format: `{}`", self._self).into())
        }
    }
}

/// Type of data.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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

/// Unofficial type representation of transcription result. Each variant 
/// contains correspondent data.
pub enum FileType {
    
    // /// Type of data is dataset report.
    // DatasetReport,

    // /// Type of data is audio.
    // Audio,

    // /// Type of data is language data.
    // LanguageData,

    // /// Type of data is pronunciation data.
    // PronunciationData,

    // /// Type of data is acoustic data archive.
    // AcousticDataArchive,

    // /// Type of data is acoustic data transcription v2.
   //  AcousticDataTranscriptionV2,

    /// Type of data is transcription.
    Transcription(TranscriptionResult),

    /// Type of data is transcription report.
    TranscriptionReport(TranscriptionReport),

    // /// Type of data is evaluation details.
    // EvaluationDetails,

    // /// Type of data is model report.
    // ModelReport,
}