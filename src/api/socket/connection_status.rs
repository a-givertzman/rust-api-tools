///
/// Connection status
pub enum IsConnected {
    Active(Vec<u8>),
    Closed,
}
