#[derive(Debug)]
pub enum ApplicationMessage {
    DatabaseOpen(String),
    DatabaseSave,
    DatabaseClose
}