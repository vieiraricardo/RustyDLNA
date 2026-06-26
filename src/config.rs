// Server configuration constants

/// Server IP address on the local network
pub const IP_ADDRESS: &str = "192.168.15.3";

/// Root directory for serving media files
pub const DIR_PATH: &str = "./";

/// Number of threads in the connection pool
pub const NUM_THREADS: i32 = 256;

/// TCP port for HTTP serving
pub const HTTP_PORT: u16 = 8200;

/// UDP multicast port for SSDP
pub const SSDP_PORT: u16 = 1900;

/// SSDP multicast address
pub const SSDP_MULTICAST_ADDR: &str = "239.255.255.250";

/// Unique device UUID (generated once, kept constant)
pub const DEVICE_UUID: &str = "4d696e69-444c-164e-9d41-b827eb96c6c2";

/// Friendly device name (appears on TV)
pub const DEVICE_FRIENDLY_NAME: &str = "RustyDLNA6";

/// Server identifier
pub const SERVER_ID: &str = "RustyDLNA6/1.3.0";

/// Cache time in seconds for SSDP responses
pub const CACHE_MAX_AGE: u32 = 1800;

/// Interval between NOTIFY broadcasts (seconds)
pub const NOTIFY_INTERVAL: u64 = 20;
