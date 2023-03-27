pub mod transcription;

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

#[derive(Clone, Debug)]
pub enum FilterField {
    DisplayName,
    Description,
    CreatedDateTime,
    LastActionDateTime,
    Status,
    Locale,
}

impl Into<String> for FilterField {
    fn into(self) -> String {
        (match self {
            Self::DisplayName => "displayName",
            Self::Description => "description",
            Self::CreatedDateTime => "createdDateTime",
            Self::LastActionDateTime => "lastActionDateTime",
            Self::Status => "status",
            Self::Locale => "locale",
        })
        .into()
    }
}

#[derive(Clone, Debug)]
pub enum FilterOperator {
    Eq(FilterField, String),
    Ne(FilterField, String),
    Gt(FilterField, String),
    Ge(FilterField, String),
    Lt(FilterField, String),
    Le(FilterField, String),
    And(Box<FilterOperator>, Box<FilterOperator>),
    Or(Box<FilterOperator>, Box<FilterOperator>),
    Not(Box<FilterOperator>),
}
impl FilterOperator {
    pub fn and(self, op: FilterOperator) -> Self {
        Self::And(Box::new(self), Box::new(op))
    }
    pub fn or(self, op: FilterOperator) -> Self {
        Self::Or(Box::new(self), Box::new(op))
    }
    pub fn not(self) -> Self {
        Self::Not(Box::new(self))
    }

    fn str(self, not: bool) -> String {
        match self {
            Self::And(a, b) => {
                if not {
                    format!("{} or {}", a.str(true), b.str(true))
                } else {
                    format!("{} and {}", a.str(false), b.str(false))
                }
            }

            Self::Or(a, b) => {
                if not {
                    format!("{} and {}", a.str(true), b.str(true))
                } else {
                    format!("{} or {}", a.str(false), b.str(false))
                }
            }

            Self::Not(a) => format!("{}", a.str(!not)),

            Self::Eq(field, value) => format!(
                "{} {} '{}'",
                Into::<String>::into(field),
                if not { "ne" } else { "eq" },
                Into::<String>::into(value)
            ),
            Self::Ne(field, value) => format!(
                "{} {} '{}'",
                Into::<String>::into(field),
                if not { "eq" } else { "ne" },
                Into::<String>::into(value)
            ),
            Self::Gt(field, value) => format!(
                "{} {} '{}'",
                Into::<String>::into(field),
                if not { "le" } else { "gt" },
                Into::<String>::into(value)
            ),
            Self::Ge(field, value) => format!(
                "{} {} '{}'",
                Into::<String>::into(field),
                if not { "lt" } else { "ge" },
                Into::<String>::into(value)
            ),
            Self::Lt(field, value) => format!(
                "{} {} '{}'",
                Into::<String>::into(field),
                if not { "ge" } else { "lt" },
                Into::<String>::into(value)
            ),
            Self::Le(field, value) => format!(
                "{} {} '{}'",
                Into::<String>::into(field),
                if not { "gt" } else { "le" },
                Into::<String>::into(value)
            ),
        }
    }
}

impl Into<String> for FilterOperator {
    fn into(self) -> String {
        self.str(false)
    }
}

impl ToString for FilterOperator {
    fn to_string(&self) -> String {
        Into::<String>::into(self.clone())
    }
}
