use once_cell::sync::Lazy;

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
/// Specifies how many concurrent jobs can be buffered in the worker channel.
pub const MAX_CONCURRENT_TASKS: usize = 64;

/// Specifies how deep various forms of computation will go before the query fails.
///
/// For reference, use ~15 per MiB of stack in release mode.
pub static MAX_COMPUTATION_DEPTH: Lazy<u8> = Lazy::new(|| {
	option_env!("SURREAL_MAX_COMPUTATION_DEPTH").and_then(|s| s.parse::<u8>().ok()).unwrap_or(120)
});

/// Specifies the names of parameters which can not be specified in a query.
pub const PROTECTED_PARAM_NAMES: &[&str] = &["auth", "scope", "token", "session"];

/// The characters which are supported in server record IDs.
pub const ID_CHARS: [char; 36] = [
	'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
	'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// The publicly visible name of the server
pub const SERVER_NAME: &str = "SurrealDB";
