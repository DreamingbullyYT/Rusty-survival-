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

/// Structured diagnostic. The code is stable for tooling; message text is supplemental.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    code: DiagnosticCode,
    severity: Severity,
    subject: Option<String>,
    detail: Option<String>,
    message: String,
}

impl Diagnostic {
    pub fn new(code: DiagnosticCode, severity: Severity, message: impl Into<String>) -> Self {
        Self { code, severity, subject: None, detail: None, message: message.into() }
    }

    pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn code(&self) -> DiagnosticCode { self.code }
    pub fn severity(&self) -> Severity { self.severity }
    pub fn subject(&self) -> Option<&str> { self.subject.as_deref() }
    pub fn detail(&self) -> Option<&str> { self.detail.as_deref() }
    pub fn message_text(&self) -> &str { &self.message }
}
