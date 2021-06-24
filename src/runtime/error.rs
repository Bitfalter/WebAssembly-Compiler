#[derive(Debug, PartialEq, Eq)]
pub enum RuntimeError {
    ModuleToShort,
    WrongMagicHeader,
    WrongVersionHeader,
    InvalidSectionCode,
    InvalidValueType,
    InvalidExportType,
    InvalidExportName,
    InvalidInstruction,
    ExportNotFound,
    InvalidArgNumber,
}
