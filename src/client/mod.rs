pub(crate) mod options;

use std::sync::Arc;
use crate::Option;

/// Internal Representation of a Client
pub struct Client{
    pub(crate) url: Arc<String>,
    pub(crate) database: Arc<String>,
    pub(crate) option: Option,
    pub(crate) _username: Arc<String>,
    pub(crate) _password: Arc<String>,
    pub(crate) _token: Arc<String>,
}

impl Default for Client {
    #[must_use = "Creating a client is pointless unless you use it"]
    fn default() -> Self{
        Client {
            url: Arc::new("http://localhost:8086".parse().unwrap()),
            database: Arc::new("".to_string()),
            _username: Arc::new("".to_string()),
            _password: Arc::new("".to_string()),
            _token: Arc::new("".to_string()),
            option: Option::default(),
        }
    }
}

impl Client {
    /// Instantiates a new [`Client`](crate::Client)
    ///
    /// # Arguments
    ///
    ///  * `url`: The URL where InfluxDB is running (ex. `http://localhost:8086`).
    ///  * `database`: The Database against which queries and writes will be run.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use influxdb::Client;
    ///
    /// let _client = Client::new("http://localhost:8086", "test");
    /// ```
    #[must_use = "Creating a client is pointless unless you use it"]
    pub fn new<S1, S2>(url: S1, database: S2) -> Self
        where
            S1: Into<String>,
            S2: Into<String>,
    {
        Client {
            url: Arc::new(url.into()),
            database: Arc::new(database.into()),
            option: Option::default(),
            _username: Arc::new("".to_string()),
            _password: Arc::new("".to_string()),
            _token: Arc::new("".to_string())
        }
    }
    pub fn set_database(mut self, database: &str) -> Self{
        self.database = Arc::new(database.parse().unwrap());
        self
    }

    /// Returns the name of the database the client is using
    pub fn database(&self) -> &str {
        // safe to unwrap: we always set the database name in `Self::new`
        self.database.as_str()
    }
    pub fn with_auth(mut self, username: &str, password: &str) -> Self {
        self._username = Arc::new(username.parse().unwrap());
        self._password = Arc::new(password.parse().unwrap());
        self
    }
    pub fn with_token(mut self, token: &str) -> Self{
        self._token = Arc::new(token.parse().unwrap());
        self
    }

}