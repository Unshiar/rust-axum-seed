#[derive(Debug)]
pub enum Status {
    Success,
    Failure,
}

#[derive(Debug)]
pub struct StatusInfo {
    pub status: Status,
    pub info: String,
}
