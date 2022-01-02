use reqwest::{Client as HttpClient, Response as HttpResponse};

use http::StatusCode;
use crate::Error;

/// Options holds http configuration properties for communicating with InfluxDB server
/// HTTP client. Default is http.DefaultClient.
/// doer is an http Doer - if set it overrides httpClient
/// Flag whether http client was created internally
/// TLS configuration for secure connection. Default nil
/// HTTP request timeout in sec. Default 20
#[derive(Default)]
pub struct HttpOption {
    http_client: HttpClient,
    http_request_timeout: i32
}

impl HttpOption {
    pub fn default() -> Self {
        HttpOption{
            http_client: HttpClient::new(),
            http_request_timeout: 20
        }
    }
    pub(crate) fn check_status(res: &HttpResponse) -> Result<(), Error> {
        let status = res.status();
        if status == StatusCode::UNAUTHORIZED.as_u16() {
            Err(Error::AuthorizationError)
        } else if status == StatusCode::FORBIDDEN.as_u16() {
            Err(Error::AuthenticationError)
        } else {
            Ok(())
        }
    }
}
