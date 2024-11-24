//! 
//! fn foo() -> Result<String, Box<dyn std::error::Error>> {
//!     Err(Box::new(StrErr("Error...")))
//! }
///
/// Error container
pub struct StrErr(pub String);
// Error doesn't require you to implement any methods, but
// your type must also implement Debug and Display.
impl std::error::Error for StrErr {}
//
//
impl std::fmt::Display for StrErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
//
//
impl std::fmt::Debug for StrErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<String> for StrErr {
    fn from(value: String) -> Self {
        StrErr(value)
    }
}
impl From<&str> for StrErr {
    fn from(value: &str) -> Self {
        StrErr(value.to_owned())
    }
}
impl Into<Box<StrErr>> for &str {
    fn into(self) -> Box<StrErr> {
        Box::new(StrErr(self.to_owned()))
    }
}