//!
//! Given a prompt and/or an input image, the model will generate a new image.
//!
//! Related guide: Image generation
//!
//! Source: OpenAI documentation

////////////////////////////////////////////////////////////////////////////////

use crate::openai::{
    endpoint::{
        request_endpoint, request_endpoint_form_data, Endpoint, EndpointVariant,
        ImageEndpointVariant,
    },
    types::{
        common::Error,
        image::{Format, ImageResponse, Size},
    },
};
use log::{debug, warn};
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// Given a prompt and an instruction, the model will return an edited version
/// of the prompt.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    #[serde(skip)]
    image: Option<(String, Vec<u8>)>,

    #[serde(skip)]
    mask: Option<(String, Vec<u8>)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<Size>,

    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<Format>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            image: None,
            mask: None,
            prompt: None,
            size: None,
            response_format: None,
            user: None,
            n: None,
        }
    }
}

impl Image {
    /// Determine and verify MIME type of input file.
    ///
    /// # Arguments
    /// - `file_name` - Name of the input file
    fn mime(&self, file_name: &str) -> Result<&'static str, Box<dyn std::error::Error>> {
        Ok(
            match *file_name.split('.').collect::<Vec<&str>>().last().unwrap() {
                "png" => "image/png",
                _ => return Err("Unsupported format!".into()),
            },
        )
    }

    /// The image to edit. Must be a valid PNG file, less than 4MB, and square.
    ///  If mask is not provided, image must have transparency, which will be
    /// used as the mask.
    ///
    /// # Arguments
    /// - `filename` - Image filename to edit or create variant
    /// - `bytes` - Image bytes to edit or create variant
    pub fn image(self, filename: &str, bytes: Vec<u8>) -> Self {
        Self {
            image: Some((filename.into(), bytes.clone())),
            ..self
        }
    }

    /// An additional image whose fully transparent areas (e.g. where alpha is
    /// zero) indicate where `image` should be edited. Must be a valid PNG
    /// file, less than 4MB, and have the same dimensions as `image`.
    ///
    /// # Arguments
    /// - `filename` - Image filename to edit or create variant
    /// - `bytes` - Image bytes to edit or create variant
    pub fn mask(self, filename: &str, bytes: Vec<u8>) -> Self {
        Self {
            mask: Some((filename.into(), bytes.clone())),
            ..self
        }
    }

    /// A text description of the desired image(s). The maximum length is 1000
    /// characters.
    pub fn prompt(self, content: &str) -> Self {
        Self {
            prompt: Some(content.into()),
            ..self
        }
    }

    /// The number of images to generate. Must be between 1 and 10.
    pub fn n(self, n: u32) -> Self {
        Self { n: Some(n), ..self }
    }

    /// The size of the generated images. Must be one of `256x256`, `512x512`,
    /// or `1024x1024`. Use given enum variant.
    pub fn size(self, size: Size) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }

    /// The format in which the generated images are returned. Must be one of
    /// `url` or `b64_json`.
    pub fn response_format(self, response_format: Format) -> Self {
        Self {
            response_format: Some(response_format),
            ..self
        }
    }

    /// A unique identifier representing your end-user, which can help OpenAI
    /// to monitor and detect abuse. Learn more.
    pub fn user(self, user: &str) -> Self {
        Self {
            user: Some(user.into()),
            ..self
        }
    }

    /// Send edit request to OpenAI.
    pub async fn editing(&self) -> Result<ImageResponse, Box<dyn std::error::Error>> {
        if let None = self.image {
            return Err("`image` required, call `image()` first".into());
        }
        if let None = self.prompt {
            return Err("`prompt` required, call `prompt()` first".into());
        }

        let mut image_response: Option<ImageResponse> = None;

        let mut form = Form::new();

        if let Some(image_tup) = self.image.as_ref() {
            let image = Part::bytes(image_tup.1.clone())
                .file_name(image_tup.0.clone())
                .mime_str(self.mime(&image_tup.0).unwrap())
                .unwrap();
            form = form.part("image", image);
        }

        if let Some(mask_tup) = self.mask.as_ref() {
            let mask = Part::bytes(mask_tup.1.clone())
                .file_name(mask_tup.0.clone())
                .mime_str(self.mime(&mask_tup.0).unwrap())
                .unwrap();
            form = form.part("mask", mask);
        }

        if let Some(prompt) = self.prompt.clone() {
            form = form.part("prompt", Part::text(prompt));
        }

        if let Some(n) = self.n.clone() {
            form = form.part("n", Part::text(n.to_string()));
        }

        if let Some(size) = self.size.clone() {
            let size: &str = size.into();
            form = form.part("size", Part::text(size));
        }

        if let Some(fmt) = self.response_format.clone() {
            let fmt: &str = fmt.into();
            form = form.part("response_format", Part::text(fmt));
        }

        if let Some(user) = self.user.clone() {
            form = form.part("user", Part::text(user));
        }

        let variant: String = ImageEndpointVariant::Editing.into();
        request_endpoint_form_data(form, &Endpoint::Image_v1, EndpointVariant::from(variant), |res| {
            if let Ok(text) = res {
                if let Ok(response_data) = serde_json::from_str::<ImageResponse>(&text) {
                    debug!(target: "openai", "Response parsed, image edit response deserialized.");
                    image_response = Some(response_data);
                } else {
                    if let Ok(response_error) = serde_json::from_str::<Error>(&text) {
                        warn!(target: "openai",
                            "OpenAI error code {}: `{:?}`",
                            response_error.error.code.unwrap_or(0),
                            text
                        );
                    } else {
                        warn!(target: "openai", "Image response not deserializable.");
                    }
                }
            }
        })
        .await?;

        if let Some(response_data) = image_response {
            Ok(response_data)
        } else {
            Err("No response or error parsing response".into())
        }
    }

    /// Send generation request to OpenAI.
    pub async fn generation(&mut self) -> Result<ImageResponse, Box<dyn std::error::Error>> {
        self.image = None;
        self.mask = None;

        if let None = self.prompt {
            return Err("`prompt` required, call `prompt()` first".into());
        }

        let mut image_response: Option<ImageResponse> = None;

        let variant: String = ImageEndpointVariant::Generation.into();
        request_endpoint(&self, &Endpoint::Image_v1, EndpointVariant::from(variant), |res| {
            if let Ok(text) = res {
                if let Ok(response_data) = serde_json::from_str::<ImageResponse>(&text) {
                    debug!(target: "openai", "Response parsed, image generation response deserialized.");
                    image_response = Some(response_data);
                } else {
                    if let Ok(response_error) = serde_json::from_str::<Error>(&text) {
                        warn!(target: "openai",
                            "OpenAI error code {}: `{:?}`",
                            response_error.error.code.unwrap_or(0),
                            text
                        );
                    } else {
                        warn!(target: "openai", "Image response not deserializable.");
                    }
                }
            }
        })
        .await?;

        if let Some(response_data) = image_response {
            Ok(response_data)
        } else {
            Err("No response or error parsing response".into())
        }
    }

    /// Send variation request to OpenAI.
    pub async fn variation(&mut self) -> Result<ImageResponse, Box<dyn std::error::Error>> {
        if let None = self.image {
            return Err("`image` required, call `image()` first".into());
        }
        self.prompt = None;
        self.mask = None;

        let mut image_response: Option<ImageResponse> = None;

        let mut form = Form::new();

        if let Some(image_tup) = self.image.as_ref() {
            let image = Part::bytes(image_tup.1.clone())
                .file_name(image_tup.0.clone())
                .mime_str(self.mime(&image_tup.0).unwrap())
                .unwrap();
            form = form.part("image", image);
        }

        if let Some(n) = self.n.clone() {
            form = form.part("n", Part::text(n.to_string()));
        }

        if let Some(size) = self.size.clone() {
            let size: &str = size.into();
            form = form.part("size", Part::text(size));
        }

        if let Some(fmt) = self.response_format.clone() {
            let fmt: &str = fmt.into();
            form = form.part("response_format", Part::text(fmt));
        }

        if let Some(user) = self.user.clone() {
            form = form.part("user", Part::text(user));
        }

        let variant: String = ImageEndpointVariant::Variation.into();

        request_endpoint_form_data(form, &Endpoint::Image_v1, EndpointVariant::from(variant), |res| {
            if let Ok(text) = res {
                if let Ok(response_data) = serde_json::from_str::<ImageResponse>(&text) {
                    debug!(target: "openai", "Response parsed, image variation response deserialized.");
                    image_response = Some(response_data);
                } else {
                    if let Ok(response_error) = serde_json::from_str::<Error>(&text) {
                        warn!(target: "openai",
                            "OpenAI error code {}: `{:?}`",
                            response_error.error.code.unwrap_or(0),
                            text
                        );
                    } else {
                        warn!(target: "openai", "Image response not deserializable.");
                    }
                }
            }
        })
        .await?;

        if let Some(response_data) = image_response {
            Ok(response_data)
        } else {
            Err("No response or error parsing response".into())
        }
    }
}
