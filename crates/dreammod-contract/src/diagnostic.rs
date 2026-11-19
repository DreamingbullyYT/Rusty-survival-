/// Stable machine-readable diagnostic categories.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DiagnosticCode {
    InvalidPackage,
    ApiIncompatible,
    CapabilityDenied,
    InvalidHandle,
    LimitExceeded,
    ModFault,
    HostFailure,
    Conflict,
    UnsupportedFeature,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Severity { Info, Warning, Error }

/// Small structured diagnostic. Text is supplemental; code is stable for tooling.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    code: DiagnosticCode,
    severity: Severity,
    subject: Option<String>,
    detail: Option<String>,
    message: String,
}

impl Diagnostic {
    pub fn new(code: DiagnosticCode, severity: Severity, message: impl Into<String>, detail: impl Into<String>) -> Self {
        Self { code, severity, subject: Some(message.into()), detail: Some(detail.into()), message: String::new() }
    }

    pub fn message(code: DiagnosticCode, severity: Severity, message: impl Into<String>) -> Self {
        Self { code, severity, subject: None, detail: None, message: message.into() }
    }

    pub fn code(&self) -> DiagnosticCode { self.code }
    pub fn severity(&self) -> Severity { self.severity }
    pub fn subject(&self) -> Option<&str> { self.subject.as_deref() }
    pub fn detail(&self) -> Option<&str> { self.detail.as_deref() }
    pub fn message_text(&self) -> &str { &self.message }
}
