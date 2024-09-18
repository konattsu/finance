#[allow(unused)]
pub trait DetailResponseError {
    fn detail_resp_err(self, err_type: &str) -> String;
}

impl<T, E> DetailResponseError for Result<T, E>
where
    T: ToString,
    E: ToString,
{
    fn detail_resp_err(self, err_type: &str) -> String {
        match self {
            Ok(v) => format!("Error {}: {}", err_type, v.to_string()),
            Err(e) => format!("Error reading response body: {}", e.to_string()),
        }
    }
}
