use crate::xfyun::types::asr::{Language, ProfessionalDomain, AudioMode};

pub struct ASRBuilder {
  file: Vec<u8>,
  file_name: String,
  file_size: usize,
  duration: usize,
  language: Option<Language>,
  callback_url: Option<String>,
  hot_words: Option<Vec<String>>,
  multiple_candidates: bool,
  separate_roles: bool,
  roles: usize,
  pd: Option<ProfessionalDomain>,
  audio_mode: Option<AudioMode>,
  audio_url: Option<String>,
  standard_wav_type: Option<WavType>,
  language_mix_type: Option<LanguageMixType>,
}