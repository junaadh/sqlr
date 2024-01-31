use crate::errors::Errors;

pub fn evaluate_meta(buffer: &str) {
    if buffer.eq_ignore_ascii_case(".exit") {
        Errors::handler(Errors::ExitSuccess, None);
    } else {
        Errors::handler(Errors::UnrecognizedMetaCommand, Some(buffer));
    }
}
