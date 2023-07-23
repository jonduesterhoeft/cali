#[derive(Debug)]
pub struct CalendarExistsError;

impl std::fmt::Display for CalendarExistsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Calendar with this name already exists.")
    }
}

impl std::error::Error for CalendarExistsError {}