use crate::tui::form::FormField;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryType {
    Normal,
    Totp,
    SSH,
}
#[derive(Debug)]
pub struct LoginForm {
    pub title: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
    pub entry_type: EntryType,
    pub active: FormField,
}
pub enum EntryAction {
    NextField,
    PrevField,
}
