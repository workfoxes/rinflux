mod client;
mod error;

pub use client::Client;
pub use client::options::Option;
pub use client::options::write::WriteOption;
pub use client::options::http::HttpOption;
pub use error::Error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
