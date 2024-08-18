pub struct ErrorMessages;

impl ErrorMessages {
    pub fn not_found(what: &str) -> String {
        format!("\"{}\" was not found", what)
    }
    
    pub(crate) fn failed_to_create(what: &str) -> String {
        format!("failed to create \"{}\"", what)
    }
}
