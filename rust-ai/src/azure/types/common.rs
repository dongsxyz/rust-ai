use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};

/// Available gender variants implemented for Azure.
#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female,
}

impl Into<String> for Gender {
    fn into(self) -> String {
        (match self {
            Self::Male => "Male",
            Self::Female => "Female",
        })
        .into()
    }
}

/// Describes what type of data is wrapped inside: byte vector, plain text
/// or JSON.
#[derive(Debug, Clone)]
pub enum ResponseType {
    Bytes(Vec<u8>),
    Text(String),
}

/// Describes how to treat the response data: as bytes or plain text/JSON.
#[derive(Debug, Clone)]
pub enum ResponseExpectation {
    Bytes,
    Text,
}

/// Microsoft output format definitions
///
/// The supported streaming and non-streaming audio formats are sent in each
/// request as the `X-Microsoft-OutputFormat` header. Each format incorporates a
/// bit rate and encoding type. The Speech service supports 48-kHz, 24-kHz,
/// 16-kHz, and 8-kHz audio outputs. Each prebuilt neural voice model is
/// available at 24kHz and high-fidelity 48kHz.
///
/// If you select 48kHz output format, the high-fidelity voice model with 48kHz
/// will be invoked accordingly. The sample rates other than 24kHz and 48kHz can
/// be obtained through upsampling or downsampling when synthesizing, for
/// example, 44.1kHz is downsampled from 48kHz.
///
/// If your selected voice and output format have different bit rates, the
/// audio is resampled as necessary. You can decode the
/// `ogg-24khz-16bit-mono-opus` format by using the Opus codec.
///
/// Source: <https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/rest-text-to-speech>
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum MicrosoftOutputFormat {
    /// Streaming output format `amr-wb-16000hz`
    Amr_Wb_16000hz,

    /// Streaming output format `audio-16khz-16bit-32kbps-mono-opus`
    Audio_16khz_16bit_32kbps_Mono_Opus,

    /// Streaming output format `audio-16khz-32kbitrate-mono-mp3`
    Audio_16khz_32kbitrate_Mono_Mp3,

    /// Streaming output format `audio-16khz-64kbitrate-mono-mp3`
    Audio_16khz_64kbitrate_Mono_Mp3,

    /// Streaming output format `audio-16khz-128kbitrate-mono-mp3`
    Audio_16khz_128kbitrate_Mono_Mp3,

    /// Streaming output format `audio-24khz-16bit-24kbps-mono-opus`
    Audio_24khz_16bit_24kbps_Mono_Opus,

    /// Streaming output format `audio-24khz-16bit-48kbps-mono-opus`
    Audio_24khz_16bit_48kbps_Mono_Opus,

    /// Streaming output format `audio-24khz-48kbitrate-mono-mp3`
    Audio_24khz_48kbitrate_Mono_Mp3,

    /// Streaming output format `audio-24khz-96kbitrate-mono-mp3`
    Audio_24khz_96kbitrate_Mono_Mp3,

    /// Streaming output format `audio-24khz-160kbitrate-mono-mp3`
    Audio_24khz_160kbitrate_Mono_Mp3,

    /// Streaming output format `audio-48khz-96kbitrate-mono-mp3`
    Audio_48khz_96kbitrate_Mono_Mp3,

    /// Streaming output format `audio-48khz-192kbitrate-mono-mp3`
    Audio_48khz_192kbitrate_Mono_Mp3,

    /// Streaming output format `ogg-16khz-16bit-mono-opus`
    Ogg_16khz_16bit_Mono_Opus,

    /// Streaming output format `ogg-24khz-16bit-mono-opus`
    Ogg_24khz_16bit_Mono_Opus,

    /// Streaming output format `ogg-48khz-16bit-mono-opus`
    Ogg_48khz_16bit_Mono_Opus,

    /// Streaming output format `raw-8khz-8bit-mono-alaw`
    Raw_8khz_8bit_Mono_Alaw,

    /// Streaming output format `raw-8khz-8bit-mono-mulaw`
    Raw_8khz_8bit_Mono_Mulaw,

    /// Streaming output format `raw-8khz-16bit-mono-pcm`
    Raw_8khz_16bit_Mono_Pcm,

    /// Streaming output format `raw-16khz-16bit-mono-pcm`
    Raw_16khz_16bit_Mono_Pcm,

    /// Streaming output format `raw-16khz-16bit-mono-truesilk`
    Raw_16khz_16bit_Mono_Truesilk,

    /// Streaming output format `raw-22050hz-16bit-mono-pcm`
    Raw_22050hz_16bit_Mono_Pcm,

    /// Streaming output format `raw-24khz-16bit-mono-pcm`
    Raw_24khz_16bit_Mono_Pcm,

    /// Streaming output format `raw-24khz-16bit-mono-truesilk`
    Raw_24khz_16bit_Mono_Truesilk,

    /// Streaming output format `raw-44100hz-16bit-mono-pcm`
    Raw_44100hz_16bit_Mono_Pcm,

    /// Streaming output format `raw-48khz-16bit-mono-pcm`
    Raw_48khz_16bit_Mono_Pcm,

    /// Streaming output format `webm-16khz-16bit-mono-opus`
    Webm_16khz_16bit_Mono_Opus,

    /// Streaming output format `webm-24khz-16bit-24kbps-mono-opus`
    Webm_24khz_16bit_24kbps_Mono_Opus,

    /// Streaming output format `webm-24khz-16bit-mono-opus`
    Webm_24khz_16bit_Mono_Opus,

    /// Non-streaming output format `riff-8khz-8bit-mono-alaw`
    Riff_8khz_8bit_Mono_Alaw,

    /// Non-streaming output format `riff-8khz-8bit-mono-mulaw`
    Riff_8khz_8bit_Mono_Mulaw,

    /// Non-streaming output format `riff-8khz-16bit-mono-pcm`
    Riff_8khz_16bit_Mono_Pcm,

    /// Non-streaming output format `riff-22050hz-16bit-mono-pcm`
    Riff_22050hz_16bit_Mono_Pcm,

    /// Non-streaming output format `riff-24khz-16bit-mono-pcm`
    Riff_24khz_16bit_Mono_Pcm,

    /// Non-streaming output format `riff-44100hz-16bit-mono-pcm`
    Riff_44100hz_16bit_Mono_Pcm,

    /// Non-streaming output format `riff-48khz-16bit-mono-pcm`
    Riff_48khz_16bit_Mono_Pcm,
}

impl Into<HeaderValue> for MicrosoftOutputFormat {
    fn into(self) -> HeaderValue {
        let s: String = self.into();
        s.as_str().parse().unwrap()
    }
}

impl Into<String> for MicrosoftOutputFormat {
    fn into(self) -> String {
        (match self {
            Self::Amr_Wb_16000hz => "amr-wb-16000hz",
            Self::Audio_16khz_16bit_32kbps_Mono_Opus => "audio-16khz-16bit-32kbps-mono-opus",
            Self::Audio_16khz_32kbitrate_Mono_Mp3 => "audio-16khz-32kbitrate-mono-mp3",
            Self::Audio_16khz_64kbitrate_Mono_Mp3 => "audio-16khz-64kbitrate-mono-mp3",
            Self::Audio_16khz_128kbitrate_Mono_Mp3 => "audio-16khz-128kbitrate-mono-mp3",
            Self::Audio_24khz_16bit_24kbps_Mono_Opus => "audio-24khz-16bit-24kbps-mono-opus",
            Self::Audio_24khz_16bit_48kbps_Mono_Opus => "audio-24khz-16bit-48kbps-mono-opus",
            Self::Audio_24khz_48kbitrate_Mono_Mp3 => "audio-24khz-48kbitrate-mono-mp3",
            Self::Audio_24khz_96kbitrate_Mono_Mp3 => "audio-24khz-96kbitrate-mono-mp3",
            Self::Audio_24khz_160kbitrate_Mono_Mp3 => "audio-24khz-160kbitrate-mono-mp3",
            Self::Audio_48khz_96kbitrate_Mono_Mp3 => "audio-48khz-96kbitrate-mono-mp3",
            Self::Audio_48khz_192kbitrate_Mono_Mp3 => "audio-48khz-192kbitrate-mono-mp3",
            Self::Ogg_16khz_16bit_Mono_Opus => "ogg-16khz-16bit-mono-opus",
            Self::Ogg_24khz_16bit_Mono_Opus => "ogg-24khz-16bit-mono-opus",
            Self::Ogg_48khz_16bit_Mono_Opus => "ogg-48khz-16bit-mono-opus",
            Self::Raw_8khz_8bit_Mono_Alaw => "raw-8khz-8bit-mono-alaw",
            Self::Raw_8khz_8bit_Mono_Mulaw => "raw-8khz-8bit-mono-mulaw",
            Self::Raw_8khz_16bit_Mono_Pcm => "raw-8khz-16bit-mono-pcm",
            Self::Raw_16khz_16bit_Mono_Pcm => "raw-16khz-16bit-mono-pcm",
            Self::Raw_16khz_16bit_Mono_Truesilk => "raw-16khz-16bit-mono-truesilk",
            Self::Raw_22050hz_16bit_Mono_Pcm => "raw-22050hz-16bit-mono-pcm",
            Self::Raw_24khz_16bit_Mono_Pcm => "raw-24khz-16bit-mono-pcm",
            Self::Raw_24khz_16bit_Mono_Truesilk => "raw-24khz-16bit-mono-truesilk",
            Self::Raw_44100hz_16bit_Mono_Pcm => "raw-44100hz-16bit-mono-pcm",
            Self::Raw_48khz_16bit_Mono_Pcm => "raw-48khz-16bit-mono-pcm",
            Self::Webm_16khz_16bit_Mono_Opus => "webm-16khz-16bit-mono-opus",
            Self::Webm_24khz_16bit_24kbps_Mono_Opus => "webm-24khz-16bit-24kbps-mono-opus",
            Self::Webm_24khz_16bit_Mono_Opus => "webm-24khz-16bit-mono-opus",

            Self::Riff_8khz_8bit_Mono_Alaw => "riff-8khz-8bit-mono-alaw",
            Self::Riff_8khz_8bit_Mono_Mulaw => "riff-8khz-8bit-mono-mulaw",
            Self::Riff_8khz_16bit_Mono_Pcm => "riff-8khz-16bit-mono-pcm",
            Self::Riff_22050hz_16bit_Mono_Pcm => "riff-22050hz-16bit-mono-pcm",
            Self::Riff_24khz_16bit_Mono_Pcm => "riff-24khz-16bit-mono-pcm",
            Self::Riff_44100hz_16bit_Mono_Pcm => "riff-44100hz-16bit-mono-pcm",
            Self::Riff_48khz_16bit_Mono_Pcm => "riff-48khz-16bit-mono-pcm",
        })
        .into()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServiceHealthResponse {
    /// Health status of the service.
    pub status: HealthStatus,

    /// Additional messages about the current service health.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Optional subcomponents of this service and their status.
    pub components: Vec<ComponentHealth>,
}

/// Subcomponent health status.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ComponentHealth {
    /// Health status of the component.
    pub status: HealthStatus,

    /// Additional messages about the current service component health.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// The name of the component.
    pub name: String,

    /// The type of this component.
    #[serde(rename = "type")]
    pub ty: String,
}

/// Health status of the service.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum HealthStatus {
    Unhealthy,
    Healthy,
    Degraded,
}

impl Into<String> for HealthStatus {
    fn into(self) -> String {
        (match self {
            Self::Degraded => "Degraded",
            Self::Healthy => "Healthy",
            Self::Unhealthy => "Unhealthy",
        })
        .into()
    }
}
