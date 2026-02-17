use chrono::Local;
use log::{error, info};
use std::sync::Arc;
use wa_rs_core::proto_helpers::MessageExt;
use wa_rs_core::types::events::Event;
use wa_rs_proto::whatsapp as wa;
use wa_rs::bot::{Bot, MessageContext};
use wa_rs::store::SqliteStore;
use wa_rs_tokio_transport::TokioWebSocketTransportFactory;
use wa_rs_ureq_http::UreqHttpClient;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
        .format(|buf, record| {
            use std::io::Write;
            writeln!(
                buf,
                "{} [{:<5}] [{}] - {}",
                Local::now().format("%H:%M:%S"),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to build tokio runtime");

    rt.block_on(async {
        let backend = match SqliteStore::new("file::memory:?cache=shared").await {
            Ok(store) => Arc::new(store),
            Err(e) => {
                error!("Failed to create SQLite backend: {}", e);
                return;
            }
        };
        info!("SQLite backend initialized successfully.");

        let mut transport_factory = TokioWebSocketTransportFactory::new();
        if let Ok(ws_url) = std::env::var("WHATSAPP_WS_URL") {
            transport_factory = transport_factory.with_url(ws_url);
        }
        let http_client = UreqHttpClient::new();

        let builder = Bot::builder()
            .with_backend(backend)
            .with_transport_factory(transport_factory)
            .with_http_client(http_client);

        let mut bot = builder
            .on_event(move |event, client| async move {
                match event {
                    Event::Message(msg, info) => {
                        let ctx = MessageContext {
                            message: msg,
                            info,
                            client,
                        };

                        if let Some(text) = ctx.message.text_content()
                            && text == "ping"
                        {
                            info!("Received text ping, sending pong...");

                            let pong_text = format!("pong {}", ctx.info.id);

                            let reply_message = wa::Message {
                                conversation: Some(pong_text),
                                ..Default::default()
                            };

                            if let Err(e) = ctx.send_message(reply_message).await {
                                error!("Failed to send pong reply: {}", e);
                            }
                        }
                    }
                    Event::Connected(_) => {
                        info!("✅ Bot connected successfully!");
                    }
                    Event::LoggedOut(_) => {
                        error!("❌ Bot was logged out!");
                    }
                    _ => {}
                }
            })
            .build()
            .await
            .expect("Failed to build bot");

        let bot_handle = match bot.run().await {
            Ok(handle) => handle,
            Err(e) => {
                error!("Bot failed to start: {}", e);
                return;
            }
        };

        bot_handle
            .await
            .expect("Bot task should complete without panicking");
    });
}
