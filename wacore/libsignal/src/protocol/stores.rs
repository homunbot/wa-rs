// Re-exporting structures from wa_rs_proto to avoid duplication
pub use wa_rs_proto::whatsapp::{
    IdentityKeyPairStructure, PreKeyRecordStructure, RecordStructure, SenderKeyRecordStructure,
    SenderKeyStateStructure, SessionStructure, SignedPreKeyRecordStructure,
};

pub use wa_rs_proto::whatsapp::sender_key_state_structure;
pub use wa_rs_proto::whatsapp::session_structure;
