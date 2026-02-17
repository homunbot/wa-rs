extern crate self as wa_rs_core;

pub use aes_gcm;
pub use wa_rs_appstate as appstate;
pub use wa_rs_noise as noise;

// Re-export derive macros
pub use wa_rs_derive::{EmptyNode, ProtocolNode, StringEnum};

pub mod client;
pub mod download;
pub mod iq;
pub mod protocol;
pub use wa_rs_noise::framing;
pub mod handshake;
pub mod history_sync;
pub mod ib;
pub use wa_rs_libsignal as libsignal;
pub mod messages;
pub mod net;
pub mod pair;
pub mod pair_code;
pub mod prekeys;
pub mod proto_helpers;
pub mod reporting_token;
pub mod request;
pub mod send;
pub mod stanza;
pub mod store;
pub mod types;
pub mod upload;
pub mod usync;
pub mod version;
pub mod xml;
