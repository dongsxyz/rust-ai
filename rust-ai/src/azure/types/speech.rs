use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: Option<String>,
    #[serde(rename = "innerError")]
    pub inner_error: InnerError,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InnerError {
    pub code: String,
    pub message: Option<String>,
}
