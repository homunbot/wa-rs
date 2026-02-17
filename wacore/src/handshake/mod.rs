// Re-export everything from wa_rs_core-noise
pub use wa_rs_noise::{
    EdgeRoutingError, HandshakeError, HandshakeResult as Result, HandshakeState, HandshakeUtils,
    MAX_EDGE_ROUTING_LEN, NoiseCipher, NoiseHandshake, WA_CERT_PUB_KEY,
    build_edge_routing_preintro, build_handshake_header, generate_iv,
};
