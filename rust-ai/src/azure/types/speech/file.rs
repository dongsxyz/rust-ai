use serde::{Deserialize, Serialize};

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
