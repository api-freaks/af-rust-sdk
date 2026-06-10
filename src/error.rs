use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("BadRequestError: Bad request - {{message}}")]
    BadRequestError {
        message: String,
        field: Option<String>,
        details: Option<String>,
    },
    #[error("UnauthorizedError: Authentication failed - {{message}}")]
    UnauthorizedError {
        message: String,
        auth_type: Option<String>,
    },
    #[error("PaymentRequiredError: {{message}}")]
    PaymentRequiredError { message: String },
    #[error("ForbiddenError: Access forbidden - {{message}}")]
    ForbiddenError {
        message: String,
        resource: Option<String>,
        required_permission: Option<String>,
    },
    #[error("NotFoundError: Resource not found - {{message}}")]
    NotFoundError {
        message: String,
        resource_id: Option<String>,
        resource_type: Option<String>,
    },
    #[error("NotAcceptableError: {{message}}")]
    NotAcceptableError { message: String },
    #[error("ContentTooLargeError: {{message}}")]
    ContentTooLargeError { message: String },
    #[error("LockedError: {{message}}")]
    LockedError { message: String },
    #[error("TooManyRequestsError: Rate limit exceeded - {{message}}")]
    TooManyRequestsError {
        message: String,
        retry_after_seconds: Option<u64>,
        limit_type: Option<String>,
    },
    #[error("InternalServerError: Internal server error - {{message}}")]
    InternalServerError {
        message: String,
        error_id: Option<String>,
    },
    #[error("ServiceUnavailableError: {{message}}")]
    ServiceUnavailableError { message: String },
    #[error("GatewayTimeoutError: {{message}}")]
    GatewayTimeoutError { message: String },
    #[error("RequestTimeoutError: {{message}}")]
    RequestTimeoutError { message: String },
    #[error("MethodNotAllowedError: {{message}}")]
    MethodNotAllowedError { message: String },
    #[error("UnsupportedMediaTypeError: {{message}}")]
    UnsupportedMediaTypeError { message: String },
    #[error("ProxyAuthenticationRequiredError: {{message}}")]
    ProxyAuthenticationRequiredError { message: String },
    #[error("NoResponseError: {{message}}")]
    NoResponseError { message: String },
    #[error("HTTP error {status}: {message}")]
    Http { status: u16, message: String },
    #[error("Network error: {0}")]
    Network(reqwest::Error),
    #[error("Serialization error: {0}")]
    Serialization(serde_json::Error),
    #[error("Configuration error: {0}")]
    Configuration(String),
    #[error("Invalid header value")]
    InvalidHeader,
    #[error("Could not clone request for retry")]
    RequestClone,
    #[error("SSE stream terminated")]
    StreamTerminated,
    #[error("SSE stream timed out waiting for next event")]
    StreamTimeout,
    #[error("SSE parse error: {0}")]
    SseParseError(String),
}

impl ApiError {
    pub fn from_response(status_code: u16, body: Option<&str>) -> Self {
        match status_code {
            400 => {
                // Parse error body for BadRequestError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::BadRequestError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            field: parsed
                                .get("field")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            details: parsed
                                .get("details")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::BadRequestError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    field: None,
                    details: None,
                };
            }
            401 => {
                // Parse error body for UnauthorizedError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::UnauthorizedError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            auth_type: parsed
                                .get("auth_type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::UnauthorizedError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    auth_type: None,
                };
            }
            402 => {
                // Parse error body for PaymentRequiredError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::PaymentRequiredError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                        };
                    }
                }
                return Self::PaymentRequiredError {
                    message: body.unwrap_or("Unknown error").to_string(),
                };
            }
            403 => {
                // Parse error body for ForbiddenError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::ForbiddenError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            resource: parsed
                                .get("resource")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            required_permission: parsed
                                .get("required_permission")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::ForbiddenError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    resource: None,
                    required_permission: None,
                };
            }
            404 => {
                // Parse error body for NotFoundError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::NotFoundError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            resource_id: parsed
                                .get("resource_id")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            resource_type: parsed
                                .get("resource_type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::NotFoundError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    resource_id: None,
                    resource_type: None,
                };
            }
            406 => {
                // Parse error body for NotAcceptableError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::NotAcceptableError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                        };
                    }
                }
                return Self::NotAcceptableError {
                    message: body.unwrap_or("Unknown error").to_string(),
                };
            }
            413 => {
                // Parse error body for ContentTooLargeError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::ContentTooLargeError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                        };
                    }
                }
                return Self::ContentTooLargeError {
                    message: body.unwrap_or("Unknown error").to_string(),
                };
            }
            423 => {
                // Parse error body for LockedError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::LockedError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                        };
                    }
                }
                return Self::LockedError {
                    message: body.unwrap_or("Unknown error").to_string(),
                };
            }
            429 => {
                // Parse error body for TooManyRequestsError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::TooManyRequestsError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            retry_after_seconds: parsed
                                .get("retry_after_seconds")
                                .and_then(|v| v.as_u64()),
                            limit_type: parsed
                                .get("limit_type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::TooManyRequestsError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    retry_after_seconds: None,
                    limit_type: None,
                };
            }
            500 => {
                // Parse error body for InternalServerError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::InternalServerError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            error_id: parsed
                                .get("error_id")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::InternalServerError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    error_id: None,
                };
            }
            503 => {
                // Parse error body for ServiceUnavailableError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::ServiceUnavailableError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                        };
                    }
                }
                return Self::ServiceUnavailableError {
                    message: body.unwrap_or("Unknown error").to_string(),
                };
            }
            504 => {
                // Parse error body for GatewayTimeoutError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::GatewayTimeoutError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                        };
                    }
                }
                return Self::GatewayTimeoutError {
                    message: body.unwrap_or("Unknown error").to_string(),
                };
            }
            408 => {
                // Parse error body for RequestTimeoutError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::RequestTimeoutError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                        };
                    }
                }
                return Self::RequestTimeoutError {
                    message: body.unwrap_or("Unknown error").to_string(),
                };
            }
            405 => {
                // Parse error body for MethodNotAllowedError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::MethodNotAllowedError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                        };
                    }
                }
                return Self::MethodNotAllowedError {
                    message: body.unwrap_or("Unknown error").to_string(),
                };
            }
            415 => {
                // Parse error body for UnsupportedMediaTypeError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::UnsupportedMediaTypeError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                        };
                    }
                }
                return Self::UnsupportedMediaTypeError {
                    message: body.unwrap_or("Unknown error").to_string(),
                };
            }
            407 => {
                // Parse error body for ProxyAuthenticationRequiredError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::ProxyAuthenticationRequiredError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                        };
                    }
                }
                return Self::ProxyAuthenticationRequiredError {
                    message: body.unwrap_or("Unknown error").to_string(),
                };
            }
            444 => {
                // Parse error body for NoResponseError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::NoResponseError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                        };
                    }
                }
                return Self::NoResponseError {
                    message: body.unwrap_or("Unknown error").to_string(),
                };
            }
            _ => Self::Http {
                status: status_code,
                message: body.unwrap_or("Unknown error").to_string(),
            },
        }
    }
}

/// Error returned when a required field was not set on a builder.
#[derive(Debug)]
pub struct BuildError {
    field: &'static str,
}

impl BuildError {
    pub fn missing_field(field: &'static str) -> Self {
        Self { field }
    }
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}` was not set but is required", self.field)
    }
}

impl std::error::Error for BuildError {}
