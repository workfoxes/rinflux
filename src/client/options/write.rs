use std::collections::HashMap;
use std::time::Duration;

/// - Maximum number of points sent to server in single request. Default 5000
/// 	* batch_size i32
/// - Interval, in ms, in which is buffer flushed if it has not been already written (by reaching batch size) . Default 1000ms
/// 	* flush_interval i32
/// - Precision to use in writes for timestamp. In unit of duration: time.Nanosecond, time.Microsecond, time.Millisecond, time.Second Default Nanosecond
/// 	* precision Duration
/// - Whether to use GZip compression in requests. Default false
/// 	* use_gzip bool
/// - Tags added to each point during writing. If a point already has a tag with the same key, it is left unchanged.
/// 	* default_tags HashMap
/// - Default retry interval in ms, if not sent by server. Default 5,000.
/// 	* retry_interval i32
/// - Maximum count of retry attempts of failed writes, default 5.
/// 	* max_retries i32
/// - Maximum number of points to keep for retry. Should be multiple of BatchSize. Default 50,000.
/// 	* retry_buffer_limit i32
/// - The maximum delay between each retry attempt in milliseconds, default 125,000.
/// 	* max_retry_interval i32
/// - The maximum total retry timeout in millisecond, default 180,000.
/// 	* max_retry_time i32
/// - The base for the exponential retry delay
/// 	* exponential_base i32
#[derive(Default)]
pub struct  WriteOption {
    batch_size: i32,
    flush_interval: i32,
    precision: Duration,
    use_gzip: bool,
    default_tags: HashMap<&'static str, String>,
    retry_interval: i32,
    max_retries: i32,
    retry_buffer_limit: i32,
    max_retry_interval: i32,
    max_retry_time: i32,
    exponential_base: i32
}

impl WriteOption {
    #[must_use = "Creating a WriteOption is pointless unless you use it"]
    pub fn default() -> Self {
        WriteOption{
            batch_size: 5000,
            flush_interval: 1000,
            precision: Duration::from_nanos(1),
            use_gzip: false,
            default_tags: HashMap::new(),
            retry_interval: 5000,
            max_retries: 5,
            retry_buffer_limit: 50000,
            max_retry_interval: 125000,
            max_retry_time: 180000,
            exponential_base: 2
        }
    }
}