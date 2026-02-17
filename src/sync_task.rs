use wa_rs_core::appstate::patch_decode::WAPatchName;
use wa_rs_proto::whatsapp::message::HistorySyncNotification;

#[derive(Debug)]
pub enum MajorSyncTask {
    HistorySync {
        message_id: String,
        notification: Box<HistorySyncNotification>,
    },
    AppStateSync {
        name: WAPatchName,
        full_sync: bool,
    },
}
