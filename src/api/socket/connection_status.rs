///
/// Connection status
pub enum IsConnected<T, E> {
    Active(T),
    Closed(E),
}
