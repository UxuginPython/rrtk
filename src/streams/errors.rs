#[cfg(not(feature = "std"))]
use core::fmt;
#[cfg(feature = "std")]
use std::fmt;
#[derive(Clone)]
pub struct AnyStreamErrorData {
    message: Option<&'static str>,
    suberror: Option<Box<dyn AnyStreamError>>,
}
impl AnyStreamErrorData {
    pub fn new(message: Option<&'static str>, suberror: Option<Box<dyn AnyStreamError>>) -> Self {
        Self {
            message: message,
            suberror: suberror,
        }
    }
}
pub trait AnyStreamError: Clone {
    fn get_any_stream_error_data_ref(&self) -> &AnyStreamErrorData;
    fn get_message(&self) -> Option<&'static str> {
        let data = self.get_any_stream_error_data_ref();
        data.message
    }
    fn get_suberror(&self) -> &Option<Box<dyn AnyStreamError>> {
        let data = self.get_any_stream_error_data_ref();
        data.suberror.clone()
    }
}
impl fmt::Debug for dyn AnyStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "hello world")
    }
}
pub struct StreamError {
    any_stream_error_data: AnyStreamErrorData,
}
impl StreamError {
    pub fn new(message: Option<&'static str>, suberror: Option<Box<dyn AnyStreamError>>) -> Self {
        Self {
            any_stream_error_data: AnyStreamErrorData::new(message, suberror),
        }
    }
}
impl AnyStreamError for StreamError {
    fn get_any_stream_error_data_ref(&self) -> &AnyStreamErrorData {
        &self.any_stream_error_data
    }
}
