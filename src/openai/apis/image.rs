//! 
//! Given a prompt and/or an input image, the model will generate a new image.
//! 
//! Related guide: Image generation
//! 
//! Source: OpenAI documentation

////////////////////////////////////////////////////////////////////////////////

use crate::openai::{
    endpoint::{request_endpoint, Endpoint, EndpointVariant, ImageEndpointVariant},
    types::{
        common::Error,
        image::{Format, ImageResponse, Size},
    },
};
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// Given a prompt and an instruction, the model will return an edited version
/// of the prompt.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    /// The image to edit. Must be a valid PNG file, less than 4MB, and square.
    ///  If mask is not provided, image must have transparency, which will be
    /// used as the mask.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    /// An additional image whose fully transparent areas (e.g. where alpha is
    /// zero) indicate where `image` should be edited. Must be a valid PNG
    /// file, less than 4MB, and have the same dimensions as `image`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,

    /// A text description of the desired image(s). The maximum length is 1000
    /// characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// The number of images to generate. Must be between 1 and 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    /// The size of the generated images. Must be one of `256x256`, `512x512`,
    /// or `1024x1024`. Use given enum variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Size>,

    /// The format in which the generated images are returned. Must be one of
    /// `url` or `b64_json`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<Format>,

    /// A unique identifier representing your end-user, which can help OpenAI
    /// to monitor and detect abuse. Learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
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
    /// Set target image.
    ///
    /// # Arguments
    /// - `image` - Image to edit or create variant
    pub fn set_image(&mut self, image: &str) {
        self.image = Some(image.into());
    }

    /// Set target image mask.
    ///
    /// # Arguments
    /// - `mask` - Image to edit or create variant
    pub fn set_image_mask(&mut self, mask: &str) {
        self.mask = Some(mask.into());
    }

    /// Set target prompt for image generations.
    ///
    /// # Arguments
    /// - `content` - Target prompt
    pub fn set_prompt(&mut self, content: &str) {
        self.prompt = Some(content.into());
    }

    /// Send edit request to OpenAI.
    pub async fn editing(&self) -> Result<ImageResponse, Box<dyn std::error::Error>> {
        if let None = self.image {
            return Err("`image` required, call `set_image` first".into());
        }
        if let None = self.prompt {
            return Err("`prompt` required, call `set_prompt` first".into());
        }

        let mut image_response: Option<ImageResponse> = None;

        let variant: String = ImageEndpointVariant::Editing.into();
        request_endpoint(&self, &Endpoint::Image_v1, EndpointVariant::from(variant), |res| {
            if let Ok(text) = res {
                if let Ok(response_data) = serde_json::from_str::<ImageResponse>(&text) {
                    debug!(target: "openai", "Response parsed, image edit response deserialized.");
                    image_response = Some(response_data);
                } else {
                    if let Ok(response_error) = serde_json::from_str::<Error>(&text) {
                        warn!(
                            "OpenAI error code {}: {:?}",
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
            return Err("`prompt` required, call `set_prompt` first".into());
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
                        warn!(
                            "OpenAI error code {}: {:?}",
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
            return Err("`image` required, call `set_image` first".into());
        }
        self.prompt = None;
        self.mask = None;

        let mut image_response: Option<ImageResponse> = None;

        let variant: String = ImageEndpointVariant::Variation.into();
        request_endpoint(&self, &Endpoint::Image_v1, EndpointVariant::from(variant), |res| {
            if let Ok(text) = res {
                if let Ok(response_data) = serde_json::from_str::<ImageResponse>(&text) {
                    debug!(target: "openai", "Response parsed, image variation response deserialized.");
                    image_response = Some(response_data);
                } else {
                    if let Ok(response_error) = serde_json::from_str::<Error>(&text) {
                        warn!(
                            "OpenAI error code {}: {:?}",
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
