pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PdfRestrictRequestRestrictionsItem {
    PrintHigh,
    PrintLow,
    EditDocumentAssembly,
    FillFormFields,
    EditAnnotations,
    ModifyContent,
    CopyAndExtractContent,
    UseAccessibility,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PdfRestrictRequestRestrictionsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PrintHigh => serializer.serialize_str("print_high"),
            Self::PrintLow => serializer.serialize_str("print_low"),
            Self::EditDocumentAssembly => serializer.serialize_str("edit_document_assembly"),
            Self::FillFormFields => serializer.serialize_str("fill_form_fields"),
            Self::EditAnnotations => serializer.serialize_str("edit_annotations"),
            Self::ModifyContent => serializer.serialize_str("modify_content"),
            Self::CopyAndExtractContent => serializer.serialize_str("copy_and_extract_content"),
            Self::UseAccessibility => serializer.serialize_str("use_accessibility"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PdfRestrictRequestRestrictionsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "print_high" => Ok(Self::PrintHigh),
            "print_low" => Ok(Self::PrintLow),
            "edit_document_assembly" => Ok(Self::EditDocumentAssembly),
            "fill_form_fields" => Ok(Self::FillFormFields),
            "edit_annotations" => Ok(Self::EditAnnotations),
            "modify_content" => Ok(Self::ModifyContent),
            "copy_and_extract_content" => Ok(Self::CopyAndExtractContent),
            "use_accessibility" => Ok(Self::UseAccessibility),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PdfRestrictRequestRestrictionsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PrintHigh => write!(f, "print_high"),
            Self::PrintLow => write!(f, "print_low"),
            Self::EditDocumentAssembly => write!(f, "edit_document_assembly"),
            Self::FillFormFields => write!(f, "fill_form_fields"),
            Self::EditAnnotations => write!(f, "edit_annotations"),
            Self::ModifyContent => write!(f, "modify_content"),
            Self::CopyAndExtractContent => write!(f, "copy_and_extract_content"),
            Self::UseAccessibility => write!(f, "use_accessibility"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
